use std::collections::HashMap;
use crate::typing::{Game, Region};


pub(crate) trait GameTrait {
    fn new(overseas: Option<&[(Game, &str)]>, chinese: Option<&[(Game, &str)]>) -> GameRoute;
    fn get_url(&self, region: Region, game: Game) ->  anyhow::Result<String>;
}


pub(crate) struct Route(String);

pub(crate) struct InternationalRoute(HashMap<Region, String>);

pub(crate) struct GameRoute(HashMap<Region, HashMap<Game, String>>);


impl Route {
    pub(crate) fn new(url: &str) -> Route {
        Route(url.to_string())
    }

    pub(crate) fn get_url(&self) -> anyhow::Result<String> {
        Ok(self.0.clone())
    }
}

impl InternationalRoute {
    pub(crate) fn new(overseas: &str, chinese: &str) -> InternationalRoute {
        let mut base = HashMap::new();
        base.insert(Region::OverSeas, overseas.to_string());
        base.insert(Region::Chinese, chinese.to_string());
        InternationalRoute(base.clone())
    }

    pub(crate) fn get_url(&self, region: &Region) -> anyhow::Result<&'_ str> {
        if self.0.get(region).is_none() {
            anyhow::bail!(format!("URL does not support `{}` name", region.which_country()));
        }
        Ok(self.0.get(&region).unwrap())
    }
}

impl GameRoute {
    pub(crate) fn new(overseas: Option<&[(Game, &str)]>, chinese: Option<&[(Game, &str)]>) -> GameRoute {
        let mut base = HashMap::new();
        let os = {
            let mut base = HashMap::new();
            if let Some(os) = overseas.clone() {
                for (key, value) in os {
                    base.insert(key.clone(), value.to_string());
                }
            }
            base
        };
        let ch = {
            let mut base = HashMap::new();
            if let Some(os) = chinese {
                for (key, value) in os.into_iter() {
                    base.insert(key.clone(), value.to_string());
                }
            }
            base
        };
        base.insert(Region::OverSeas, os);
        base.insert(Region::Chinese, ch);

        GameRoute(base)
    }

    pub(crate) fn get_url(&self, region: &Region, game: &Game) -> anyhow::Result<String> {
        if self.0.get(&region).is_none() {
            anyhow::bail!(format!("URL does not support {}", region.which_country()));
        };
        if self.0.get(&region).unwrap().get(&game).is_none() {
            anyhow::bail!(format!("URL does not support {}", game.which_title()));
        }
        Ok(self.0.get(&region).unwrap().get(&game).unwrap().clone())
    }
}
