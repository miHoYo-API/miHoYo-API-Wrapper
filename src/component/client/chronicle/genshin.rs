use reqwest::Response;

use crate::component::client::base::InnerClient;
use crate::component::client::chronicle::client::Chronicle;
use crate::model::genshin;
use crate::model::ModelBase;
use crate::types::{Game, Languages};
use crate::util::kwargs::Kwargs;
use crate::util::uid::{recognize_genshin_server, recognize_region};

#[derive(Debug, Clone)]
pub(crate) struct GenshinClient(pub(crate) InnerClient<'static>);


impl GenshinClient {
    async fn inner_get_genshin_record(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<Languages>, payload: Option<Kwargs<'static>>, _cache: Option<bool>
    ) -> anyhow::Result<Response> {
        let mut payload = payload.unwrap_or_else(|| Kwargs::new());
        payload.set("role_id", uid);
        payload.set("server", recognize_genshin_server(&uid).unwrap());

        let mut kwargs = Kwargs::new();
        let method = method.unwrap_or("GET");

        if method.eq("GET") {
            kwargs.set("params", payload);
        } else {
            kwargs.set("data", payload);
        };

        let data = self.0.request_game_record(
            endpoint,
            method,
            lang,
            recognize_region(&mut uid.clone(), Game::GENSHIN),
            Some(Game::GENSHIN),
            Some(kwargs)
        )
            .await
            .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_notes(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::notes::GenshinNote> {
        let result = self.inner_get_genshin_record("dailyNote", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<genshin::notes::GenshinNote>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_partial_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::stats::PartialUser> {
        let result = self.inner_get_genshin_record("index", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<genshin::stats::PartialUser>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_characters(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::character::Characters> {
        let result = self.inner_get_genshin_record("character", uid.unwrap(), Some("POST"), lang, None, None)
            .await
            .unwrap()
            .json::<ModelBase<genshin::character::Characters>>()
            .await
            .unwrap();
        Ok(result.data)
    }

    pub(crate) async fn get_activities(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<()> {
        let result = self.inner_get_genshin_record("activities", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();

        dbg!(result.text()
            .await
            .unwrap());

        Ok(())
    }

    pub(crate) async fn get_spiral_abyss(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<Languages>) -> anyhow::Result<genshin::abyss::SpiralAbyss> {
        let mut kwargs = Kwargs::new();
        let previous = if previous.is_some() { 2 } else { 1 };
        kwargs.set("schedule_type", previous);

        let result = self.inner_get_genshin_record("spiralAbyss", uid.unwrap(), None, lang, Some(kwargs), None)
            .await.unwrap()
            .json::<ModelBase<genshin::abyss::SpiralAbyss>>()
            .await
            .unwrap();

        Ok(result.data)
    }

    pub(crate) async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<&str>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }
}


impl Chronicle<GenshinClient> {
    pub(crate) fn new() -> Self {
        Chronicle(GenshinClient(InnerClient::default()))
    }

}
