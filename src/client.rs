use std::collections::HashMap;
use itertools::Itertools;
use crate::components::base::InnerClient;
use crate::components::chronicle::starrail::StarRailClient;
use crate::components::models::Base;
use crate::components::models::hoyolab::record::{Account, AccountList};
use crate::typing::{Dict, Game, Languages};



pub struct Client {
    pub(crate) client: InnerClient,
    pub starrail: StarRailClient,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            client: InnerClient::default(),
            #[cfg(feature = "starrail")]
            starrail: StarRailClient::default(),
        }
    }
}

impl AsMut<Client> for Client {
    fn as_mut(&mut self) -> &mut Client {
        self
    }
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: InnerClient::default(),
            #[cfg(feature = "starrail")]
            starrail: StarRailClient::default(),
        }
    }


    /// HELP: Someone tell me how to use as method chain without as_mut func
    pub fn set_cookies(mut self,  cookies: CookieType) -> anyhow::Result<Self> {
        use crate::components::managers::manager::{__parse_cookies, auto_cookie};

        let cookies = match cookies {
            CookieType::Str(cookies) => __parse_cookies(String::from(cookies)),
            CookieType::Dict(cookies) => {
                let mut dict = Dict::new();
                for (key, value) in cookies.into_iter() {
                    dict.insert(key.to_string(), value.to_string());
                }
                dict
            }
        };

        self.client.cookie_manager = auto_cookie(cookies.clone());

        #[cfg(feature = "genshin")] {

        }

        #[cfg(feature = "starrail")] {
            self.starrail.0.cookie_manager = auto_cookie(cookies.clone());
        }

        #[cfg(feature = "honkai")] {

        }

        Ok(self)
    }

    pub fn set_from_env(mut self, path: Option<&str>) -> anyhow::Result<Self> {
        use std::env::var;

        match path {
            None => dotenv::dotenv()?,
            Some(path) => dotenv::from_filename(path)?
        };

        let ltuid = var("ltuid").unwrap_or_else(|_| var("ltuid_v2").unwrap());
        let ltoken = var("ltoken").unwrap_or_else(|_| var("ltoken_v2").unwrap());
        let name = if ltoken.contains("v2") {
            (String::from("ltuid_v2"), String::from("ltoken_v2"))
        } else {
            (String::from("ltuid"), String::from("ltoken"))
        };

        let dict = HashMap::from([
            (name.0, ltuid),
            (name.1, ltoken),
        ]);

        self.set_cookies(CookieType::Dict(dict))
    }

    // Vec<Account>
    pub async fn get_game_accounts(&self, lang: Option<Languages>) -> anyhow::Result<Vec<Account>> {
        let result = self.client.request_hoyolab("binding/api/getUserGameRolesByCookie",
             lang,
         None,
         None,
         None,
         None
        ).await?;

        match result.json::<Base<AccountList>>().await {
            Ok(val) => Ok(val.data.list),
            Err(why) => {
                panic!("{}", why)
            }
        }
    }

    pub async fn get_game_account(&self, game: Option<Game>, lang: Option<Languages>) -> anyhow::Result<Vec<Account>> {
        let game = game.unwrap_or_else(|| self.client.game.clone());
        let accounts = self.get_game_accounts(lang).await?
            .into_iter()
            .filter(|account| {
                account.which_game().eq(&game)
            })
            .collect_vec();
        Ok(accounts)
    }
}

pub enum CookieType {
    Str(&'static str),
    Dict(HashMap<String, String>),
}