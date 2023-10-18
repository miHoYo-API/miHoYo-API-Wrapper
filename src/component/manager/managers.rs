use crate::types::{
    AnyCookieOrHeader,
    CookieOrHeader,
};


//  I will WRITE someday
// pub(crate) fn parse_cookie<'a>(cookie: Option<CookieOrHeader>) -> NaturalDict<'a> {
//     let mut cookies = NaturalDict::new();
//
//     if cookie.is_none() {
//         return cookies;
//     }
//
//     if let CookieOrHeader::Str(cookie) =  cookie.as_ref().unwrap() {
//         cookies = _parse_cookie(cookie);
//     }
//
//     for (k,v) in cookies {
//
//     }
//
//     cookies
// }

// fn _parse_cookie(cookie: &str) -> NaturalDict {
//     let mut dict = NaturalDict::new();
//     let material = cookie.split(",").collect::<Vec<String>>();
//
//     for i in material {
//         if i.contains("=") {
//             let x = i.split("=").collect();
//             dict.insert(x.0,x.1);
//         }
//     }
//
//     dict
// }


#[derive(Debug)]
pub(crate) struct BaseCookieManager {
    cookies: Option<CookieOrHeader>
}

impl BaseCookieManager {
    pub(crate) fn new(cookie: Option<CookieOrHeader>) -> BaseCookieManager {
        BaseCookieManager { cookies: cookie }
    }

    pub(crate) fn from_cookies(cookies: Option<AnyCookieOrHeader>) -> BaseCookieManager {
        if cookies.is_none() {
            return BaseCookieManager { cookies: None };
        }

        match cookies.unwrap() {
            AnyCookieOrHeader::CookieOrHeader(any_cookie) => {
                return match any_cookie {
                    CookieOrHeader::Dict(dict) => {
                        BaseCookieManager::new(Some(CookieOrHeader::Dict(dict)))
                    }
                }
            }
            _ => {
            }
        }
        BaseCookieManager::new(None)
    }

    pub(crate) fn forming_cookie(&self) -> (String, String) {
        let header = &self.cookies.as_ref().unwrap();

        match header {
            CookieOrHeader::Dict(cookie) => {
                let ltuid = cookie.get("ltuid").unwrap();
                let ltoken = cookie.get("ltoken").unwrap();
                return (format!("ltuid={}", ltuid), format!("ltoken={}", ltoken))
            }
            _ => (String::new(), String::new())
        }
    }

}

