use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::md5;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::components::utils::constant::DS_SALT;
use crate::typing::{Region, Languages};

pub(crate) mod constant;
pub(crate) mod route;
pub(crate) mod uid;


fn generate_dynamic_secret(salt: Option<&str>) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let salt = salt.unwrap_or(DS_SALT.get(&Region::OverSeas).unwrap());

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

pub(crate) fn gen_ds_header(region: &Region, lang: Option<Languages>) -> HeaderMap {
    let mut map = HeaderMap::new();
    match region {
        Region::OverSeas => {
            map.insert("x-rpc-app_version", HeaderValue::from_static("1.5.0"));
            map.insert("x-rpc-client_type", HeaderValue::from_static("5"));
            map.insert("x-rpc-language", HeaderValue::from_str(lang.unwrap_or(Languages::EnUs).name().as_str()).unwrap());
            map.insert("ds", HeaderValue::from_str(generate_dynamic_secret(None).as_str()).unwrap());
        }
        Region::Chinese => {
            map.insert("x-rpc-app_version", HeaderValue::from_static("2.11.1"));
            map.insert("x-rpc-client_type", HeaderValue::from_static("5"));
        }
    };

    map
}