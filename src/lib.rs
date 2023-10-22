//! miHoYo-API Wrapper is a API wrapper literally.
//!
//! ### Here's Original
//!
//! [genshin.py](https://github.com/thesadru/genshin.py) by [thesadru](https://github.com/thesadru)
//! With Grateful Respect.
//!
//!
//! ### Installation
//!
//! `$ cargo add mihoyo-api`
//!
//! Also
//!
//! ```toml
//! miHoYo-API = "0.1"
//! ```
//!
//! - Last edit: 22/10/2023


#![allow(non_snake_case)]


pub(crate) mod component;
mod model;
mod util;
pub mod types;
pub mod client;

