use serde::Deserialize;

#[derive(Debug)]
pub struct UserStats {
    pub stats: PartialUserStats,
    pub info: UserLittleInfo
}
impl UserStats {
    pub(crate) fn new(stats: PartialUserStats, info: UserLittleInfo) -> UserStats {
        UserStats { stats, info }
    }
}


#[derive(Debug, Deserialize)]
pub struct PartialUserStats {
    pub stats: Stats,
    #[serde(rename = "avatar_list")]
    pub characters: Vec<Character>
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub active_days: u32,
    pub avatar_num: u32,
    pub achievement_num: u32,
    pub chest_num: u32,
    pub abyss_process: String,
}


#[derive(Debug, Deserialize)]
pub struct Character {
    pub id: u32,
    pub level: u32,
    pub name: String,
    pub element: String,
    pub icon: String,
    pub rarity: u8,
    pub rank: u32,
    pub is_chosen: bool
}


#[derive(Debug, Deserialize)]
pub struct UserLittleInfo {
    pub avatar: String,
    pub nickname: String,
    pub region: String,
    pub level: u32,
}