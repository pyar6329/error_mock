use anyhow::Error;
// use std::fmt::Display;
use std::fmt::Debug;
use derive_more::Display as DeriveDisplay;
use derive_more::Debug as DeriveDebug;
use super::BusinessLogicError;

#[derive(Debug, DeriveDisplay)]
#[display("nothing")]
pub struct Crowdfunding {
    pub id: u64,
    pub name: String,
}

#[derive(DeriveDebug, DeriveDisplay)]
#[display("nothing")]
pub struct NewCrowdfunding {
    pub name: String,
    #[debug(skip)]
    pub description: String,
    pub user_id: u64,
}

pub trait CrowdfundingRepository
{
    type ResponseError: Into<Error> + BusinessLogicError;
    fn get_crowdfunding(&self, id: &u64) -> Result<Crowdfunding, Error>;
    fn create_crowdfunding(&self, crowdfunding: &NewCrowdfunding) -> Result<(), Error>;
}
