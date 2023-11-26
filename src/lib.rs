pub mod typing;
pub mod client;

pub(crate) mod components;
pub(crate) mod error;


#[cfg(test)]
mod test {
    use crate::client::Client;
    use crate::typing::{Game, Languages};


    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        /// Initialize Client variable.
        let mut client = Client::new();

        /// Setting for two cookies connect [Hoyolab](https://www.hoyolab.com/home).
        /// And another way to set,  you can use [`Client::set_cookies`]
        client.set_from_env(None)?;

        /// Getting [`crate::components::models::hoyolab::record::Account`] as elements in Vectors.
        let accounts = client.get_game_account(Some(Game::StarRail), None).await?;

        /// Extract UID from account.
        let uid = accounts.get(0).unwrap().get_uid();

        /// Extract StarRail UID from [Hoyolab](https://www.hoyolab.com/home).
        /// getting user accounts as contains in Vector and then filtered by user level.
        let account_id = client.get_game_accounts(Some(Languages::JaJp)).await?
            .into_iter().filter(|account| account.level == 70).next().unwrap().get_uid();

        /// This [`crate::components::chronicle::starrail::StarRailClient::get_preview_data`] is only β.
        /// Getting as [`crate::components::models::starrail::mihomo::Mihomo`].
        /// --About lang argument, Here's [corresponding string list]()--
        /// I'm gonna create a enum of Language.
        let user_data = client.starrail.get_preview_data(account_id, Some("jp")).await.unwrap();
        dbg!(user_data);

        Ok(())
    }
}
