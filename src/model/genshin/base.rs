use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct ScheduleTime {
    #[serde(rename = "Day")]
    pub day: u8,
    #[serde(rename = "Hour")]
    pub hour: u8,
    #[serde(rename = "Minute")]
    pub minute: u8,
    #[serde(rename = "Second")]
    pub second: u8,
    pub reached: bool,
}