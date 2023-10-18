use serde::Deserialize;
use crate::model::genshin::character::GenshinCharacter;

#[derive(Debug, Deserialize)]
pub struct UserWithCharacters {
    pub user: PartialUser,
    pub characters: Vec<GenshinCharacter>,
}
impl UserWithCharacters {
    pub(crate) fn new(user: PartialUser, characters: Vec<GenshinCharacter>) -> UserWithCharacters {
        Self { user, characters }
    }
}

#[derive(Debug, Deserialize)]
pub struct PartialUser {
    pub role: GenshinRole,
    pub avatars: Vec<GenshinAvatar>,
    pub stats: GenshinStats,
    pub city_explorations: Option<serde_json::Value>,
    pub world_explorations: Vec<GenshinWorldExploration>,
    pub homes: Vec<GenshinHome>,
    pub query_tool_link: String,
    pub query_tool_image: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinRole {
    #[serde(rename = "AvatarUrl")]
    pub avatar_url: String,
    pub nickname: String,
    pub region: String,
    pub level: u8,
    pub game_head_icon: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinAvatar {
    pub id: u32,
    pub image: String,
    pub name: String,
    pub element: String,
    pub fetter: u8,
    pub level: u8,
    pub rarity: u8,
    pub actived_constellation_num: u8,
    pub card_image: String,
    pub is_chosen: bool,
}

#[derive(Debug, Deserialize)]
pub struct GenshinStats {
    pub active_day_number: u16,
    pub achievement_number: u16,
    pub anemoculus_number: u16,
    pub geoculus_number: u16,
    pub avatar_number: u8,
    pub way_point_number: u16,
    pub domain_number: u16,
    pub spiral_abyss: String,
    pub precious_chest_number: u16,
    pub luxurious_chest_number: u16,
    pub exquisite_chest_number: u16,
    pub common_chest_number: u16,
    pub electroculus_number: u16,
    pub magic_chest_number: u16,
    pub dendroculus_number: u16,
    pub hydroculus_number: u16,
    pub field_ext_map: GenshinFieldExtMap,
}

#[derive(Debug, Deserialize)]
pub struct GenshinFieldExtMap {
    pub magic_chest_number: GenshinPairOfMap,
    pub exquisite_chest_number: GenshinPairOfMap,
    pub way_point_number: GenshinPairOfMap,
    pub geoculus_number: GenshinPairOfMap,
    pub luxurious_chest_number: GenshinPairOfMap,
    pub avatar_number: GenshinPairOfMap,
    pub spiral_abyss: GenshinPairOfMap,
    pub domain_number: GenshinPairOfMap,
    pub dendroculus_number: GenshinPairOfMap,
    pub common_chest_number: GenshinPairOfMap,
    pub anemoculus_number: GenshinPairOfMap,
    pub hydroculus_number: GenshinPairOfMap,
    pub electroculus_number: GenshinPairOfMap,
}

#[derive(Debug, Deserialize)]
pub struct GenshinPairOfMap {
    pub link: String,
    pub backup_link: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinWorldExploration {
    pub level: u16,
    pub exploration_percentage: u16,
    pub icon: String,
    pub name: String,
    pub r#type: String,
    pub offerings: Vec<GenshinOffering>,
    pub id: u16,
    pub parent_id: u8,
    pub map_url: String,
    pub strategy_url: String,
    pub background_image: String,
    pub inner_icon: String,
    pub cover: String,
    pub area_exploration_list: Option<Vec<AreaExploration>>,
    pub boss_list: Option<Vec<Boss>>,
    pub is_hot: bool,
}

#[derive(Debug, Deserialize)]
pub struct GenshinOffering {
    pub name: String,
    pub level: u16,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct AreaExploration {
    pub exploration_percentage: u8,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Boss {
    pub kill_num: u16,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinHome {
    pub level: u16,
    pub visit_num: u16,
    pub comfort_num: u16,
    pub item_num: u16,
    pub name: String,
    pub icon: String,
    pub comfort_level_name: String,
    pub comfort_level_icon: String,
}