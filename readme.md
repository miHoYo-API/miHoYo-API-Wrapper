<h1 align="center">miHoYo API-Wrapper</h1>

<p align="right">version. 0.1.0</p>


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
[dependencies]
async-trait = "0.1.72"
anyhow = "1.0.72"
dotenv = "0.15.0"
once_cell = "1.18.0"
serde = { version = "1.0.182", features = ["derive"] }
serde_json = "1.0.104"
rand = "0.8.5"
rust-crypto = "0.2.36"

[dependencies.reqwest]
version = "0.11.7"
features = [
    "json",
    "multipart",
    "stream",
    "cookies"
]

[dependencies.tokio]
version = "1.33"
features = ["full"]
```
