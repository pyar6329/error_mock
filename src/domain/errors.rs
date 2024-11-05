use anyhow::Error;
use std::fmt::Debug;
// use std::fmt::Display;

pub trait BusinessLogicError: Into<Error>
{
    // TOOD:
    // 引数をTではなく、Into<String>とかにしてあげると、ToStringを実装すれば内部でinto()呼び出しにより共通化しやすい？
    // ただこれ外部で&strやStringにして渡す実装も出来てしまうので、もうちょっと制約をかけたい
    fn not_found(resource: &impl Debug) -> Self;
    // こうでもいいかもしれない
    // fn fmtの手動実装時に特定の値をマスク出来るっぽい
    // ref: https://loige.co/how-to-to-string-in-rust/
    // fn not_found(resource: &impl Display) -> Self;
    fn already_exist(resource: &impl Debug) -> Self;
    fn permission_denied(resource: &impl Debug) -> Self;

    fn is_not_found(&self) -> bool;
    fn is_already_exists(&self) -> bool;
    fn is_permission_denied(&self) -> bool;
}
