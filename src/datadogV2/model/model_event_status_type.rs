// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventStatusType {
    #[serde(rename = "failure")]
    FAILURE,
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

impl ToString for EventStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::FAILURE => String::from("failure"),
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
