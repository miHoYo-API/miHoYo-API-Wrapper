use std::collections::HashMap;
use serde::Deserialize;
use crate::model::genshin::base::ScheduleTime;



#[derive(Debug, Deserialize)]
pub struct GenshinNote {
    pub current_resin: u32,
    pub max_resin: u32,
    pub resin_recovery_time: String,
    pub finished_task_num: u8,
    pub total_task_num: u8,
    pub is_extra_task_reward_received: bool,
    pub remain_resin_discount_num: u32,
    pub current_expedition_num: u32,
    pub max_expedition_num: u32,
    pub expeditions: Vec<GenshinExpedition>,
    pub current_home_coin: u32,
    pub max_home_coin: u32,
    pub home_coin_recovery_time: String,
    pub calendar_url: String,
    pub transformer: GenshinTransformer,
    pub daily_task: GenshinDailyTask
}

#[derive(Debug, Deserialize)]
pub struct GenshinExpedition {
    pub avatar_side_icon: String,
    pub status: String,
    pub remained_time: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinTransformer {
    pub obtained: bool,
    pub recovery_time: ScheduleTime,
    pub wiki: String,
    pub noticed: bool,
    pub latest_job_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinDailyTask {
    pub total_num: u8,
    pub finished_num: u8,
    pub is_extra_task_reward_received: bool,
    pub task_rewards: Vec<HashMap<String, String>>,
    pub attendance_rewards: Vec<AttendanceReward>,
    pub attendance_visible: bool,
}

#[derive(Debug, Deserialize)]
pub struct AttendanceReward {
    pub status: String,
    pub progress: u32,
}