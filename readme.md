<div align="center">
    <img src="https://i.imgur.com/pYG0dpf.png" alt="">
    <img alt="Crates.io" src="https://img.shields.io/crates/d/miHoYo-API">
    <img alt="Crates.io (latest)" src="https://img.shields.io/crates/dv/miHoYo-API">
    <img alt="Crates.io (recent)" src="https://img.shields.io/crates/dr/miHoYo-API">

[//]: # (    <img alt="Crates.io &#40;version&#41;" src="https://img.shields.io/crates/dv/miHoYo-API/0.2.">)
</div>


## CAUTION

- Only **success** get to the data if you've done with [genshin.py](https://github.com/thesadru/genshin.py) even once the __same Cookies__. Cuz I don't know at all fr.
- miHoYoAPI-Wrapper cannot use v2 cookies. Cuz the message was "Login expired" from API. tbh idk how to solution 
- that's all, well, If I had vitality smthing, this crate would be more powerful. but I'm dumb, I'm sorry. 

## Now I'm working on those problems.
And I need a huge help to improvement this, So I will appreciate you help if you text me.

Discord: ennui_lw


## Original

[genshin.py](https://github.com/thesadru/genshin.py) by [thesadru](https://github.com/thesadru)

 
## Features

*TBA

|              | Genshin | Honkai | StarRail |
|:------------:|:-------:|:------:|:--------:| 
|     User     |         |        |          |
|  Characters  |         |        |          |
| Characters*1 |         |        |          |
| Challenge*2  |         |        |          |
|    Notes     |         |        |          |

- *1 Game Characters on Preview
- *2 Spiral Abyss / (None) / Challenge


## How to Use

#### THIS is OLD code. I will edit this

``Cargo.toml``
```toml
miHoYo-API = "0.1"
tokio = { version = "1.33.0", features = ["full"] }
```

``main.rs``
```rust
#[allow(non_snake_case)]
use miHoYo_API::client::Client;
use miHoYo_API::types::Game;


#[tokio::main]
async fn main() {
    let client = Client::default().set_from_env().unwrap();
    let star_rail_id = client.get_game_account(Some("ja-jp"), Game::STARRAIL)
        .await
        .unwrap()
        .get_uid();

    let data = client.get_starrail_notes(Some(star_rail_id), Some("ja-jp"))
        .await
        .unwrap();

    dbg!(data.recover_time_as_duration());
}
```


## TODO

|                | Genshin | Honkai | StarRail |
|:--------------:|:-------:|:------:|:--------:| 
|    preview     |         |        |    ✓     |
|  Calc Relics   |         |        |          |
| ExpeditionUtil |         |        |          |


Last Edit (_26/11/2023_)


## 


## FAQ

※Idk how to expression so I write in Japanese sometime sry

Q. Why is no there Honkai?

A. Idk How to change to public. You'll be known that meant I said.

