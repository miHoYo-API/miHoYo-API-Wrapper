use std::any::Any;
use std::collections::HashMap;
use std::time::SystemTime;

use crypto::digest::Digest;
use crypto::md5;
use rand::{self, Rng};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::types;
use crate::util::constants::DS_SALT;

#[derive(Debug)]
pub struct Kwargs<'a> {
    values: HashMap<&'a str, Box<dyn Any>>,
}

impl<'a> Kwargs<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, k: &'a str, v: T)
        where T: Any {
        self.values.insert(k, Box::new(v));
    }

    pub fn get<T>(&self, key: &'a str) -> Option<&T>
        where T: Any {
        self.values.get(&key).and_then(|v| v.downcast_ref::<T>())
    }

    pub fn get_pair<T>(&self, key: &'a str) -> Option<(String, &T)>
        where T: Any
    {
        for (k, v) in &self.values {
            if k.eq(&key) {
                return Some((k.to_string(), v.downcast_ref::<T>().unwrap()));
            }
        }
        None
    }
}



pub(crate) fn generate_dynamic_secret(salt: Option<&str>) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let salt = salt.unwrap_or(DS_SALT.get(&types::Region::OVERSEAS).unwrap());

    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let r = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            char::from(unsafe { *CHARS.get_unchecked(idx) }).to_string()
        })
        .collect::<String>();
    let mut h = md5::Md5::new();
    h.input(format!("salt={}&t={}&r={}", salt, t, r).as_bytes());

    format!("{},{},{}", t, r, h.result_str())
}

// pub(crate) fn generate_cn_dynamic_secret(body: ) -> &str {
//
//     format!("").as_str()
// }


pub(crate) fn get_ds_headers(
    region: &types::Region,
    // data:
    // params: ,
    lang: Option<&str>,

) -> HeaderMap {
    let mut map = HeaderMap::new();
    match region {
        types::Region::OVERSEAS => {
            map.insert("x-rpc-app_version", HeaderValue::from_static("1.5.0"));
            map.insert("x-rpc-client_type", HeaderValue::from_static("5"));
            map.insert("x-rpc-language", HeaderValue::from_str(lang.unwrap_or("None")).unwrap());
            map.insert("ds", HeaderValue::from_str(generate_dynamic_secret(None).as_str()).unwrap());
            map
        }
        types::Region::CHINESE => {
            map.insert("x-rpc-app_version", HeaderValue::from_static("2.11.1"));
            map.insert("x-rpc-client_type", HeaderValue::from_static("5"));
            map
        }
    }
}