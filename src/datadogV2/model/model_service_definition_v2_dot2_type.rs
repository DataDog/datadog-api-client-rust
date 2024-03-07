// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServiceDefinitionV2Dot2Type {
    #[serde(rename = "web")]
    WEB,
    #[serde(rename = "db")]
    DB,
    #[serde(rename = "cache")]
    CACHE,
    #[serde(rename = "function")]
    FUNCTION,
    #[serde(rename = "browser")]
    BROSWER,
    #[serde(rename = "mobile")]
    MOBILE,
    #[serde(rename = "custom")]
    CUSTOM,
}

impl ToString for ServiceDefinitionV2Dot2Type {
    fn to_string(&self) -> String {
        match self {
            Self::WEB => String::from("web"),
            Self::DB => String::from("db"),
            Self::CACHE => String::from("cache"),
            Self::FUNCTION => String::from("function"),
            Self::BROSWER => String::from("browser"),
            Self::MOBILE => String::from("mobile"),
            Self::CUSTOM => String::from("custom"),
        }
    }
}
