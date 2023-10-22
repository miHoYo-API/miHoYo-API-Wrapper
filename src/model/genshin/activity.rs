use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Activities {
    effigy: serde_json::Value,
    mechanicus: serde_json::Value,
    fleur_fair: serde_json::Value,
    channeller_slab: serde_json::Value,
    martial_legend: serde_json::Value,
    // pub activities: Vec<>,
}




// pub sumo: Option<Sumo>,
// pub game_version: String,
// pub sumo_second: Option<Sumo>,
// pub channeller_slab_copy: Option<RecordOfChanneller>,


#[derive(Debug, Deserialize)]
pub struct Sumo {
    pub records: Vec<Record>,
    pub exists_data: bool,
    pub is_hot: bool,

}


#[derive(Debug, Deserialize)]
pub struct Record {
    pub lineup: Vec<Lineup>,
    pub heraldry_icon: String,
    pub difficulty: u16,
    pub challenge_id: u16,
    pub challenge_name:String,
    pub max_score: u16,
    pub score_multiple: u16,
}

#[derive(Debug, Deserialize)]
pub struct Lineup {
    pub avatars: Vec<Avatar>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub id: u16,
    pub icon: String,
    pub level: u16,
    pub is_trail_avatar: bool,
    pub rarity: u16,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub id: u16,
    pub icon: String,
    pub level: u16,
    pub is_trail_avatar: bool,
    pub rarity: u16,
}

#[derive(Debug, Deserialize)]
pub struct ChannellerSlabCopy {
    pub start_time: u8,
    pub end_time: u8,
    pub total_score: u128,
    pub records: Vec<RecordOfChanneller>,
    pub exists_data: bool,
}


#[derive(Debug, Deserialize)]
pub struct RecordOfChanneller {
    pub avatars: serde_json::Value,
    pub energy: u16,
    pub difficulty: u16,
    pub challenge_id: u16,
    pub challenge_name: String,
    pub max_score: u16,
    pub limit_conditions: serde_json::Value,
    pub score_multiple: u16,
    pub buffs: serde_json::Value,
}