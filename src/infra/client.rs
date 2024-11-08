use anyhow::{anyhow, Error, Result};
use http::{Extensions, StatusCode};
use reqwest::{Request, Response};
use reqwest_middleware::{Error as ReqwestMiddlewareError, Middleware, Next};
use std::future::Future;
use std::marker::Send;
use thiserror::Error as ThisError;

struct RetryMiddleware<F> {
    retry_condition: F,
}

impl<T, F, Fut, E> RetryMiddleware<F>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Into<Error>,
{
    fn new(retry_condition: F) -> Self {
        Self { retry_condition }
    }

    fn call_retry_condition(&self) -> Fut {
        (self.retry_condition)()
    }
}

pub type ReqwestResult<T> = std::result::Result<T, ReqwestMiddlewareError>;

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl<T, F, Fut, E> Middleware for RetryMiddleware<F>
where
    F: Fn() -> Fut + Sync + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Sync + Send + 'static,
    T: Sync + Send + 'static,
    E: Into<Error> + Sync + Send + 'static,
{
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> ReqwestResult<Response> {
        let mut retry_count = 0;
        let duplicate_request = req.try_clone().ok_or_else(|| {
            ReqwestMiddlewareError::Middleware(anyhow!(
                "Request object is not cloneable. Are you passing a streaming body?".to_string()
            ))
        })?;

        let result = next.clone().run(duplicate_request, extensions).await;

        let check_retry = match &result {
            Ok(success) => {
                let async_fn = move || check_retry_condition(&success);
                let retry_middleware = RetryMiddleware::new(async_fn);
                let foo = retry_middleware.call_retry_condition().await;
                None
                // default_on_request_success(&success)
            }
            Err(error) => default_on_request_failure(&error),
        };

        // ここでretry + waiting処理をする
        if let Some(Retryable::Transient) = check_retry {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            retry_count += 1;
        }

        // retry_countが3回を超えた場合はretry上限が行った感じ
        if retry_count > 3 {
            result.map_err(|err| {
                ReqwestMiddlewareError::Middleware(
                    RetryError::WithRetries {
                        retries: retry_count,
                        err,
                    }
                    .into(),
                )
            })
        } else {
            result.map_err(|err| ReqwestMiddlewareError::Middleware(RetryError::Error(err).into()))
        }

        // let bbb = response.get_ref();
        // let bbb = self.call_retry_condition(result).await;

        // result
    }
}

#[derive(Debug, ThisError)]
pub enum RetryError {
    #[error("Request failed after {retries} retries")]
    WithRetries {
        retries: u32,
        #[source]
        err: reqwest_middleware::Error,
    },
    #[error(transparent)]
    Error(reqwest_middleware::Error),
}

#[derive(PartialEq, Eq)]
pub enum Retryable {
    /// The failure was due to something that might resolve in the future.
    Transient,
    /// Unresolvable error.
    Fatal,
    /// No error.
    NoError,
}

pub fn default_on_request_success(success: &reqwest::Response) -> Option<Retryable> {
    let status = success.status();
    if status.is_server_error() {
        Some(Retryable::Transient)
    } else if status.is_client_error()
        && status != StatusCode::REQUEST_TIMEOUT
        && status != StatusCode::TOO_MANY_REQUESTS
    {
        Some(Retryable::Fatal)
    } else if status.is_success() {
        None
    } else if status == StatusCode::REQUEST_TIMEOUT || status == StatusCode::TOO_MANY_REQUESTS {
        Some(Retryable::Transient)
    } else {
        Some(Retryable::Fatal)
    }
}

