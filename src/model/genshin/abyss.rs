use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpiralAbyss {
    pub schedule_id: u32,
    pub start_time: String,
    pub end_time: String,
    pub total_battle_times: u128,
    pub total_win_times: u128,
    pub max_floor: String,
    pub reveal_rank: serde_json::Value,
    pub defeat_rank: serde_json::Value,
    pub damage_rank: serde_json::Value,
    pub take_damage_rank: serde_json::Value,
    pub normal_skill_rank: serde_json::Value,
    pub energy_skill_rank: serde_json::Value,
    pub floors: serde_json::Value,
    pub total_star: u8,
    pub is_unlock: bool,
}