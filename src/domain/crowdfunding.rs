use anyhow::Error;
// use std::fmt::Display;
use std::fmt::Debug;
use derive_more::Display as DeriveDisplay;
use super::BusinessLogicError;

#[derive(Debug, DeriveDisplay)]
#[display("nothing")]
pub struct Crowdfunding {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, DeriveDisplay)]
#[display("nothing")]
pub struct NewCrowdfunding {
    pub name: String,
}

pub trait CrowdfundingRepository<T>
where
    T: Debug,
{
    type ResponseError: Into<Error> + BusinessLogicError<T>;
    fn get_crowdfunding(&self, id: &u64) -> Result<Crowdfunding, Error>;
    fn create_crowdfunding(&self, crowdfunding: &NewCrowdfunding) -> Result<(), Error>;
}
