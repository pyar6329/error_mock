use anyhow::Error;
use std::fmt::Debug;

pub trait BusinessLogicError<T>: Into<Error>
where
    T: Debug,
{
    // TOOD:
    // 引数をTではなく、Into<String>とかにしてあげると、ToStringを実装すれば内部でinto()呼び出しにより共通化しやすい？
    // ただこれ外部で&strやStringにして渡す実装も出来てしまうので、もうちょっと制約をかけたい
    fn not_found(resource: &T) -> Self;
    fn already_exist(resource: &T) -> Self;
    fn permission_denied(resource: &T) -> Self;

    fn is_not_found(&self) -> bool;
    fn is_already_exists(&self) -> bool;
    fn is_permission_denied(&self) -> bool;
}
