use lazy_static::lazy_static;

use crate::configuration::{Config, get_config};

lazy_static! {
    pub static ref CONFIG: Config = get_config().unwrap();
}

pub mod db;
pub mod error;
pub mod igdb;
pub mod models;
pub mod configuration;
