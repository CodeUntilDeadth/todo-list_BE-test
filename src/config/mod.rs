mod gg;

pub use gg::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server_port: u16,
}
