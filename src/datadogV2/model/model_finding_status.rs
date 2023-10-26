// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FindingStatus {
    #[serde(rename = "critical")]
    CRITICAL,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "info")]
    INFO,
}

impl ToString for FindingStatus {
    fn to_string(&self) -> String {
        match self {
            Self::CRITICAL => String::from("critical"),
            Self::HIGH => String::from("high"),
            Self::MEDIUM => String::from("medium"),
            Self::LOW => String::from("low"),
            Self::INFO => String::from("info"),
        }
    }
}

impl Default for FindingStatus {
    fn default() -> FindingStatus {
        Self::CRITICAL
    }
}
