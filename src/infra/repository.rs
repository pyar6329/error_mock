use crate::usecase::{BusinessLogicError, CrowdfundingRepository, NewCrowdfunding, Crowdfunding};
use tracing::error;
use std::fmt::Debug;
use strum::EnumIs;
use thiserror::Error as ThisError;
use anyhow::{Result, Error};

#[derive(ThisError, Debug, Clone, Eq, PartialEq, EnumIs)]
pub enum CrowdfundingRepositoryError {
    #[error("not found error occurred: `{0}`")]
    NotFound(String),
    #[error("already exist error occurred: `{0}`")]
    AlreadyExist(String),
    #[error("invalid argument error occurred: `{0}`")]
    InvalidArgument(String),
    #[error("permission error occurred: `{0}`")]
    PermissionDenied(String),
    #[error("internal error occurred: `{0}`")]
    InternalError(String),
}

impl Default for CrowdfundingRepositoryError {
    fn default() -> Self {
        Self::InternalError("".to_string())
    }
}

impl BusinessLogicError for CrowdfundingRepositoryError
{
    fn not_found(resource: &impl Debug) -> Self {
        let e = Self::NotFound(format!("{:?}", resource));
        error!("{}", &e);
        e
    }
    fn already_exist(resource: &impl Debug) -> Self {
        let e = Self::AlreadyExist(format!("{:?}", resource));
        error!("{}", &e);
        e
    }
    fn permission_denied(resource: &impl Debug) -> Self {
        let e = Self::PermissionDenied(format!("{:?}", resource));
        error!("{}", &e);
        e
    }
    fn is_not_found(&self) -> bool {
        self.is_not_found()
    }
    fn is_already_exists(&self) -> bool {
        self.is_already_exist()
    }
    fn is_permission_denied(&self) -> bool {
        self.is_permission_denied()
    }
}

pub struct CrowdfundingRepositoryImpl;

impl CrowdfundingRepository for CrowdfundingRepositoryImpl
{
    type ResponseError = CrowdfundingRepositoryError;

    fn get_crowdfunding(&self, id: &u64) -> Result<Crowdfunding, Error> {
        Ok(Crowdfunding { id: *id, name: "foobar".to_string() })
    }

    fn create_crowdfunding(&self, crowdfunding: &NewCrowdfunding) -> Result<(), Error> {
        Ok(())
    }
}