pub fn default_on_request_failure(error: &ReqwestMiddlewareError) -> Option<Retryable> {
    match error {
        // If something fails in the middleware we're screwed.
        ReqwestMiddlewareError::Middleware(_) => Some(Retryable::Fatal),
        ReqwestMiddlewareError::Reqwest(error) => {
            #[cfg(not(target_arch = "wasm32"))]
            let is_connect = error.is_connect();
            #[cfg(target_arch = "wasm32")]
            let is_connect = false;
            if error.is_timeout() || is_connect {
                Some(Retryable::Transient)
            } else if error.is_body()
                || error.is_decode()
                || error.is_builder()
                || error.is_redirect()
            {
                Some(Retryable::Fatal)
            } else if error.is_request() {
                // It seems that hyper::Error(IncompleteMessage) is not correctly handled by reqwest.
                // Here we check if the Reqwest error was originated by hyper and map it consistently.
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(hyper_error) = get_source_error_type::<hyper::Error>(&error) {
                    // The hyper::Error(IncompleteMessage) is raised if the HTTP response is well formatted but does not contain all the bytes.
                    // This can happen when the server has started sending back the response but the connection is cut halfway through.
                    // We can safely retry the call, hence marking this error as [`Retryable::Transient`].
                    // Instead hyper::Error(Canceled) is raised when the connection is
                    // gracefully closed on the server side.
                    if hyper_error.is_incomplete_message() || hyper_error.is_canceled() {
                        Some(Retryable::Transient)

                    // Try and downcast the hyper error to io::Error if that is the
                    // underlying error, and try and classify it.
                    } else if let Some(io_error) =
                        get_source_error_type::<std::io::Error>(hyper_error)
                    {
                        Some(classify_io_error(io_error))
                    } else {
                        Some(Retryable::Fatal)
                    }
                } else {
                    Some(Retryable::Fatal)
                }
                #[cfg(target_arch = "wasm32")]
                Some(Retryable::Fatal)
            } else {
                // We omit checking if error.is_status() since we check that already.
                // However, if Response::error_for_status is used the status will still
                // remain in the response object.
                None
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn classify_io_error(error: &std::io::Error) -> Retryable {
    match error.kind() {
        std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted => {
            Retryable::Transient
        }
        _ => Retryable::Fatal,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_source_error_type<T: std::error::Error + 'static>(
    err: &dyn std::error::Error,
) -> Option<&T> {
    let mut source = err.source();

    while let Some(err) = source {
        if let Some(err) = err.downcast_ref::<T>() {
            return Some(err);
        }

        source = err.source();
    }
    None
}

#[derive(Debug, ThisError)]
enum ErrorType {
    #[error("Response error")]
    ResponseError(ResponseError),
    #[error("other error")]
    OtherError(Error),
}

#[derive(Debug)]
struct ResponseError {
    pub error: ResponseErrorMessage,
}

#[derive(Debug)]
struct ResponseErrorMessage {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
struct ResponseBody {
    id: u64,
    name: String,
}

// async fn check_retry_condition(user_id: &str) -> Result<ResponseBody, ErrorType> {
//     let response = ResponseBody {
//         id: 10,
//         name: "John Doe".to_string(),
//     };
//
//     let error_response = ErrorType::ResponseError(ResponseError {
//         error: ResponseErrorMessage {
//             code: 412,
//             message: "Precondition Failed".to_string(),
//         },
//     });
//
//     if user_id.contains("hogehoge") {
//         Ok(response)
//     } else {
//         Err(error_response)
//     }
// }

async fn check_retry_condition(success: &reqwest::Response) -> Result<Retryable, Error> {
    let status = success.status();
    if status.is_server_error() {
        Ok(Retryable::Transient)
    } else if status.is_client_error()
        && status != StatusCode::REQUEST_TIMEOUT
        && status != StatusCode::TOO_MANY_REQUESTS
    {
        Ok(Retryable::Fatal)
    } else if status.is_success() {
        Ok(Retryable::NoError)
    } else if status == StatusCode::REQUEST_TIMEOUT || status == StatusCode::TOO_MANY_REQUESTS {
        Ok(Retryable::Transient)
    } else {
        Ok(Retryable::Fatal)
    }
}

pub async fn foobar() {
    // let async_fn = move || check_retry_condition("foo");
    // let retry_middleware = RetryMiddleware::new(async_fn);

    println!("hello");
}
