use std::collections::HashMap;
use crate::types::{
    GeneralResult,
 Region,
 Game,

};

type Dict<'a,
 T> = HashMap<T,
 &'a str>;


pub(crate) trait RouteTrait<'a> {
    fn new(url: &'a str) -> Route {
        Route(url)
    }
    fn get_url(&self) -> GeneralResult<&'_ str>;
}

pub(crate) trait InternationalTrait<'a> {
    fn new(overseas: &'a str,
 chinese: &'a str) -> InternationalRoute<'a>;
    fn get_url(&self,
 region: Region) ->  GeneralResult<&'_ str>;
}

pub(crate) trait GameTrait<'a> {
    fn new(overseas: Option<&'a[(Game,
 &'a str)]>,
 chinese: Option<&'a[(Game,
 &'a str)]>) -> GameRoute<'a>;
    fn get_url(&self,
 region: Region,
 game: Game) ->  GeneralResult<&'_ str>;
}


pub(crate) struct Route<'a>(&'a str);

pub(crate) struct InternationalRoute<'a>(Dict<'a,
 Region>);

pub(crate) struct GameRoute<'a>(HashMap<&'a Region,
 Dict<'a,
 Game>>);


impl RouteTrait<'_> for Route<'_> {
    fn get_url(&self) -> GeneralResult<&'_ str> {
        Ok(self.0)
    }
}

impl InternationalTrait<'_> for InternationalRoute<'_> {
    fn new<'a>(overseas: &'a str,
 chinese: &'a str) -> InternationalRoute<'a> {
        let mut base = Dict::new();
        base.insert(Region::OVERSEAS,
 overseas);
        base.insert(Region::CHINESE,
 chinese);
        InternationalRoute(base)
    }

    fn get_url(&self,
 region: Region) -> GeneralResult<&'_ str> {
        if self.0.get(&region).is_none() {
            return Err(Box::try_from(format!("URL does not support `{}` name",
 region.name())).unwrap());
        }
        Ok(self.0.get(&region).unwrap())
    }
}

impl GameTrait<'_> for GameRoute<'_> {
    fn new<'a>(overseas: Option<&'a[(Game,
 &'a str)]>,
 chinese: Option<&'a[(Game,
 &'a str)]>) -> GameRoute<'a> {
        let mut base = HashMap::new();
        let os = {
            let mut base = Dict::<Game>::new();
            if let Some(os) = overseas {
                for item in os {
                    base.insert(item.0,
 item.1);
                }
            }
            base
        };
        let ch = {
            let mut base = Dict::<Game>::new();
            if let Some(os) = chinese {
                for item in os {
                    base.insert(item.0,
 item.1);
                }
            }
            base
        };
        base.insert(&Region::OVERSEAS,
 os);
        base.insert(&Region::CHINESE,
 ch);

        GameRoute(base)
    }

    fn get_url(&self,
 region: Region,
 game: Game) -> GeneralResult<&'_ str> {
        if self.0.get(&region).is_none() {
            return Err(Box::try_from(format!("URL does not support {}",
 region.name())).unwrap());
        };
        if self.0.get(&region).unwrap().get(&game).is_none() {
            return Err(Box::try_from(format!("URL does not support {}",
 game.name())).unwrap());
        }
        Ok(self.0.get(&region).unwrap().get(&game).unwrap())
    }
}
