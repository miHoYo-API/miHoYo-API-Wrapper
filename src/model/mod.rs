//! There' are models for Deserialize.

use serde::Deserialize;

pub mod genshin;
pub mod honkai;
pub mod hoyolab;
pub mod starrail;


#[derive(Debug, Deserialize)]
pub(crate) struct ModelBase<T> {
    #[serde(rename = "retcode")]
    _retcode: u32,
    #[serde(rename = "message")]
    _message: String,
    pub(crate) data: T
}