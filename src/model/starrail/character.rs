use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Characters {
    #[serde(rename = "avatar_list")]
    pub list: Vec<CharacterDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterDetails {
    pub id: u32,
    pub level: u32,
    pub name: String,
    pub element: String,
    pub icon: String,
    pub rarity: u32,
    pub rank: u8,
    pub image: String,
    pub equip: Option<Equipment>,
    pub relics: Option<Vec<Relic>>,
    pub ornaments: Option<Vec<Ornament>>,
    pub ranks: Vec<Rank>,
}

#[derive(Debug, Deserialize)]
pub struct Equipment {
    pub id: u32,
    pub level: u32,
    pub rank: u32,
    pub name: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Relic {
    pub id: u32,
    pub pos: u32,
    pub name: String,
    pub desc: String,
    pub icon: String,
    pub rarity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Ornament {
    pub id: u32,
    pub level: u8,
    pub pos: u8,
    pub name: String,
    pub desc: String,
    pub rarity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub id: u32,
    pub pos: u32,
    pub name: String,
    pub icon: String,
    pub desc: String,
    pub is_unlocked: bool
}
