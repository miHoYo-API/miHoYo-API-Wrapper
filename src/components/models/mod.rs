pub(crate) mod hoyolab;
pub(crate) mod starrail;


use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct Base<T> {
    pub(crate) retcode: i32,
    pub(crate) message: String,
    pub(crate) data: T
}
