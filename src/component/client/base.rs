use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{bail, Result};
use reqwest::{Response, Url};
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{COOKIE, HeaderMap};
use serde_json::Value;

use crate::component::cache::Cache;
use crate::component::manager::managers::BaseCookieManager;
use crate::component::routes::InternationalTrait;
use crate::model::hoyolab::record::{RecordCard, RecordCardList};
use crate::model::ModelBase;
use crate::types::{AnyCookieOrHeader, Game, Region, Languages};
use crate::util::constants::*;
use crate::util::kwargs::get_ds_headers;
use crate::util::kwargs::Kwargs;

type Uid = HashMap<Game, u32>;
//   ^ ?

#[derive(Debug, Clone)]
pub(crate) struct InnerClient<'a> {
    pub(crate) cookie_manager: Option<BaseCookieManager>,
    pub(crate) auth_key: Option<&'a str>,
    pub(crate) lang: &'a str,
    pub(crate) region: Region,
    pub(crate) proxy: Option<&'a str>,
    pub(crate) game: Option<Game>,
    pub(crate) uid: Option<Uid>,
    pub(crate) hoyolab_id: Option<u32>,
    // pub(crate) cache: Option<Cache>,
    pub(crate) debug: bool,
}


impl<'a> Default for InnerClient<'a> {
    fn default() -> Self {
        InnerClient {
            cookie_manager: None,
            auth_key: None,
            lang: "en-us",
            region: Region::OVERSEAS,
            proxy: None,
            game: None,
            uid: None,
            hoyolab_id: None,
            // cache: None,
            debug: true,
        }
    }
}


