use reqwest::Response;
use crate::component::client::base::InnerClient;
use crate::component::client::chronicle::client::Chronicle;
use crate::types;
use crate::types::Game;
use crate::util::kwargs::Kwargs;
use crate::util::uid::{recognize_genshin_server, recognize_honkai_server, recognize_region};



#[derive(Debug)]
pub(crate) struct HonkaiClient(pub(crate) InnerClient<'static>);


impl HonkaiClient {
    async fn inner_get_honkai_record(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<&str>, payload: Option<Kwargs<'static>>, _cache: Option<bool>
    ) -> anyhow::Result<Response> {
        let mut payload = payload.unwrap_or_else(|| Kwargs::new());
        payload.set("role_id", uid);
        payload.set("server", recognize_honkai_server(&uid).unwrap());

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
            Some(types::Region::OVERSEAS),
            Some(Game::HONKAI),
            Some(kwargs)
        )
            .await
            .unwrap();
        Ok(data)
    }

    pub(crate) async fn get_notes(&self, uid: Option<u32>, lang: Option<&str>, auto_auth: Option<bool>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_user(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<()> {
        let result = self.inner_get_honkai_record("index", uid.unwrap(), None, lang, None, None)
            .await
            .unwrap();

        dbg!(result.text()
            .await
            .unwrap());

        Ok(())
    }

    pub(crate) async fn get_characters(&self, uid: Option<u32>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }

    pub(crate) async fn get_rouge(&self, uid: Option<u32>, schedule_type: Option<&str>, lang: Option<&str>) -> anyhow::Result<()> {
        todo!()
    }
}

// FUCKFUCKFUCKFUCK

impl Chronicle<HonkaiClient> {
    pub(crate) fn new() -> Self {
        Chronicle(HonkaiClient(InnerClient::default()))
    }

}

