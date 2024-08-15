mod database;
mod gg;

pub use database::*;
pub use gg::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server_port: u16,

    #[serde(flatten, with = "pdb")]
    pub database: Database,
}
