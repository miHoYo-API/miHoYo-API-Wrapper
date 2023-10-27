use std::ops::Add;
use std::time::Duration;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StarRailNote {
    /// A.K.A. Trailblaze Power.
    pub current_stamina: u16,
    /// Max value of Trailblaze Power.
    pub max_stamina: u16,
    /// Full recovery time.
    pub stamina_recover_time: u32,
    /// A limit of accept expeditions

    #[serde(rename = "accepted_epedition_num")]
    pub accepted_expedition_num: u8,
    /// Current Count that expeditions
    pub total_expedition_num: u8,
    /// Details of Expeditions.
    pub expeditions: Option<Vec<Expedition>>,
    /// Current Value of Daily Training.
    pub current_train_score: u16,
    /// Max Value of Daily Training.
    pub max_train_score: u16,
    /// Current Value of Point Rewards on Simulated Universe.
    pub current_rogue_score: u32,
    /// Max Value of Point Rewards on Simulated Universe.
    pub max_rogue_score: u32,
    /// Echo of War count that can get Reward claims.
    pub weekly_cocoon_cnt: u8,
    /// Echo of War attempt Limit that can get Reward claims.
    pub weekly_cocoon_limit: u8,
    /// Current Owned Reserved Trailblaze Power
    pub current_reserve_stamina: u32,
    /// Filled Reserved Trailblaze Power or Not
    pub is_reserve_stamina_full: bool,
}
impl StarRailNote {
    /// The difference from max [`max_stamina`] to [`current_stamina`]
    pub fn diff_stamina(&self) -> u16 {
        self.max_stamina - self.current_stamina
    }

    /// Check the all [`expeditions`] finished or not as `Option<bool>`. If there's no Expeditions return value is `None`
    pub fn is_all_done(&self) -> Option<bool> {
        if self.expeditions.is_none() {
            return None;
        }

        let result = self.expeditions.as_ref().unwrap()
            .iter()
            .all(|expedition| {
                expedition.is_done()
            });
        Some(result)
    }

    /// A simply info of [`expedition`].
    pub fn expedition_details(&self) -> Option<HashMap<String, Duration>> {
        if self.expeditions.is_none() {
            return None;
        }
        let mut details = HashMap::new();

        for info in self.expeditions.as_ref().unwrap() {
            details.insert(info.name.to_string(), Duration::from_secs(info.remaining_time as u64));
        }

        Some(details)
    }

    /// The return value is Stamina recover time as [`Duration`]
    pub fn recover_time_as_duration(&self) -> Duration {
        let duration = Duration::from_secs(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
            .add(Duration::from_secs(self.stamina_recover_time as u64));
        duration
    }
}

/// Detail of Expedition
#[derive(Debug, Deserialize)]
pub struct Expedition {
    /// Dispatched Character(s)
    pub avatars: Vec<String>,
    /// Finished or Not yet
    pub status: String,
    /// Time remaining
    pub remaining_time: u32,
    /// Place Name to expedition
    pub name: String,
    /// image url of Material
    pub item_url: String,
}
impl Expedition {
    /// Check the expedition finished or not as `bool`. If already finished, the return value is true.
    pub fn is_done(&self) -> bool {
        self.status.eq("Finished")
    }
}
