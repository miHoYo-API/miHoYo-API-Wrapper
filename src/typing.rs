use std::collections::HashMap;
use once_cell::sync::Lazy;


pub(crate) type Dict = HashMap<String, String>;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Game {
    Genshin,
    Honkai,
    StarRail,
}
impl Game {
    pub fn which_title(&self) -> String {
        let title = match self {
            Game::Genshin => "genshin",
            Game::Honkai => "honkai",
            Game::StarRail => "starrail",
        };
        title.to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Region {
    OverSeas,
    Chinese,
}
impl Region {
    pub fn which_country(&self) -> String {
        let name = match self {
            Region::OverSeas => "os",
            Region::Chinese => "ch",
        };
        name.to_string()
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Default)]
pub enum Languages {
    ZhCh,
    ZhTw,
    DeDe,
    #[default]
    EnUs,
    EsEs,
    FrFr,
    IdId,
    ItIt,
    JaJp,
    KoKr,
    PtPt,
    RuRu,
    ThTh,
    ViVn,
    TrTr,
}

static LANG_DICT: Lazy<HashMap<&'static str, Languages>> = Lazy::new(|| {
    HashMap::from([
        ("zh-cn", Languages::ZhCh),
        ("zh-tw", Languages::ZhTw),
        ("de-de", Languages::DeDe),
        ("en-us", Languages::EnUs),
        ("es-es", Languages::EsEs),
        ("fr-fr", Languages::FrFr),
        ("id-id", Languages::IdId),
        ("it-it", Languages::ItIt),
        ("ja-jp", Languages::JaJp),
        ("ko-kr", Languages::KoKr),
        ("pt-pt", Languages::PtPt),
        ("ru-ru", Languages::RuRu),
        ("th-th", Languages::ThTh),
        ("vi-vn", Languages::ViVn),
        ("tr-tr", Languages::TrTr),
    ])
});

impl Languages {
    pub fn name(&self) -> String {
        let result = match self {
            Languages::ZhCh => "zh-cn",
            Languages::ZhTw => "zh-tw",
            Languages::DeDe => "de-de",
            Languages::EnUs => "en-us",
            Languages::EsEs => "es-es",
            Languages::FrFr => "fr-fr",
            Languages::IdId => "id-id",
            Languages::ItIt => "it-it",
            Languages::JaJp => "ja-jp",
            Languages::KoKr => "ko-kr",
            Languages::PtPt => "pt-pt",
            Languages::RuRu => "ru-ru",
            Languages::ThTh => "th-th",
            Languages::ViVn => "vi-vn",
            Languages::TrTr => "tr-tr",
        };
        result.to_string()
    }

    pub fn what_is_this(&self) -> String {
        let result = match self {
            Languages::ZhCh => "简体中文",
            Languages::ZhTw => "繁體中文",
            Languages::DeDe => "Deutsch",
            Languages::EnUs => "English",
            Languages::EsEs => "Español",
            Languages::FrFr => "Français",
            Languages::IdId => "Indonesia",
            Languages::ItIt => "Italiano",
            Languages::JaJp => "日本語",
            Languages::KoKr => "한국어",
            Languages::PtPt => "Português",
            Languages::RuRu => "Pусский",
            Languages::ThTh => "ภาษาไทย",
            Languages::ViVn => "Tiếng Việt",
            Languages::TrTr => "Türkçe",
        };
        result.to_string()
    }

    pub fn from_str(str: &str) -> Option<Self> {
        match LANG_DICT.get(str) {
            None => None,
            Some(val) => Some(val.clone())
        }
    }
}

#[cfg(feature = "mihomo")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum StarRailStatusType {
    Hp,
    Atk,
    Def,
    Spd,
    CritRate,
    CritDMG,
}
impl StarRailStatusType {
    pub fn name(&self) -> String {
        let result = match self {
            StarRailStatusType::Hp => "HP",
            StarRailStatusType::Atk => "ATK",
            StarRailStatusType::Def => "DEF",
            StarRailStatusType::Spd => "SPD",
            StarRailStatusType::CritRate => "CRIT_RATE",
            StarRailStatusType::CritDMG => "CRIT_DMG",
        };
        result.to_string()
    }
}


#[cfg(feature = "mihomo")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum RelicType {
    Head,
    Hands,
    Body,
    Feet,
    Ornament(OrnamentType),
}
impl RelicType {
    pub fn name(&self) -> String {
        match self {
            RelicType::Head => "HEAD".to_string(),
            RelicType::Hands => "HANDS".to_string(),
            RelicType::Body => "BODY".to_string(),
            RelicType::Feet => "FEET".to_string(),
            RelicType::Ornament(ornament) => ornament.name(),
        }
    }

    pub fn which_type(specific_name: &String) -> anyhow::Result<RelicType> {
        let result = match specific_name.to_uppercase().as_str() {
            "HEAD" => Self::Head,
            "HANDS" => Self::Hands,
            "BODY" => Self::Body,
            "FEET" => Self::Feet,
            "PLANAR_SPHERE" => Self::Ornament(OrnamentType::PlanarSphere),
            "LINK_ROPE" => Self::Ornament(OrnamentType::LinkRope),
            _ => anyhow::bail!("`{}` is doesn't any matches", specific_name)
        };
        Ok(result)
    }
}

#[cfg(feature = "mihomo")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum OrnamentType {
    PlanarSphere,
    LinkRope
}
impl OrnamentType {
    pub fn name(&self) -> String {
        let result = match self {
            OrnamentType::PlanarSphere => "PLANAR_SPHERE",
            OrnamentType::LinkRope => "LINK_ROPE",
        };
        result.to_string()
    }
}


// pub(crate) enum Iterable<T> {
//     List(T),
//     Vec(Vec<T>),
//     HashMap()
// }