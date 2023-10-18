use serde::Deserialize;

pub(crate) mod genshin;
pub(crate) mod honkai;
pub(crate) mod hoyolab;
pub(crate) mod starrail;


#[derive(Debug, Deserialize)]
pub(crate) struct ModelBase<T> {
    retcode: u32,
    message: String,
    pub(crate) data: T
}