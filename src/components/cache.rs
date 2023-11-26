use std::collections::HashMap;
use async_trait::async_trait;
use crate::typing::Dict as normal_dict;
use itertools::Itertools;


const MINUTE: f32 = 60f32;
const HOUR: f32 = MINUTE * 60.;
const DAY: f32 = HOUR * 24.;
const WEEK: f32 = DAY * 7.;

type Dict = HashMap<i64, (i64, reqwest::Response)>;


// fn separate(values: Vec<?>, sep: Option<&str>) -> String {
//     let mut parts: Vec<&str> = vec![];
//
//     for value in &values {
//         if value.is_empty() {
//             parts.push("null");
//         } else if value.
//     }
//
//     return parts.iter().map(|str| str.to_string()).join(sep.unwrap_or(";"))
// }


// pub(crate) fn cache_key(key: &str, kwargs: normal_dict) -> CacheKey {
//     let mut name = if !key.is_empty() {
//
//     } else {
//
//     };
//
//     name.push_str("CacheKey");
//
//     dbg!(name);
//
//     CacheKey {}
// }


#[derive(Debug)]
pub(crate) struct CacheKey;

impl CacheKey {
    // pub(crate) fn

}


#[derive(Debug)]
pub(crate) struct Cache {
    pub(crate) cache: Dict,
    pub(crate) maxsize: usize,
    pub(crate) ttl: f32,
    pub(crate) static_ttl: f32,
}

impl Cache {
    pub(crate) fn new(maxsize: Option<usize>, ttl: Option<f32>, static_ttl: Option<f32>) -> Self {
        Self {
            cache: HashMap::new(),
            maxsize: maxsize.unwrap_or(1024),
            ttl: ttl.unwrap_or(HOUR),
            static_ttl: static_ttl.unwrap_or(DAY),
        }
    }

    pub(crate) fn static_new(ttl: Option<f32>) -> Self {
        Self {
            cache: Default::default(),
            maxsize: u32::MAX as usize,
            ttl: 0f32,
            static_ttl: ttl.unwrap_or(DAY),
        }
    }

    // pub(crate) fn clear_cache(&mut self) {
    //     let now = chrono::Utc::now().timestamp();
    //     let tmp = &self.cache;
    //
    //     for (key, val) in tmp {
    //         if val.0 < now {
    //             self.cache.remove(key);
    //         }
    //     }



        // if self.len() > self.maxsize {
        //     let overflow = self.len() - self.maxsize;
        //
        //     for key in &self.cache.keys().collect_vec()[..=overflow] {
        //         self.cache.remove(key);
        //     }
        // }
    // }

    pub(crate) fn len(&self) -> usize {
        self.cache.len()
    }

    // pub(crate) async fn get(&mut self, key: &str) -> Option<reqwest::Response> {
    //     self.clear_cache();
    //
    //
    //     self.cache.get(key)
    // }
    //
    // async fn set(&mut self, key: &str, value: reqwest::Response) {
    //     let now = chrono::Utc::now().timestamp();
    //     self.cache.insert(now, ())
    // }

    async fn get_static(&self, key: &str) -> Option<reqwest::Response> {
        todo!()
    }

    async fn set_static(&self, key: &str, value: reqwest::Response) {
        todo!()
    }

}
