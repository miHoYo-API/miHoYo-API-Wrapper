// use std::time::Duration;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub schedule_id: u32,
    pub begin_time: ScoreTime,
    pub end_time: ScoreTime,
    pub star_num: u8,
    pub max_floor: String,
    pub battle_num: u8,
    pub has_data: bool,
    pub max_floor_detail: serde_json::Value,
    pub all_floor_detail: Vec<FloorDetail>,
    pub max_floor_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct FloorDetail {
    pub name: String,
    pub round_num: u8,
    pub star_num: u8,
    pub node_1: Node,
    pub node_2: Node,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub challenge_time: ScoreTime,
    pub avatars: Vec<FloorCharacter>
}

#[derive(Debug, Deserialize)]
pub struct ScoreTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}
impl ScoreTime {
    // fn to_datetime(&self) -> Duration {
    //     Duration::new()
    // }
}

#[derive(Debug, Deserialize)]
pub struct FloorCharacter {
    pub id: u32,
    pub level: u32,
    pub icon: String,
    pub rarity: u8,
    pub element: String,
    pub rank: u32,
}
