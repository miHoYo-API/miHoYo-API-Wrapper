use anyhow::bail;

use crate::component::client::base::InnerClient;
use crate::component::client::chronicle::client::Chronicle;
#[cfg(feature = "genshin")]
use crate::component::client::chronicle::genshin::GenshinClient;
#[cfg(feature = "honkai")]
use crate::component::client::chronicle::honkai::HonkaiClient;
#[cfg(feature = "starrail")]
use crate::component::client::chronicle::starrail::StarRailClient;
use crate::component::manager::managers::BaseCookieManager;
#[allow(unused)]
use crate::model::{genshin, honkai, ModelBase, starrail};
use crate::model::genshin::stats::UserWithCharacters;
use crate::model::hoyolab::record::{Account, AccountList, RecordCard};
use crate::types::{AnyCookieOrHeader, CookieOrHeader, Game, StringDict};
use crate::util::kwargs::Kwargs;
use crate::types::Languages;

/// A Client which can
#[derive(Debug, Clone)]
pub struct Client<'a> {
    pub(crate) client: InnerClient<'a>,
    #[cfg(feature = "genshin")]
    pub(crate) genshin: Chronicle<GenshinClient>,
    #[cfg(feature = "honkai")]
    pub(crate) honkai: Chronicle<HonkaiClient>,
    #[cfg(feature = "starrail")]
    pub(crate) starrail: Chronicle<StarRailClient>
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Self {
            client: InnerClient::default(),
            #[cfg(feature = "genshin")]
            genshin: Chronicle::<GenshinClient>::new(),
            #[cfg(feature = "honkai")]
            honkai: Chronicle::<HonkaiClient>::new(),
            #[cfg(feature = "starrail")]
            starrail: Chronicle::<StarRailClient>::new(),
        }
    }
}

impl Client<'_> {
    /// To Connect with HTTP(S) so need setting Cookies
    pub fn set_cookies<T: ToString>(&mut self, ltuid: T, ltoken: T) -> anyhow::Result<Self> {
        let mut dict = StringDict::new();
        dict.insert(String::from("ltuid"), ltuid.to_string());
        dict.insert(String::from("ltoken"), ltoken.to_string());

        self.client.cookie_manager = Some(BaseCookieManager::from_cookies(
            Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
        ));

        #[cfg(feature = "genshin")]
        {
            self.genshin.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }

        #[cfg(feature = "honkai")]
        {
            self.honkai.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }

        #[cfg(feature = "starrail")]
        {
            self.starrail.0.0.cookie_manager = Some(BaseCookieManager::from_cookies(
                Some(AnyCookieOrHeader::CookieOrHeader(CookieOrHeader::Dict(dict.clone())))
            ));
        }

        Ok(self.clone())
    }

    /// setting cookies from .env file.
    pub fn set_from_env<'a>(&mut self) -> anyhow::Result<Self> {
        use std::env;

        if let Err(why) = dotenv::dotenv() {
            bail!("Unable find .env file: {}", why);
        };

        self.set_cookies(
            env::var("ltuid").unwrap(),
            env::var("ltoken").unwrap()
        )
    }

    /// Get the accounts that miHoYo game as listed in `Vec`
    /// - _Genshin_
    /// - _Honkai 3rd_
    /// - _StarRail_
    pub async fn get_game_accounts(&self, lang: Option<Languages>) -> anyhow::Result<Vec<Account>> {
        let result = self.client.request_hoyolab(
            "binding/api/getUserGameRolesByCookie",
            lang,
            None,
            "GET",
            None,
            Kwargs::new(),
        ).await.unwrap();
        let account_data = result.json::<ModelBase<AccountList>>().await.unwrap();
        Ok(account_data.data.list)
    }


    /// You can set the Game you want
    pub async fn get_game_account(&self, lang: Option<Languages>, game: Game) -> Option<Account> {
        let result = self.get_game_accounts(lang)
            .await
            .unwrap();
        result
            .into_iter()
            .filter(|account| account.which_game() == game)
            .next()
    }

    pub async fn get_record_cards(&self, hoyolab_id: Option<u32>, lang: Option<Languages>) -> anyhow::Result<Vec<RecordCard>> {
        let result = self.client.get_record_cards(hoyolab_id, lang)
            .await
            .unwrap();
        Ok(result)
    }


    #[cfg(feature = "genshin")]
    pub async fn get_genshin_notes(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::notes::GenshinNote> {
        let result = self.genshin.0.get_notes(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "genshin")]
    pub async fn get_genshin_partial_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::stats::PartialUser> {
        let result = self.genshin.0.get_partial_user(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "genshin")]
    pub async fn get_genshin_characters(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::character::Characters> {
        let result = self.genshin.0.get_characters(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "genshin")]
    pub async fn get_genshin_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<genshin::stats::UserWithCharacters> {
        let user = self.get_genshin_partial_user(uid.clone(), lang.clone())
            .await.unwrap();
        let characters = self.get_genshin_characters(uid, lang)
            .await.unwrap();
        Ok(UserWithCharacters::new(user, characters.characters))
    }

    #[deprecated = "It so annoying to write A model for Deserialize. Its killed me."]
    #[cfg(feature = "genshin")]
    pub async fn get_genshin_activities(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<()> {
        let _result = self.genshin.0.get_activities(uid, lang)
            .await
            .unwrap();
        Ok(())
    }

    #[cfg(feature = "genshin")]
    pub async fn get_genshin_spiral_abyss(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<Languages>) -> anyhow::Result<genshin::abyss::SpiralAbyss> {
        let result = self.genshin.0.get_spiral_abyss(uid, previous, lang)
            .await
            .unwrap();
        Ok(result)
    }



    #[deprecated = "the response data of send thats always {\"data\":null,\"message\":\"Data is not public for the user\",\"retcode\":10102}. and idk how to turn to public"]
    #[cfg(feature = "honkai")]
    pub async fn get_honkai_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<()> {
        let _result = self.honkai.0.get_user(uid, lang)
            .await
            .unwrap();
        Ok(())
    }



    #[cfg(feature = "starrail")]
    pub async fn get_starrail_notes(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<starrail::notes::StarRailNote> {
        let result = self.starrail.0.get_notes(uid, lang, None)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_user(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<starrail::stats::UserStats> {
        let result = self.starrail.0.get_user(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_characters(&self, uid: Option<u32>, lang: Option<Languages>) -> anyhow::Result<Vec<starrail::character::CharacterDetails>> {
        let result = self.starrail.0.get_characters(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_challenge(&self, uid: Option<u32>, previous: Option<bool>, lang: Option<Languages>) -> anyhow::Result<starrail::challenge::Challenge> {
        let result = self.starrail.0.get_challenge(uid, previous, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_rogue(&self, uid: Option<u32>, schedule_type: Option<i32>, lang: Option<Languages>) -> anyhow::Result<starrail::rogue::Rogue> {
        let result = self.starrail.0.get_rouge(uid, schedule_type, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_preview(&self, uid: u32, lang: Option<&str>) -> anyhow::Result<starrail::mihomo::Mihomo> {
        let result = self.starrail.0.get_preview(uid, lang)
            .await
            .unwrap();
        Ok(result)
    }

    #[cfg(feature = "starrail")]
    pub async fn get_starrail_preview_characters(&self, uid: u32, lang: Option<&str>) -> anyhow::Result<Vec<starrail::mihomo::Characters>> {
        let result = self.starrail.0.get_preview(uid, lang)
            .await
            .unwrap();
        Ok(result.characters)
    }
}

