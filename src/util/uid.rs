use crate::types::{Game, Region};
use super::constants::UID_RANGE;

pub(crate) fn recognize_genshin_server(uid: &u32) -> anyhow::Result<String> {
    let server = match uid.to_string().char_indices().nth(0).unwrap().1.to_string().as_str() {
        "1" => "cn_gf01",
        "2" => "cn_gf01",
        "5" => "cn_qd01",
        "6" => "os_usa",
        "7" => "os_euro",
        "8" => "os_asia",
        "9" => "os_cht",
        _ => anyhow::bail!("uid doesn't much the server number like smthing"),
    };
    Ok(server.to_string())
}


pub(crate) fn recognize_honkai_server(uid: &u32) -> anyhow::Result<String> {
    let uid = uid.clone();
    let server = if 10000000 < uid && uid < 100000000 {
        "overseas01"
    } else if 100000000 < uid && uid < 200000000 {
        "usa01"
    } else if 200000000 < uid && uid < 300000000 {
        "eur01"
    } else {
        anyhow::bail!("uid doesn't much the server number like smthing")
    };
    Ok(server.to_string())
}


pub(crate) fn recognize_starrail_server(uid: &u32) -> anyhow::Result<String> {
    let server = match uid.to_string().char_indices().nth(0).unwrap().1.to_string().as_str() {
        "1" => "prod_gf_cn",
        "2" => "prod_gf_cn",
        "5" => "prod_qd_cn",
        "6" => "prod_official_usa",
        "7" => "prod_official_eur",
        "8" => "prod_official_asia",
        "9" => "prod_official_cht",
        _ => anyhow::bail!("uid doesn't much the server number like smthing"),
    };
    Ok(server.to_string())
}


pub(crate) fn recognize_server(uid: &u32, game: Game) -> anyhow::Result<String> {
    match game {
        Game::GENSHIN => {
            recognize_genshin_server(uid)
        }
        Game::HONKAI => {
            recognize_honkai_server(uid)
        }
        Game::STARRAIL => {
            recognize_starrail_server(uid)
        }
    }
}



pub(crate) fn recognize_region(uid: &mut u32, game: Game) -> Option<Region> {
    let first = &uid.to_string()[0..1].parse::<u8>().unwrap();
    for (region, dict) in UID_RANGE.get(&game).unwrap() {
        if dict.contains(first) {
            return Some(region.clone())
        }
    }
    None
}