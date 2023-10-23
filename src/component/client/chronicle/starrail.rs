use reqwest::header::HeaderMap;
use reqwest::Response;

use crate::component::client::base::InnerClient;
use crate::component::client::chronicle::client::Chronicle;
use crate::model::ModelBase;
use crate::model::starrail;
use crate::types::{Game, Languages};
use crate::util::kwargs::Kwargs;
use crate::util::uid::{recognize_region, recognize_starrail_server};

#[derive(Debug, Clone)]
pub(crate) struct StarRailClient(pub(crate) InnerClient<'static>);


impl StarRailClient {
    async fn inner_get_starrail_record<'a>(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<Languages>, payload: Option<Kwargs<'static>>, _cache: Option<bool>
    ) -> anyhow::Result<Response> {
        let mut payload = payload.unwrap_or_else(|| Kwargs::new());
        payload.set("role_id", uid);
        payload.set("server", recognize_starrail_server(&uid).unwrap());

        let mut kwargs = Kwargs::new();

        let method =  method.unwrap_or("GET");
        if method.eq("GET") {
            kwargs.set("params", payload);
        } else {
            kwargs.set("data", payload);
        };

        let data = self.0.request_game_record(
            endpoint,
            method,
            lang,
            recognize_region(&mut uid.clone(), Game::STARRAIL),
            Some(Game::STARRAIL),
            Some(kwargs)
        )
        .await
        .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_notes(&self, uid: Option<u32>, lang: Option<Languages>, _auto_auth: Option<bool>) -> anyhow::Result<starrail::notes::StarRailNote> {
        let result = self.inner_get_starrail_record("note", uid.unwrap(), Some("GET"), lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<starrail::notes::StarRailNote>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<starrail::stats::UserStats> {
        let index_data = self.inner_get_starrail_record("index", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();
        let basic_info = self.inner_get_starrail_record("role/basicInfo", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();
        let partial_user = index_data.json::<ModelBase<starrail::stats::PartialUserStats>>()
            .await
            .unwrap();
        let little_info = basic_info.json::<ModelBase<starrail::stats::UserLittleInfo>>()
            .await
            .unwrap();
        Ok(starrail::stats::UserStats::new(partial_user.data, little_info.data))
    }

    pub(crate) async fn get_characters(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<Vec<starrail::character::CharacterDetails>>{
        let result = self.inner_get_starrail_record("avatar/info", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<starrail::character::Characters>>()
            .await
            .unwrap();
        Ok(result.data.list)
    }

    pub(crate) async fn get_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<Languages>) -> anyhow::Result<starrail::challenge::Challenge> {
        let mut payload = Kwargs::new();
        payload.set("schedule_type", if previous.is_some() { 2 } else { 1 });
        payload.set("need_all", "true");

        let result = self.inner_get_starrail_record("challenge", uid.unwrap(), None, lang, Some(payload), None)
            .await
            .unwrap()
            .json::<ModelBase<starrail::challenge::Challenge>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<i32>, lang: Option<Languages>) -> anyhow::Result<starrail::rogue::Rogue> {
        let mut payload = Kwargs::new();
        payload.set("schedule_type", schedule_type.unwrap_or(3));
        payload.set("need_detail", "true");
        let result = self.inner_get_starrail_record("rogue", uid.unwrap(), None, lang, Some(payload), None)
            .await
            .unwrap()
            .json::<ModelBase<starrail::rogue::Rogue>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    #[inline]
    pub(crate) async fn get_preview(&self, uid: u32, lang: Option<&str>) -> anyhow::Result<starrail::mihomo::Mihomo> {
        let url = {
            let lang = lang.unwrap_or_else(|| self.0.lang.as_ref());
            format!("https://api.mihomo.me/sr_info_parsed/{}?lang={}", uid, lang)
        };
        let result = self.0.request(url.as_str(), "GET", HeaderMap::new(), Kwargs::new())
            .await
            .unwrap()
            .json::<starrail::mihomo::Mihomo>()
            .await
            .unwrap();
        Ok(result)
    }
}


impl Chronicle<StarRailClient> {
    pub(crate) fn new() -> Self {
        Chronicle(StarRailClient(InnerClient::default()))
    }
}
