use std::sync::LazyLock;

use super::env_or_default;

pub static SERVER_PORT: LazyLock<u16> = LazyLock::new(|| env_or_default("SERVER_PORT", 3005));
