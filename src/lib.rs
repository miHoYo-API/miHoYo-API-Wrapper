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
        // let client = Client::default().set_cookies("88303400", "mIiIw4qsb275z8D1UFPNihNJdoKz5hMjs16dEKzw").unwrap();
        // let client = Client::default().set_cookies("119480035", "cnF7TiZqHAAvYqgCBoSPx5EjwezOh1ZHoqSHf7dT").unwrap();
        // let client = Client::default().set_cookies("330483524", "3IERzJyX4qE4skSt1quVsDgqOQXotLwqRquQgAeG").unwrap();
        // let client = Client::default().set_cookies(
        //     "290341787",
        //     "v2_CAISDGM5b3FhcTNzM2d1OBokNTllMmUxNDQtNWVmMy00NjE4LThkYzItMDQ5NzEwOGVlYzNmIJG-76kGKJOF1Ukwm4e5igFCC2Jic19vdmVyc2Vh"
        // ).unwrap();

        let client = Client::default().set_from_env().unwrap();
        let game = client.get_game_account(Some(Languages::JaJp), Game::STARRAIL).await.unwrap().get_uid();

        let data = client.get_starrail_characters(Some(game), Some(Languages::JaJp))
            .await.unwrap();

        dbg!(data);
    }
}

// ltoken : 3IERzJyX4qE4skSt1quVsDgqOQXotLwqRquQgAeG
//
// ltuid : 330483524