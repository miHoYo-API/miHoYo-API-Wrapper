use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rogue {
    pub role: Role,
    pub basic_info: BasicInfo,
    pub current_record: CurrentRecord,
    pub last_record: LastRecord,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub server: String,
    pub nickname: String,
    pub level: u32,
}

#[derive(Debug, Deserialize)]
pub struct BasicInfo {
    pub unlocked_buff_num: u32,
    pub unlocked_miracle_num: u32,
    pub unlocked_skill_points: u32,
}

#[derive(Debug, Deserialize)]
pub struct CurrentRecord {
    pub basic: CurrentRecordBasic,
    pub records: serde_json::Value,
    pub has_data: bool,
    pub best_record: Option<BestRecord>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentRecordBasic {
    pub id: u32,
    pub finish_cnt: u32,
    pub schedule_begin: Schedule,
    pub schedule_end: Schedule,
    pub current_rogue_score: u32,
    pub max_rogue_score: u32
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, Deserialize)]
pub struct BestRecord {
    pub base_type_list: Vec<BaseTypeList>,
    pub buffs: Vec<Buff>,
}

#[derive(Debug, Deserialize)]
pub struct BaseTypeList {
    pub cnt: u8,
    pub id: u16,
    pub name: String,
}


#[derive(Debug, Deserialize)]
pub struct Buff {
    pub base_type: BaseTypeList,
    pub items: Vec<Item>,

}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: u32,
    pub is_evoluted: bool,
    pub name: String,
    pub rank: u8,
}

#[derive(Debug, Deserialize)]
pub struct LastRecord {
    pub basic: LastRecordBasic,
    pub records: serde_json::Value,
    pub has_data: bool,
    pub best_record: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct LastRecordBasic {
    pub id: u32,
    pub finish_cnt: u32,
    pub schedule_begin: Schedule,
    pub schedule_end: Schedule,
    pub current_rogue_score: u32,
    pub max_rogue_score: u32
}