impl<'a> InnerClient<'a> {
    pub(crate) fn new(cookies: Option<AnyCookieOrHeader>, auth_key: Option<&'a str>, lang: &'a str, region: Region, proxy: Option<&'a str>, game: Option<Game>, uid: Option<Uid>, hoyolab_id: Option<u32>, _cache: Option<Cache>, debug: bool) -> InnerClient<'a> {
        let cookie_manager = Some(BaseCookieManager::from_cookies(cookies));
        InnerClient {
            cookie_manager,
            auth_key,
            lang,
            region,
            proxy,
            game,
            uid,
            hoyolab_id,
            // cache,
            debug,
        }
    }

    pub(crate) fn get_cookies(&self) -> Option<&BaseCookieManager> {
        self.cookie_manager.as_ref()
    }

    pub(crate) fn get_hoyolab_id(&self) -> Result<u32> {
        if let Some(hoyolab_id) = self.hoyolab_id.clone() {
            return Ok(hoyolab_id);
        }
        bail!("")
    }

    fn get_region(&self) -> Result<Region> {
        Ok(self.region.clone())
    }

    fn get_uid(&self, game: &Game) -> Result<u32> {
        if let Some(uid) = &self.uid {
            return Ok(uid.get(game).unwrap().clone());
        }
        bail!("")
    }

    fn forming_params(&self, kwargs: Kwargs) -> Vec<(String, String)> {
        let mut base = vec![];

        if let Some(params) = kwargs.get_pair::<Kwargs>("params") {
            if let Some(pair) = params.1.get_pair::<u32>("uid") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<u32>("role_id") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<String>("server") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<i32>("schedule_type") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = params.1.get_pair::<&str>("need_all") {
                base.push((pair.0, pair.1.to_string()));
            }
        };
        if let Some(data) = kwargs.get_pair::<Kwargs>("data") {
            if let Some(pair) = data.1.get_pair::<u32>("role_id") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = data.1.get_pair::<String>("server") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = data.1.get_pair::<i64>("switch_id") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = data.1.get_pair::<bool>("is_public") {
                base.push((pair.0, pair.1.to_string()));
            }
            if let Some(pair) = data.1.get_pair::<i32>("game_id") {
                base.push((pair.0, pair.1.to_string()));
            }
        }
        base
    }

    fn to_json(&self, kwargs: Kwargs) -> Value {
        let mut new = HashMap::new();

        for (val1, val2) in self.forming_params(kwargs) {
            new.insert(val1, val2);
        }

        serde_json::json!(&new)
    }

    pub(crate) async fn request(
        &self,
        url: &str,
        method: &str,
        mut headers: HeaderMap,
        kwargs: Kwargs,
    ) -> Result<Response> {
        let jar = Jar::default();
        let cookies = self.get_cookies().unwrap();
        let (ltuid, ltoken) = cookies.forming_cookie();
        jar.add_cookie_str(ltuid.as_str(), &url.parse::<Url>().unwrap());
        jar.add_cookie_str(ltoken.as_str(), &url.parse::<Url>().unwrap());
        headers.insert(COOKIE, jar.cookies(&url.parse::<Url>().unwrap()).unwrap());

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_provider(Arc::new(jar))
            .cookie_store(true)
            .default_headers(headers.clone())
            .build()
            .unwrap();

        let mut data = client.request(method.parse().unwrap(), url);

        if method.eq("GET") {
            data = data.query(&self.forming_params(kwargs));
        } else {
            data = data.json(&self.to_json(kwargs));
        }
        let re = data.send().await.unwrap();

        Ok(re)
    }

    pub(crate) async fn request_hoyolab(
        &self,
        url: &str,
        lang: Option<Languages>,
        region: Option<Region>,
        method: &str,
        headers: Option<HeaderMap>,
        kwargs: Kwargs,
    ) -> Result<Response> {
        // ensure!(lang.is_none(),"lang were None");
        // let lang = lang.unwrap_or(self.lang.clone());
        let region = region.unwrap_or(self.get_region().unwrap());
        let url = if url.starts_with("https://") {
            url.to_string()
        } else {
            format!("{}{}", TAKUMI_URL.get_url(region).unwrap(), url)
        };

        let mut new_headers = headers.unwrap_or_else(|| HeaderMap::new());
        new_headers.extend(get_ds_headers(&region, lang));

        let data = self.request(
            url.as_str(),
            method,
            new_headers,
            kwargs,
        )
            .await
            .unwrap();
        Ok(data)
    }


    pub(crate) async fn request_game_record(&self, endpoint: &str, method: &str, lang: Option<Languages>, region: Option<Region>, game: Option<Game>, kwargs: Option<Kwargs>) -> Result<Response> {
        let base_url = {
            let mut url = RECORD_URL.get_url(region.unwrap_or(Region::OVERSEAS)).unwrap().to_string();
            if let Some(game) = game {
                url = format!("{}{}/api/", url, game.name().to_lowercase());
            };
            url
        };
        let url = format!("{}{}", base_url, endpoint);
        let kwargs = kwargs.unwrap_or_else(|| Kwargs::new());
        let data = self.request_hoyolab(url.as_str(), lang, region, method, None, kwargs)
            .await
            .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_record_cards(&self, hoyolab_id: Option<u32>, lang: Option<Languages>) -> Result<Vec<RecordCard>> {
        let hoyolab_id = hoyolab_id.unwrap_or_else(|| self.get_hoyolab_id().unwrap());
        // let cache_key = cache

        let mut kwargs = Kwargs::new();
        let mut inner = Kwargs::new();

        inner.set("uid", hoyolab_id);
        kwargs.set("params", inner);

        let result = self.request_game_record(
            "card/wapi/getGameRecordCard",
            "GET",
            lang,
            None,
            None,
            Some(kwargs),
        )
            .await
            .unwrap()
            .json::<ModelBase<RecordCardList>>()
            .await
            .unwrap();
        Ok(result.data.list)
    }

    pub(crate) async fn update_settings(&self, settings: crate::types::IDOr, on: bool, game: Option<Game>) -> Result<()> {
        let mut game_title: Option<Game> = None;
        if let Some(title) = game {
            if let Some(default_game) = self.game {
                if title == Game::STARRAIL || default_game == Game::STARRAIL {
                    panic!("Star Rail does not provide a Battle Chronicle or Real-Time Notes.");
                }
            }
        } else {
            if settings.to_int() == 3 {
                game_title = Some(Game::GENSHIN);
            };
            if game_title.is_none() {
                game_title = self.game.clone();
            };
        };

        let game_id = match game_title {
            None => 0,
            Some(value) => {
                match value {
                    Game::GENSHIN => 2,
                    Game::HONKAI => 1,
                    Game::STARRAIL => 6,
                }
            }
        };

        let mut kwargs = Kwargs::new();
        let mut inner = Kwargs::new();
        inner.set("switch_id", settings.to_int());
        inner.set("is_public", on);
        inner.set("game_id", game_id);
        kwargs.set("data", inner);

        dbg!(&kwargs.get::<Kwargs>("data").unwrap());

        self.request_game_record(
            "card/wapi/changeDataSwitch",
            "POST",
            None,
            None,
            None,
            Some(kwargs)
        ).await.unwrap();
        Ok(())
    }
}
