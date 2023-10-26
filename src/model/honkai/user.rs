use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Test {
    pub message: String
}