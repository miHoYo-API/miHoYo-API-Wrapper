use itertools::Itertools;
use crate::typing::Dict;
use super::CookieOrHeader;


#[derive(Debug)]
pub(crate) enum CookieType {
    Normal(CookieManager),

    #[cfg(feature = "working_on")]
    Sequence(CookieSequence),
}


pub(crate) fn auto_cookie(cookies: Dict) -> CookieType {
    #[cfg(feature = "working_on")]
    if cookies.len() > 2 {
        // CookieType::Sequence(CookieSequence::new(Some()))
    };
    CookieType::Normal(CookieManager::new(Some(CookieOrHeader::Dict(cookies))))
}


fn parse_cookie(cookie: Option<CookieOrHeader>) -> Dict {
    let mut dict = Dict::new();

    if let Some(cookies) = cookie {
        match cookies {
            CookieOrHeader::Dict(val) => dict = val,
            CookieOrHeader::Str(val) => dict = __parse_cookies(val),
        }
    }
    dict
}

// Forgive me what wrote this shit. i was just exhausted
// Someone correct me anytime, anywhere(e.g. Github),
pub(crate) fn __parse_cookies(val: String) -> Dict {
    let cut = |material: String, keyword: &str| -> Dict {
        let mut dict = Dict::new();

        for i in material.split(keyword) {
            let map = i.split("=")
                    .collect_vec()
                    .chunks_exact(2)
                    .map(|b| (b[0].trim().to_string(), b[1].trim().to_string()))
                    .collect::<Dict>();
            dict.extend(map);
        }
        dict
    };

    if val.contains(";") {
        cut(val, ";")
    } else if val.contains(",") {
        cut(val, ",")
    } else {
        Dict::new()
    }
}

pub(crate) fn get_cookie_identifier(cookie: Dict) -> Option<String> {
    for (name, value) in cookie.into_iter() {
        if vec!["ltuid", "account_id", "ltuid_v2", "account_id_v2"]
            .contains(&name.as_str()) {
            return Some(value);
        }
    }
    None
}



pub(crate) trait BaseCookieManager: Sized {
    fn from_cookies(cookies: Option<CookieOrHeader>) -> Self;

}


#[derive(Debug)]
pub(crate) struct CookieManager {
    cookies: Dict,
}
impl CookieManager {
    pub(crate) fn new(cookies: Option<CookieOrHeader>) -> CookieManager {
        CookieManager { cookies: parse_cookie(cookies) }
    }

    pub(crate) fn forming_cookie(&self) -> Vec<String> {
        self.cookies
            .iter()
            .take(2)
            .into_iter()
            .map(|(a, b)| {
                format!("{}={}" ,a.to_string(), b.to_string())
            })
            .collect_vec()
    }

    pub(crate) fn is_exist(&self) -> bool {
        self.cookies.is_empty()
    }
}

impl BaseCookieManager for CookieManager {
    fn from_cookies(cookies: Option<CookieOrHeader>) -> Self {
        if cookies.is_none() {
            return CookieManager::new(None);
        };

        match cookies.unwrap() {
            CookieOrHeader::Dict(cookie) => CookieManager::new(Some(CookieOrHeader::Dict(cookie))),
            CookieOrHeader::Str(cookie) => CookieManager::new(Some(CookieOrHeader::Str(cookie)))
        }
    }

}

impl Clone for CookieManager {
    fn clone(&self) -> Self {
        CookieManager { cookies: self.cookies.clone() }
    }
}


#[non_exhaustive]
#[cfg(feature = "working_on")]
pub(crate) struct CookieSequence {
    // cookies: HashMap<String, (HashMap<String, String>, i32)>,
    cookies: Vec<CookieOrHeader>,
    max_uses: u8,
}
#[cfg(feature = "working_on")]
impl CookieSequence {
    fn new(cookies: Vec<CookieOrHeader>) -> CookieSequence {
        CookieSequence { cookies, max_uses: 30 }
    }
}

#[cfg(feature = "working_on")]
impl BaseCookieManager for CookieSequence {
    fn from_cookies(cookies: Option<CookieOrHeader>) -> Self {
        todo!()
    }
}

#[cfg(feature = "working_on")]
pub(crate) struct RotatingCookieManager {
    cookies: Option<Vec<Dict>>
}
// #[cfg(feature = "working_on")]
// impl RotatingCookieManager {
//     pub(crate) fn new(cookies: Option<Vec<CookieOrHeader>>) {
//
//     }
// }

