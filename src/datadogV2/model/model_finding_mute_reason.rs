// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FindingMuteReason {
    #[serde(rename = "PENDING_FIX")]
    PENDING_FIX,
    #[serde(rename = "FALSE_POSITIVE")]
    FALSE_POSITIVE,
    #[serde(rename = "ACCEPTED_RISK")]
    ACCEPTED_RISK,
    #[serde(rename = "NO_PENDING_FIX")]
    NO_PENDING_FIX,
    #[serde(rename = "HUMAN_ERROR")]
    HUMAN_ERROR,
    #[serde(rename = "NO_LONGER_ACCEPTED_RISK")]
    NO_LONGER_ACCEPTED_RISK,
    #[serde(rename = "OTHER")]
    OTHER,
}

impl ToString for FindingMuteReason {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING_FIX => String::from("PENDING_FIX"),
            Self::FALSE_POSITIVE => String::from("FALSE_POSITIVE"),
            Self::ACCEPTED_RISK => String::from("ACCEPTED_RISK"),
            Self::NO_PENDING_FIX => String::from("NO_PENDING_FIX"),
            Self::HUMAN_ERROR => String::from("HUMAN_ERROR"),
            Self::NO_LONGER_ACCEPTED_RISK => String::from("NO_LONGER_ACCEPTED_RISK"),
            Self::OTHER => String::from("OTHER"),
        }
    }
}
