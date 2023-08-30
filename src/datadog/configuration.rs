// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::env::consts::{ARCH, OS};
use std::env::var;

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
        let sdk_version = match var("CARGO_PKG_VERSION") {
            Ok(v) => v,
            Err(_) => "0.0.0".to_owned(),
        };
        Configuration {
            base_path: "https://api.datadoghq.com".to_owned(),
            user_agent: Some(format!(
                "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
                sdk_version, "", OS, ARCH
            )),
            client: reqwest::Client::new(),
            api_key_auth: None,
            app_key_auth: None,
        }
    }
}
