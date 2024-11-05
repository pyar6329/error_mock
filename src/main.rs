use anyhow::{Result, Error};
use tracing::debug;

use error_mock::{
  common::setup_tracing,
  infra::grpc::{
    grpc_create_crowdfunding, grpc_get_crowdfunding
  },
};


fn main() -> Result<(), Error> {
    setup_tracing()?;
    grpc_create_crowdfunding();
    let _ = grpc_get_crowdfunding();

    Ok(())
}

