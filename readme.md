<div align="center">
    <img src="https://i.imgur.com/pYG0dpf.png">
    <img alt="Crates.io" src="https://img.shields.io/crates/d/miHoYo-API">
    <img alt="Crates.io (latest)" src="https://img.shields.io/crates/dv/miHoYo-API">
    <img alt="Crates.io (recent)" src="https://img.shields.io/crates/dr/miHoYo-API">
    <img alt="Crates.io (version)" src="https://img.shields.io/crates/dv/miHoYo-API/0.1.5">
    <br>
    <img alt="Discord" src="https://img.shields.io/discord/1058271208442953728?color=9001F5">
    <img alt="Discord" src="https://img.shields.io/discord/1058271208442953728?color=9001F5">
</div>


## Original

[genshin.py](https://github.com/thesadru/genshin.py) by [thesadru](https://github.com/thesadru)

 
## Features
*TBA

| Func ＼ Game | Genshin | Honkai | StarRail |
|:-----------:|:-------:|:------:|:--------:| 
|    User     |    ✓    |        |    ✓     |
|  Character  |    ✓    |        |    ✓     |
|  Challenge  |    ✓    |        |    ✓     |
|    Rogue    |         |        |    ✓     |



## Requirements

```toml
miHoYo-API = "0.1.5"
tokio = { version = "1.33.0", features = ["full"] }
```

```rust
use miHoYo_API::client::Client;
use miHoYo_API::types::Game;

#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.set_from_env().unwrap();

    let starrail = client.get_game_account(Some("ja-jp"),Game::STARRAIL).await.unwrap();
    let genshin = client.get_game_account(Some("ja-jp"),Game::GENSHIN).await.unwrap();

    let starrail_note = client.get_starrail_notes(Some(starrail.get_uid()), Some("ja-jp"))
        .await
        .unwrap();
    dbg!(starrail_note);

    let starrail_stats = client.get_starrail_user(Some(starrail.get_uid()), Some("ja-jp"))
        .await
        .unwrap();
    dbg!(starrail_stats);

    let genshin_spiral = client.get_genshin_spiral_abyss(Some(genshin.get_uid()), None, Some("ja-jp"))
        .await.unwrap();
    dbg!(genshin_spiral);
}
```
