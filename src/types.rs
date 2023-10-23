//! Types

use std::any::Any;
use std::collections::HashMap;
use anyhow::bail;

pub(crate) type GeneralResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub(crate) type StringDict = HashMap<String, String>;
pub(crate) type GeneralAny = Box<dyn Any + Send + Sync>;


#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CookieOrHeader {
    Dict(StringDict),
    Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AnyCookieOrHeader {
    CookieOrHeader(CookieOrHeader),
    SequenceCookieOrHeader(Vec<CookieOrHeader>)
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Region {
    OVERSEAS,
    CHINESE,
}
impl Region {
    pub fn name(&self) -> &str {
        match self {
            Region::OVERSEAS => "os",
            Region::CHINESE => "cn",
        }
    }
}


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Game {
    GENSHIN,
    HONKAI,
    STARRAIL,
}
impl Game {
    pub fn name(&self) -> &str {
        match self {
            Game::GENSHIN => "genshin",
            Game::HONKAI => "honkai3rd",
            Game::STARRAIL => "hkrpg",
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
            _ => bail!("`{}` is doesn't any matches", specific_name)
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


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Languages {
    ZhCh,
    ZhTw,
    DeDe,
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
}