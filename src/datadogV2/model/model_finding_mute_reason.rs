// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FindingMuteReason {
    PENDING_FIX,
    FALSE_POSITIVE,
    ACCEPTED_RISK,
    NO_PENDING_FIX,
    HUMAN_ERROR,
    NO_LONGER_ACCEPTED_RISK,
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

impl Serialize for FindingMuteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for FindingMuteReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PENDING_FIX" => Self::PENDING_FIX,
            "FALSE_POSITIVE" => Self::FALSE_POSITIVE,
            "ACCEPTED_RISK" => Self::ACCEPTED_RISK,
            "NO_PENDING_FIX" => Self::NO_PENDING_FIX,
            "HUMAN_ERROR" => Self::HUMAN_ERROR,
            "NO_LONGER_ACCEPTED_RISK" => Self::NO_LONGER_ACCEPTED_RISK,
            "OTHER" => Self::OTHER,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
