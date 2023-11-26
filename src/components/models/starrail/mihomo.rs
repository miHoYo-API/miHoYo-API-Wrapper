use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use anyhow::bail;
use serde::Deserialize;
use crate::typing::RelicType;


type Info = HashMap<String, String>;


#[derive(Debug, Deserialize)]
pub struct Mihomo {
    pub player: Player,
    pub characters: Vec<Characters>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub uid: String,
    pub nickname: String,
    pub level: u32,
    pub world_level: u32,
    pub friend_count: u32,
    pub avatar: Avatar,
    pub signature: String,
    pub is_display: bool,
    pub space_info: SpaceInfo,
}

#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct SpaceInfo {
    pub challenge_data: ChallengeData,
    pub pass_area_progress: u32,
    pub light_cone_count: u32,
    pub avatar_count: u32,
    pub achievement_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeData {
    pub maze_group_id: u32,
    pub maze_group_index: u32,
    pub pre_maze_group_index: u32
}

#[derive(Debug, Deserialize)]
pub struct Characters {
    pub id: String,
    pub name: String,
    pub rarity: u32,
    pub level: u32,
    pub promotion: u32,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub rank_icons: Vec<String>,
    pub path: Path,
    pub element: Element,
    pub skills: Vec<Skills>,
    pub skill_trees: Vec<SkillTrees>,
    pub light_cone: Option<LightCone>,
    pub relics: Vec<Relic>,
    pub properties: Vec<Properties>,
}
impl Characters {
    /// A set of [`types::RelicType`] and [`Relic`]
    pub fn collect_relics_with_type(&self) -> anyhow::Result<Vec<(RelicType, Relic)>> {
        let relic_info = relic_deserialize().unwrap();
        let mut base = vec![];

        for relic in self.relics.iter().cloned() {
            let relic_type = relic_info.what_type(&relic.id).unwrap();
            base.push((relic_type, relic));
        };

        Ok(base)
    }
}

#[derive(Debug, Deserialize)]
pub struct Path {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct Skills {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub element: Option<Element>,
    pub r#type: String,
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct SkillTrees {
    pub id: String,
    pub level: u32,
    pub anchor: String,
    pub max_level: u32,
    pub icon: String,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub rarity: u32,
    pub rank: u32,
    pub level: u32,
    pub promotion: u32,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub attributes: Vec<Attributes>,
    pub properties: Vec<Properties>,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub set_id: String,
    pub set_name: String,
    pub rarity: u32,
    pub level: u32,
    pub icon: String,
    pub main_affix: MainAffix,
    pub sub_affix: Vec<SubAffix>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MainAffix {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubAffix {
    pub r#type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    pub display: String,
    pub percent: bool,
    pub count: u32,
    pub step: u32,
}

#[derive(Debug, Deserialize)]
pub struct RelicsSets {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub num: u32,
    pub desc: u32,
    pub properties: Vec<Properties>,
}



#[derive(Debug, Deserialize)]
pub(crate) struct RelicsInfo {
    pub(crate) thirty_thousand: Vec<Info>,
    pub(crate) forty_thousand: Vec<Info>,
    pub(crate) fifty_thousand: Vec<Info>,
    pub(crate) sixty_thousand: Vec<Info>,
}
impl RelicsInfo {
    pub(crate) fn what_type(&self, id: &String) -> anyhow::Result<RelicType> {
        let first = id.char_indices().nth(0).unwrap().1;

        let entry = match first.to_string().as_str() {
            "3" => self.thirty_thousand.iter(),
            "4" => self.forty_thousand.iter(),
            "5" => self.fifty_thousand.iter(),
            "6" => self.sixty_thousand.iter(),
            _ => bail!("UNDEFINED")
        };
        let relic = entry.filter(|info| info.get("id").unwrap().eq(id)).next().unwrap();
        let type_name = relic.get("type").unwrap();
        RelicType::which_type(type_name)
    }
}

pub(crate) fn relic_deserialize() -> anyhow::Result<RelicsInfo> {
    let file = File::open("resources/relic_info.json").unwrap();
    let reader = BufReader::new(file);
    let x: RelicsInfo = serde_json::from_reader(reader).unwrap();
    Ok(x)
}

