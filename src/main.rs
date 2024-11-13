use anyhow::{Error, Result};

use error_mock::{
    common::setup_tracing,
    // infra::client::foobar,
    infra::grpc::{grpc_create_crowdfunding, grpc_get_crowdfunding},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    status: String,
    zen: String,
}

#[derive(Debug, Deserialize)]
struct Response2 {
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // setup_tracing()?;
    // let _ = grpc_create_crowdfunding();
    // let _ = grpc_get_crowdfunding();
    //
    // let _ = foobar().await;

    let response = reqwest::get("https://api.dsfootball.dev/v2/healthz").await;

    let body = response?.text().await?;
    let parsed_body: Response = serde_json::from_str(&body)?;
    let parsed_body2: Response2 = serde_json::from_str(&body)?;
    println!("content: {:?}", &parsed_body);
    println!("content: {:?}", &parsed_body2);

    Ok(())
}
