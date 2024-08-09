mod gg;
mod server;

use std::{env, str::FromStr};

pub use gg::*;
pub use server::*;

pub fn env_or_default<T: FromStr>(env_name: &str, default: T) -> T {
    match env::var(env_name) {
        Ok(raw) => match raw.replace("\\n", "\n").parse() {
            Ok(value) => value,
            Err(_) => default,
        },
        Err(_) => default,
    }
}
