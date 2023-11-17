// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventAlertType {
    #[serde(rename = "error")]
    ERROR,
    #[serde(rename = "warning")]
    WARNING,
    #[serde(rename = "info")]
    INFO,
    #[serde(rename = "success")]
    SUCCESS,
    #[serde(rename = "user_update")]
    USER_UPDATE,
    #[serde(rename = "recommendation")]
    RECOMMENDATION,
    #[serde(rename = "snapshot")]
    SNAPSHOT,
}

impl ToString for EventAlertType {
    fn to_string(&self) -> String {
        match self {
            Self::ERROR => String::from("error"),
            Self::WARNING => String::from("warning"),
            Self::INFO => String::from("info"),
            Self::SUCCESS => String::from("success"),
            Self::USER_UPDATE => String::from("user_update"),
            Self::RECOMMENDATION => String::from("recommendation"),
            Self::SNAPSHOT => String::from("snapshot"),
        }
    }
}

impl Default for EventAlertType {
    fn default() -> EventAlertType {
        Self::ERROR
    }
}
