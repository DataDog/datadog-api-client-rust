// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use rustc_version::{version, Version};
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub api_key_auth: Option<String>,
    pub app_key_auth: Option<String>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.datadoghq.com".to_owned(),
            user_agent: Some(format!(
                "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
                env::var("CARGO_PKG_VERSION").unwrap_or("?".to_owned()),
                version().unwrap_or(Version::new(0, 0, 0)),
                env::consts::OS,
                env::consts::ARCH,
            )),
            client: reqwest::Client::new(),
            api_key_auth: env::var("DD_API_KEY").ok(),
            app_key_auth: env::var("DD_APP_KEY").ok(),
        }
    }
}
