//! Types

use std::any::Any;
use std::collections::HashMap;

pub(crate) type GeneralResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub(crate) type StringDict = HashMap<String, String>;
pub(crate) type GeneralAny = Box<dyn Any + Send + Sync>;


#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CookieOrHeader {
    Dict(StringDict),
    Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AnyCookieOrHeader {
    CookieOrHeader(CookieOrHeader),
    SequenceCookieOrHeader(Vec<CookieOrHeader>)
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Region {
    OVERSEAS,
    CHINESE,
}
impl Region {
    pub fn name(&self) -> &str {
        match self {
            Region::OVERSEAS => "os",
            Region::CHINESE => "cn",
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Game {
    GENSHIN,
    HONKAI,
    STARRAIL,
}
impl Game {
    pub fn name(&self) -> &str {
        match self {
            Game::GENSHIN => "genshin",
            Game::HONKAI => "honkai3rd",
            Game::STARRAIL => "hkrpg",
        }
    }
}