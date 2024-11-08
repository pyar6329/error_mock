use anyhow::{Error, Result};

use error_mock::{
    common::setup_tracing,
    infra::client::foobar,
    infra::grpc::{grpc_create_crowdfunding, grpc_get_crowdfunding},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing()?;
    let _ = grpc_create_crowdfunding();
    let _ = grpc_get_crowdfunding();

    let _ = foobar().await;

    Ok(())
}
