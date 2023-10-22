use serde::Deserialize;

use crate::types::Game;

#[derive(Debug,
 Deserialize)]
pub struct AccountList {
    pub list: Vec<Account>
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub game_biz: String,
    #[serde(rename = "region")]
    pub server: String,
    #[serde(rename = "game_uid")]
    pub uid: String,
    pub nickname: String,
    pub level: u32,
    pub is_chosen: bool,
    #[serde(rename = "region_name")]
    pub server_name: String,
    pub is_official: bool,
}
impl Account {
    pub fn which_game(&self) -> Game {
        match self.game_biz.as_str() {
            "hk4e_global" => Game::GENSHIN,
            "bh3_global" => Game::HONKAI,
            _ => Game::STARRAIL
        }
    }
    pub fn get_uid(&self) -> u32 {
        self.uid.parse().unwrap()
    }
}


#[derive(Debug, Deserialize)]
pub struct RecordCards {
    pub retcode: u32,
    pub message: String,
    pub data: RecordCardList,
}

#[derive(Debug, Deserialize)]
pub struct RecordCardList {
    pub list: Vec<RecordCard>
}

#[derive(Debug, Deserialize)]
pub struct RecordCard {
    pub has_role: bool,
    pub game_id: i32,
    pub game_role_id: String,
    pub nickname: String,
    pub region: String,
    pub level: i8,
    pub background_image: String,
    pub is_public: bool,
    pub data: Vec<RecordData>,
    #[serde(rename = "region_name")]
    pub server_name: String,
    pub url: String,
    pub data_switches: Vec<DataSwitch>,
    // pub h5_data_switches: serde_json::Value,
    pub background_color: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordData {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: i8,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct DataSwitch {
    pub switch_id: i8,
    pub is_public: bool,
    pub switch_name: String,
}