use crate::infra::repository::{CrowdfundingRepositoryError, CrowdfundingRepositoryImpl};
use crate::usecase::{NewCrowdfunding, Crowdfunding, create_crowdfunding, get_crowdfunding};
use tracing::debug;
use anyhow::{Result, Error};

pub fn grpc_get_crowdfunding() -> Result<Crowdfunding, Error> {
  let id: u64 = 1;
  let repo = CrowdfundingRepositoryImpl;

  let result = get_crowdfunding(&repo, &id)?;

  Ok(result)
}

pub fn grpc_create_crowdfunding() -> Result<(), Error> {
  let name = "foobar".to_string();
  let new_crowdfunding = NewCrowdfunding {name};
  let repo = CrowdfundingRepositoryImpl;

  let _ = create_crowdfunding(&repo, &new_crowdfunding)?;

  Ok(())
}
