// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest_middleware::ClientWithMiddleware,
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
        let http_client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new());
        Configuration {
            base_path: "https://api.datadoghq.com".to_owned(),
            user_agent: Some(format!(
                "datadog-api-client-rust/{} (rust {}; os {}; arch {})",
                option_env!("CARGO_PKG_VERSION").unwrap_or("?"),
                option_env!("DD_RUSTC_VERSION").unwrap_or("?"),
                env::consts::OS,
                env::consts::ARCH,
            )),
            client: http_client.build(),
            api_key_auth: env::var("DD_API_KEY").ok(),
            app_key_auth: env::var("DD_APP_KEY").ok(),
        }
    }
}
