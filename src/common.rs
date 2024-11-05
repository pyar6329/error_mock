use anyhow::{Result, Error};
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

pub fn setup_tracing() -> Result<(), Error> {
    let log_filter = EnvFilter::from_default_env() // We can use: error!(), warn!(), info!(), debug!()
            .add_directive("error_mock=debug".parse()?);


    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(log_filter)
        .init();

    Ok(())
}
