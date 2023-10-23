pub(crate) mod constants;
pub(crate) mod contain;
pub mod error;
pub(crate) mod kwargs;
pub(crate) mod uid;


pub mod image {
    pub fn get_image(icon: String) -> String {
        format!("https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/{}", icon)
    }
}