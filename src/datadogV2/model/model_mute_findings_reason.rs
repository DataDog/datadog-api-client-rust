// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MuteFindingsReason {
    PENDING_FIX,
    FALSE_POSITIVE,
    OTHER,
    NO_FIX,
    DUPLICATE,
    RISK_ACCEPTED,
    NO_PENDING_FIX,
    HUMAN_ERROR,
    NO_LONGER_ACCEPTED_RISK,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for MuteFindingsReason {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING_FIX => String::from("PENDING_FIX"),
            Self::FALSE_POSITIVE => String::from("FALSE_POSITIVE"),
            Self::OTHER => String::from("OTHER"),
            Self::NO_FIX => String::from("NO_FIX"),
            Self::DUPLICATE => String::from("DUPLICATE"),
            Self::RISK_ACCEPTED => String::from("RISK_ACCEPTED"),
            Self::NO_PENDING_FIX => String::from("NO_PENDING_FIX"),
            Self::HUMAN_ERROR => String::from("HUMAN_ERROR"),
            Self::NO_LONGER_ACCEPTED_RISK => String::from("NO_LONGER_ACCEPTED_RISK"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MuteFindingsReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for MuteFindingsReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PENDING_FIX" => Self::PENDING_FIX,
            "FALSE_POSITIVE" => Self::FALSE_POSITIVE,
            "OTHER" => Self::OTHER,
            "NO_FIX" => Self::NO_FIX,
            "DUPLICATE" => Self::DUPLICATE,
            "RISK_ACCEPTED" => Self::RISK_ACCEPTED,
            "NO_PENDING_FIX" => Self::NO_PENDING_FIX,
            "HUMAN_ERROR" => Self::HUMAN_ERROR,
            "NO_LONGER_ACCEPTED_RISK" => Self::NO_LONGER_ACCEPTED_RISK,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
