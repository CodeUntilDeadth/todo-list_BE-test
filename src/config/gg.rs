use std::sync::LazyLock;

use super::env_or_default;
pub static GG_CLIENT_ID: LazyLock<String> =
    LazyLock::new(|| env_or_default("GG_CLIENT_ID", "your_client_id".to_string()));
pub static GG_CLIENT_SECRET: LazyLock<String> =
    LazyLock::new(|| env_or_default("GG_CLIENT_SECRET", "your_client_secret".to_string()));
pub static GG_REDIRECT_URI: LazyLock<String> =
    LazyLock::new(|| env_or_default("GG_REDIRECT_URI", "your_redirect_uri".to_string()));
pub static GG_AUTH_URL: LazyLock<String> =
    LazyLock::new(|| env_or_default("GG_AUTH_URL", "your_auth_url".to_string()));
pub static GG_TOKEN_URL: LazyLock<String> =
    LazyLock::new(|| env_or_default("GG_TOKEN_URL", "your_token_url".to_string()));
