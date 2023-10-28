//! miHoYo-API Wrapper is a API wrapper literally.
//!
//! ### Here's Original
//!
//! [genshin.py](https://github.com/thesadru/genshin.py) by [thesadru](https://github.com/thesadru)
//! With Grateful Respect.
//!
//!
//! ### Installation
//!
//! `$ cargo add mihoyo-api`
//!
//! Also
//!
//! ```toml
//! miHoYo-API = "0.1"
//! ```
//!
//! - Last edit: 22/10/2023



pub(crate) mod component;
pub mod model;
pub mod util;
pub mod types;
pub mod client;



#[cfg(test)]
mod tests {
    use super::client::Client;
    use super::types::{Languages, Game};

    #[tokio::test]
    async fn it_works() {
        let client = Client::default().set_from_env().unwrap();
        let game = client.get_game_account(Some(Languages::JaJp), Game::STARRAIL).await.unwrap();

        let data = client.get_starrail_characters(Some(game.get_uid()), Some(Languages::JaJp))
            .await.unwrap();

        dbg!(data);
    }
}
