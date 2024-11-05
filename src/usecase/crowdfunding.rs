pub use crate::domain::{NewCrowdfunding, Crowdfunding, BusinessLogicError, CrowdfundingRepository};
use anyhow::Error;
use tracing::debug;

pub fn get_crowdfunding<E>(
    repo: &impl CrowdfundingRepository<ResponseError = E>,
    // repo: &impl CrowdfundingRepository<NewCrowdfunding, ResponseError = E>
    // TODO:
    // これでも通ってしまう。引数をTにすると何でも渡せてしまうので、BusinessLogicErrorはGenericsを使わないようにした方が良さげ
    id: &u64,
) -> Result<Crowdfunding, Error>
where
    E: BusinessLogicError + Into<Error>
{
    let _result = repo.get_crowdfunding(id);
    let err = E::not_found(id).into(); // ここでエラーを呼び出す
                                                 //
    debug!("error in usecase: {:?}", &err);
    Err(err)
}

pub fn create_crowdfunding<E>(
    repo: &impl CrowdfundingRepository<ResponseError = E>,
    crowdfunding: &NewCrowdfunding,
) -> Result<(), Error>
where
    E: BusinessLogicError + Into<Error>
{
    let _result = repo.create_crowdfunding(crowdfunding);
    let err = E::already_exist(crowdfunding).into(); // ここでエラーを呼び出す
                                                 //
    debug!("error in usecase: {:?}", &err);
    Err(err)
}
