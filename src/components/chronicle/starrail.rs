use anyhow::Result;
use reqwest::Response;
use crate::components::models::starrail;
use crate::components::base::InnerClient;
use crate::typing::{Dict, Game, Languages};

use super::super::utils::uid::{recognize_starrail_server, recognize_region};


pub struct StarRailClient(pub(crate) InnerClient);

impl Default for StarRailClient {
    fn default() -> Self {
        Self(InnerClient::default())
    }
}

impl StarRailClient {
    async fn inner_get_record(
        &self, endpoint: &str, uid: u32, method: Option<&str>, lang: Option<Languages>, payload: Option<Dict>, _cache: Option<bool>
    ) -> Result<Response> {
        let mut kwargs= payload.unwrap_or_else(|| Dict::new());
        kwargs.insert("role_id".to_string(), uid.to_string());
        kwargs.insert("server".to_string(), recognize_starrail_server(&uid)?);

        let data = self.0.request_game_record(
            endpoint,
            method,
            lang,
            recognize_region(&mut uid.clone(), Game::StarRail),
            Some(Game::StarRail),
            Some(kwargs)
        )
            .await?;
        Ok(data)
    }

    pub async fn get_notes(&self, uid: u32, lang: Option<Languages>) -> Result<()> {
        let result = self.inner_get_record(
            "note",
            uid,
            None,
            lang,
            None,
            None,
        )
            .await?;

        dbg!(result.text().await?);

        Ok(())
    }

    /// This functions is only β.
    /// Ref: https://github.com/Mar-7th

    #[cfg(feature = "mihomo")]
    pub async fn get_preview_data(&self, uid: u32, lang: Option<&str>) -> Result<starrail::mihomo::Mihomo> {
        let url = format!("https://api.mihomo.me/sr_info_parsed/{}?lang={}",
                              uid, lang.unwrap_or("en"));

        let client = reqwest::Client::builder()
            .build()?.request("GET".parse()?, url);

        // dbg!(self.0.request(client).await.unwrap().text().await.unwrap());

        let data = self.0.request(client).await?
            .json::<starrail::mihomo::Mihomo>()
            .await?;

        Ok(data)
    }


}