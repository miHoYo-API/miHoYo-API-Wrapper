use std::collections::HashMap;
use reqwest::{header::HeaderMap};
use reqwest::cookie::CookieStore;
use crate::components::utils::constant::{RECORD_URL, TAKUMI_URL, WEB_STATIC_URL};
use crate::components::managers::{CookieOrHeader, manager::CookieType};
use crate::components::managers::manager::CookieManager;
use crate::components::models::Base;
use crate::components::models::hoyolab::record::{RecordCard, RecordCardList};
use crate::components::utils::gen_ds_header;
use crate::typing::{Dict, Game, Languages, Region};



pub(crate) struct InnerClient {
    pub(crate) cookie_manager: CookieType,
    pub(crate) cache: super::cache::Cache,
    pub(crate) lang: Languages,
    pub(crate) region: Region,
    pub(crate) game: Game,
    pub(crate) hoyolab_id: Option<u32>,
    pub(crate) auth_key: Option<String>,
    pub(crate) proxy: Option<String>,
}

impl Default for InnerClient {
    fn default() -> Self {
        let cookies = Some(CookieOrHeader::Str(String::new()));
        let cache = super::cache::Cache::static_new(None);

        Self {
            cookie_manager: CookieType::Normal(CookieManager::new(cookies)),
            cache,
            lang: Languages::EnUs,
            region: Region::OverSeas,
            game: Game::Genshin,
            hoyolab_id: None,
            auth_key: None,
            proxy: None,
        }
    }
}

impl InnerClient {
    // pub(crate) fn new(
    //     lang: Option<Languages>,
    //     region: Option<Region>,
    //     game: Option<Game>,
    //     hoyolab_id: Option<u32>,
    //     auth_key: Option<String>,
    //     proxy: Option<String>
    // ) -> InnerClient {
    //     let cookies = Some(CookieOrHeader::Str(String::new()));
    //     let cookie_manager = CookieType::Normal(CookieManager::new(cookies));
    //     let cache = super::cache::Cache::static_new(None);
    //
    //     Self {
    //         cookie_manager,
    //         cache,
    //         lang: lang.unwrap_or(Languages::EnUs),
    //         region: region.unwrap_or(Region::OverSeas),
    //         game: game.unwrap_or(Game::Genshin),
    //         hoyolab_id,
    //         auth_key,
    //         proxy,
    //     }
    // }

    async fn to_params(&self, kwargs: Dict) -> Vec<(String, String)> {
        let mut vec = vec![];

        for (key, value) in kwargs {
            vec.push((key, value));
        }
        vec
    }

    async fn to_json(&self, kwargs: Dict) -> serde_json::Value {
        let mut map = HashMap::new();
        for (val1, val2) in self.to_params(kwargs).await {
            map.insert(val1, val2);
        }
        serde_json::json!(map)
    }

    pub(crate) async fn request_with_cookies(
        &self, url: &str, method: &str, mut headers: HeaderMap, kwargs: Dict
    ) -> anyhow::Result<reqwest::Response> {
        let jar = reqwest::cookie::Jar::default();
        let cookies = match &self.cookie_manager {
            CookieType::Normal(cookie) => cookie.forming_cookie(),
            #[cfg(feature = "working_on")]
            CookieType::Sequence(_cookie) => {}
        };
        for cookie in cookies.iter() {
            jar.add_cookie_str(cookie.as_str(), &url.parse()?);
        };
        headers.insert(reqwest::header::COOKIE, jar.cookies(&url.parse()?).unwrap());

        let client = reqwest::Client::builder()
            .user_agent(super::utils::constant::USER_AGENT)
            .cookie_provider(std::sync::Arc::new(jar))
            .cookie_store(true)
            .default_headers(headers.clone())
            .build()?;

        let mut data = client.request(method.parse()?, url);
        data = if method.eq("GET") {
            data.query(&self.to_params(kwargs).await)
        } else {
            data.json(&self.to_json(kwargs).await)
        };

        self.request(data).await
    }

    pub(crate) async fn request(&self, data: reqwest::RequestBuilder) -> anyhow::Result<reqwest::Response> {
        Ok(data.send().await?)
    }

    pub(crate) async fn request_web_static(
        &self, url: Option<&str>, headers: Option<&mut HeaderMap>, region: Option<&Region>, kwargs: Dict
    ) -> anyhow::Result<()> {
        let url = WEB_STATIC_URL.get_url(region.unwrap_or(&Region::OverSeas))?;


        Ok(())
    }

    pub(crate) async fn request_hoyolab(
        &self,
        url: &str,
        lang: Option<Languages>,
        region: Option<Region>,
        method: Option<&str>,
        headers: Option<HeaderMap>,
        kwargs: Option<Dict>,
    ) -> anyhow::Result<reqwest::Response> {
        let region = region.unwrap_or(self.region.clone());
        let url = if url.starts_with("https://") {
            url.to_string()
        } else {
            format!("{}{}", TAKUMI_URL.get_url(&region)?, url)
        };

        let mut headers = headers.unwrap_or_else(|| HeaderMap::new());
        headers.extend(gen_ds_header(&region, lang));

        let data = self.request_with_cookies(
            url.as_str(),
            method.unwrap_or("GET"),
            headers,
            kwargs.unwrap_or(Dict::new())
        )
            .await?;

        Ok(data)
    }

    pub(crate) async fn request_game_record(
        &self, endpoint: &str, method: Option<&str>, lang: Option<Languages>, region: Option<Region>, game: Option<Game>, kwargs: Option<Dict>
    ) -> anyhow::Result<reqwest::Response> {
        let url = {
            let mut base = RECORD_URL.get_url(&region.unwrap_or(Region::OverSeas))?.to_string();
            if let Some(game) = game {
                base = format!("{}{}/api/", base, game.which_title());
            };
            format!("{}{}", base, endpoint)
        };
        let kwargs = kwargs.unwrap_or_else(|| Dict::new());
        let data = self.request_hoyolab(url.as_str(), lang, region, method, None, Some(kwargs))
            .await?;

        Ok(data)
    }

    pub(crate) async fn get_record_cards(&self, hoyolab_id: Option<u32>, lang: Option<Languages>) -> anyhow::Result<Vec<RecordCard>> {
        let hoyolab_id = hoyolab_id.unwrap_or_else(|| self.hoyolab_id.clone().unwrap());
        let mut kwargs = Dict::new();
        kwargs.insert("uid".to_string(), hoyolab_id.to_string());

        let result = self.request_game_record(
            "card/wapi/getGameRecordCard",
            None,
            lang,
            None,
            None,
            Some(kwargs),
        )
            .await?;

        let result = result.json::<Base<RecordCardList>>()
            .await?;

        Ok(result.data.list)
    }
}