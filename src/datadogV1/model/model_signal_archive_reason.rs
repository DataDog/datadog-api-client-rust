// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignalArchiveReason {
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "false_positive")]
    FALSE_POSITIVE,
    #[serde(rename = "testing_or_maintenance")]
    TESTING_OR_MAINTENANCE,
    #[serde(rename = "investigated_case_opened")]
    INVESTIGATED_CASE_OPENED,
    #[serde(rename = "other")]
    OTHER,
}

impl ToString for SignalArchiveReason {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::FALSE_POSITIVE => String::from("false_positive"),
            Self::TESTING_OR_MAINTENANCE => String::from("testing_or_maintenance"),
            Self::INVESTIGATED_CASE_OPENED => String::from("investigated_case_opened"),
            Self::OTHER => String::from("other"),
        }
    }
}

impl Default for SignalArchiveReason {
    fn default() -> SignalArchiveReason {
        Self::NONE
    }
}
