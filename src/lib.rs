pub(crate) mod component;
pub mod model;
pub mod util;
pub mod types;
pub mod client;



#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::types::Game;

    #[tokio::test]
    async fn game_power_barabra() {
        let mut client = Client::new();
        client.set_from_env().unwrap();

        let accounts = client.get_game_accounts(None)
            .await
            .unwrap();

        let g = accounts.iter().filter(|a| a.which_game().name().eq("genshin").clone()).next().unwrap();
        let h = accounts.iter().filter(|a| a.which_game().name().eq("hkrpg")).next().unwrap();

        let genshin = client.get_genshin_notes(Some(g.get_uid()), Some("ja-jp"))
            .await.unwrap();
        let starrail = client.get_starrail_notes(Some(h.get_uid()), Some("ja-jp"))
            .await.unwrap();

        println!("Genshin: [{}/{}]", genshin.current_resin, genshin.max_resin);
        println!("Starrail: [{}/{}]", starrail.current_stamina, starrail.max_stamina);
    }


    #[tokio::test]
    async fn record_cards() {
        use std::env::var;

        let mut client = Client::new();
        client.set_from_env().unwrap();
        let records = client.get_record_cards(Some(var("ltuid").unwrap().parse().unwrap()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(records);
    }


    #[tokio::test]
    async fn starrail_notes() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::STARRAIL)
            .await
            .unwrap();

        let x = client.get_starrail_notes(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }

    #[tokio::test]
    async fn starrail_user() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::STARRAIL)
            .await
            .unwrap();

        let x = client.get_starrail_user(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }

    #[tokio::test]
    async fn starrail_challenges() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::STARRAIL)
            .await
            .unwrap();

        let x = client.get_starrail_challenge(Some(account.get_uid()), None, Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }

    #[tokio::test]
    async fn starrail_rogue() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::STARRAIL)
            .await
            .unwrap();

        let x = client.get_starrail_rogue(Some(account.get_uid()), None, Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }



    #[tokio::test]
    async fn genshin_notes() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::GENSHIN)
            .await
            .unwrap();

        let x = client.get_genshin_notes(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }

    #[tokio::test]
    async fn genshin_partial_user() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::GENSHIN)
            .await
            .unwrap();

        let x = client.get_genshin_partial_user(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }


    #[tokio::test]
    async fn genshin_characters() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::GENSHIN)
            .await
            .unwrap();

        let x = client.get_genshin_characters(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }


    #[tokio::test]
    async fn genshin_user() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::GENSHIN)
            .await
            .unwrap();

        let x = client.get_genshin_user(Some(account.get_uid()), Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }

    #[tokio::test]
    async fn genshin_spiral_abyss() {
        let mut client = Client::new();
        client.set_from_env().unwrap();
        let account = client.get_game_account(None, Game::GENSHIN)
            .await
            .unwrap();

        let x = client.get_genshin_spiral_abyss(Some(account.get_uid()), None, Some("ja-jp"))
            .await
            .unwrap();
        dbg!(x);
    }
}