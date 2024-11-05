use anyhow::Error;
use std::fmt::Debug;

pub trait BusinessLogicError<T>: Into<Error>
where
    T: Debug,
{
    fn not_found(resource: &T) -> Self;
    fn already_exist(resource: &T) -> Self;
    fn permission_denied(resource: &T) -> Self;

    fn is_not_found(&self) -> bool;
    fn is_already_exists(&self) -> bool;
    fn is_permission_denied(&self) -> bool;
}
