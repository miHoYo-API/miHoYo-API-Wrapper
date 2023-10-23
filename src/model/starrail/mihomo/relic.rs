use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use anyhow::bail;
use serde::Deserialize;
use crate::types::RelicType;


type Info = HashMap<String, String>;


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

