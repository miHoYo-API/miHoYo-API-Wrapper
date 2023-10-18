use serde::Deserialize;





#[derive(Debug, Deserialize)]
pub struct StarRailNote {
    pub current_stamina: u32,
    pub max_stamina: u32,
    pub stamina_recover_time: u64,
    pub accepted_epedition_num: u32,
    pub total_expedition_num: u32,
    pub expeditions: Option<Vec<Expedition>>,
    pub current_train_score: u32,
    pub max_train_score: u32,
    pub current_rogue_score: u32,
    pub max_rogue_score: u32,
    pub weekly_cocoon_cnt: u32,
    pub weekly_cocoon_limit: u32,
    pub current_reserve_stamina: u32,
    pub is_reserve_stamina_full: bool,
}


#[derive(Debug, Deserialize)]
pub struct Expedition {
    pub avatars: Vec<String>,
    pub status: String,
    pub remaining_time: u32,
    pub name: String,
    pub item_url: String,
}

