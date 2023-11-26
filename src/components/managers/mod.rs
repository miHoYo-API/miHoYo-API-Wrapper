use crate::typing::Dict;

pub(crate) mod cookies;
pub(crate) mod manager;


pub(crate) enum CookieOrHeader {
    Dict(Dict),
    Str(String),
}

#[cfg(feature = "working_on")]
pub(crate) enum AnyCookieOrHeader {
    Normal(CookieOrHeader),
    #[cfg(feature = "working_on")]
    Sequence(Vec<CookieOrHeader>)
}