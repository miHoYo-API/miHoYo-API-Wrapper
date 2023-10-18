use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::component::routes::{GameRoute, GameTrait, InternationalRoute, InternationalTrait, Route, RouteTrait};
use crate::types::{Region, Game::{self, GENSHIN, HONKAI, STARRAIL}};


pub(crate) static DS_SALT: Lazy<HashMap<Region,
 &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Region::OVERSEAS, "6s25p5ox5y14umn1p61aqyyvbvvl3lrt");
    map.insert(Region::CHINESE, "xV8v4Qu54lUKrEYFZkJhB8cuOh9Asafs");
    map
});

pub(crate) static UID_RANGE: Lazy<HashMap<Game, HashMap<Region, Vec<u8>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    {
        let mut inner = HashMap::new();
        inner.insert(Region::OVERSEAS, vec![6, 7, 8, 9]);
        inner.insert(Region::CHINESE, vec![1, 2, 5]);
        map.insert(GENSHIN, inner);
    }
    {
        let mut inner = HashMap::new();
        inner.insert(Region::OVERSEAS, vec![6, 7, 8, 9]);
        inner.insert(Region::CHINESE, vec![1, 2, 5]);
        map.insert(STARRAIL, inner);
    }
    {
        let mut inner = HashMap::new();
        inner.insert(Region::OVERSEAS, vec![1, 2]);
        inner.insert(Region::CHINESE, vec![3, 4]);
        map.insert(HONKAI, inner);
    }
    map
});


pub(crate) static USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) miHoYoBBS/2.11.1)";


pub(crate) static WEB_STATIC_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://webstatic-sea.hoyoverse.com/",
    "https://webstatic.mihoyo.com/"
));
pub(crate) static WEB_API_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://webapi-os.account.hoyoverse.com/Api/",
    "https://webapi.account.mihoyo.com/Api/",
));
pub(crate) static ACCOUNT_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://api-account-os.hoyolab.com/account/auth/api",
    "https://api-takumi.mihoyo.com/account/auth/api/",
));
pub(crate) static BBS_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://bbs-api-os.hoyolab.com/",
    "https://bbs-api.mihoyo.com/",
));
pub(crate) static BBS_REFERER_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://www.hoyolab.com/",
    "https://bbs.mihoyo.com/",
));
pub(crate) static TAKUMI_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://api-os-takumi.mihoyo.com/",
    "https://api-takumi.mihoyo.com/",
));
pub(crate) static COMMUNITY_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://bbs-api-os.hoyolab.com/community/",
    "https://api-takumi-record.mihoyo.com/community/",
));
pub(crate) static RECORD_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://bbs-api-os.hoyolab.com/game_record/",
    "https://api-takumi-record.mihoyo.com/game_record/app/",
));
pub(crate) static LINEUP_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://sg-pub(crate)lic-api.hoyoverse.com/event/simulatoros/",
    "https://api-takumi.mihoyo.com/event/platsimulator/",
));
pub(crate) static INFO_LEDGER_URL: Lazy<GameRoute> = Lazy::new(|| GameRoute::new(
    Some(&[
        (GENSHIN, "https://sg-hk4e-api.hoyolab.com/event/ysledgeros/month_info"),
        (STARRAIL, "https://sg-pub(crate)lic-api.hoyolab.com/event/srledger/month_info"),
    ]),
    Some(&[
        (GENSHIN, "https://hk4e-api.mihoyo.com/event/ys_ledger/monthInfo"),
        (STARRAIL, "https://api-takumi.mihoyo.com/event/srledger/month_info")
    ])
));
pub(crate) static DETAIL_LEDGER_URL: Lazy<GameRoute> = Lazy::new(|| GameRoute::new(
    Some(&[
        (GENSHIN, "https://sg-hk4e-api.hoyolab.com/event/ysledgeros/month_detail"),
        (STARRAIL, "https://sg-pub(crate)lic-api.hoyolab.com/event/srledger/month_detail"),
    ]),
    Some(&[
        (GENSHIN, "https://hk4e-api.mihoyo.com/event/ys_ledger/monthDetail"),
        (STARRAIL, "https://api-takumi.mihoyo.com/event/srledger/month_detail"),
    ])
));
pub(crate) static CALCULATOR_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://sg-pub(crate)lic-api.hoyoverse.com/event/calculateos/",
    "https://api-takumi.mihoyo.com/event/e20200928calculate/v1/",
));
pub(crate) static CALCULATOR_REFERER_URL: Lazy<Route> = Lazy::new(|| Route::new("https://webstatic.mihoyo.com/ys/event/e20200923adopt_calculator/index.html"));
pub(crate) static TEAPOT_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://sg-hk4e-api.hoyolab.com/event/e20221121ugcos/",
    "",
));
pub(crate) static WIKI_URL: Lazy<Route> = Lazy::new(|| Route::new("https://sg-wiki-api.hoyolab.com/hoyowiki/wapi"));
pub(crate) static HK4E_URL: Lazy<Route> = Lazy::new(|| Route::new("https://sg-hk4e-api.hoyoverse.com/common/hk4e_global/"));
pub(crate) static REWARD_URL: Lazy<GameRoute> = Lazy::new(|| GameRoute::new(
    Some(&[
        (GENSHIN, "https://sg-hk4e-api.hoyolab.com/event/sol?act_id=e202102251931481"),
        (HONKAI, "https://sg-pub(crate)lic-api.hoyolab.com/event/mani?act_id=e202110291205111"),
        (STARRAIL, "https://sg-pub(crate)lic-api.hoyolab.com/event/luna/os?act_id=e202303301540311"),
    ]),
    Some(&[
        (GENSHIN, "https://api-takumi.mihoyo.com/event/bbs_sign_reward/?act_id=e202009291139501"),
        (HONKAI, "https://api-takumi.mihoyo.com/event/luna/?act_id=e202207181446311"),
        (STARRAIL, "https://api-takumi.mihoyo.com/event/luna/?act_id=e202304121516551"),
    ])
));
pub(crate) static CODE_URL: Lazy<GameRoute> = Lazy::new(|| GameRoute::new(
    Some(&[
        (GENSHIN, "https://sg-hk4e-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey"),
        (STARRAIL, "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey")
    ]),
    None,
));
pub(crate) static GACHA_URL: Lazy<GameRoute> = Lazy::new(|| GameRoute::new(
    Some(&[
        (GENSHIN, "https://hk4e-api-os.hoyoverse.com/event/gacha_info/api/"),
        (STARRAIL, "https://api-os-takumi.mihoyo.com/common/gacha_record/api/"),
    ]),
    Some(&[
        (GENSHIN, "https://hk4e-api.mihoyo.com/event/gacha_info/api/"),
        (STARRAIL, "https://api-takumi.mihoyo.com/common/gacha_record/api/")
    ])
));
pub(crate) static YSULOG_URL: Lazy<InternationalRoute> = Lazy::new(|| InternationalRoute::new(
    "https://hk4e-api-os.hoyoverse.com/common/hk4e_self_help_query/User/",
    "https://hk4e-api.mihoyo.com/common/hk4e_self_help_query/User/"
));
pub(crate) static MI18N: Lazy<HashMap<&'_ str,
 &'_ str>> = Lazy::new(|| HashMap::from(
    [(
        "https://webstatic-sea.mihoyo.com/admin/mi18n/bbs_cn/m11241040191111/m11241040191111-{lang}.json",
        "https://mi18n-os.hoyoverse.com/webstatic/admin/mi18n/hk4e_global/m02251421001311/m02251421001311-{lang}.json"
    )]
));
