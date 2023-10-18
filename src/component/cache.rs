use std::collections::HashMap;
use async_trait::async_trait;
use crate::types::GeneralAny;

type CacheDict = HashMap<GeneralAny, (f64, GeneralAny)>;


const MINUTE: u64 = 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;


#[async_trait]
pub(crate) trait BaseCache { async fn get(&self, key: GeneralAny) -> Option<GeneralAny>;
    async fn set(&self, key: GeneralAny, value: GeneralAny);
    async fn get_static(&self, key: GeneralAny) -> Option<GeneralAny>;
    async fn set_static(&self, key: GeneralAny, value: GeneralAny);
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Cache {
    cache: CacheDict,
    maxsize: i32,
    ttl: u64,
    static_ttl: u64
}

impl Cache {
    pub(crate) fn new(max: Option<i32>, ttl: Option<u64>, static_ttl: Option<u64>) -> Self {
        Self {
            cache: HashMap::default(),
            maxsize: max.unwrap_or(1024),
            ttl: ttl.unwrap_or(HOUR),
            static_ttl: static_ttl.unwrap_or(DAY),
        }
    }
}


#[async_trait]
impl BaseCache for Cache {
    async fn get(&self, key: GeneralAny) -> Option<GeneralAny> {
        todo!()
    }
    async fn set(&self, key: GeneralAny, value: GeneralAny) {
        todo!()
    }
    async fn get_static(&self, key: GeneralAny) -> Option<GeneralAny> {
        todo!()
    }
    async fn set_static(&self, key: GeneralAny, value: GeneralAny) {
        todo!()
    }
}
