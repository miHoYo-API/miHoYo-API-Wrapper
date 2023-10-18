use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Characters {
    #[serde(rename = "avatars")]
    pub characters: Vec<GenshinCharacter>,
    pub role: GenshinSimplyRole,
}

#[derive(Debug, Deserialize)]
pub struct GenshinCharacter {
    pub id: u32,
    pub image: String,
    pub icon: String,
    pub name: String,
    pub element: String,
    pub fetter: u8,
    pub level: u8,
    pub rarity: u8,
    pub weapon: GenshinWeapon,
    pub reliquaries: Vec<GenshinRelic>,
    pub constellations: Vec<GenshinConstellation>,
    pub actived_constellation_num: u8,
    pub costumes: Option<Vec<GenshinCostume>>,
    pub external: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GenshinWeapon {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub r#type: u8,
    pub rarity: u8,
    pub level: u8,
    pub promote_level: u8,
    pub type_name: String,
    pub desc: String,
    pub affix_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct GenshinRelic {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub pos: u8,
    pub rarity: u8,
    pub level: u8,
    pub set: GenshinRelicsSet,
    pub pos_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinRelicsSet {
    pub id: u32,
    pub name: String,
    pub affixes: Vec<GenshinRelicsAffixes>,
}

#[derive(Debug, Deserialize)]
pub struct GenshinRelicsAffixes {
    #[serde(rename = "activation_number")]
    pub activation_num: u8,
    pub effect: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinConstellation {
    pub id: u16,
    pub name: String,
    pub icon: String,
    pub effect: String,
    pub is_actived: bool,
    pub pos: u8,
}
impl GenshinConstellation {
    // pub fn extract_text(&self) -> String {
    //     self.effect.replace("")
    // }
    // I will code rid of "<color=#~~~~~~>"
}

#[derive(Debug, Deserialize)]
pub struct GenshinCostume {
    pub id: u32,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct GenshinSimplyRole {
    #[serde(rename = "AvatarUrl")]
    pub avatar_url: String,
    pub nickname: String,
    pub region: String,
    pub level: u8,
}
