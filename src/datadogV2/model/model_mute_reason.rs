// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MuteReason {
    DUPLICATE,
    FALSE_POSITIVE,
    NO_FIX,
    OTHER,
    PENDING_FIX,
    RISK_ACCEPTED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for MuteReason {
    fn to_string(&self) -> String {
        match self {
            Self::DUPLICATE => String::from("duplicate"),
            Self::FALSE_POSITIVE => String::from("false_positive"),
            Self::NO_FIX => String::from("no_fix"),
            Self::OTHER => String::from("other"),
            Self::PENDING_FIX => String::from("pending_fix"),
            Self::RISK_ACCEPTED => String::from("risk_accepted"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MuteReason {
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

impl<'de> Deserialize<'de> for MuteReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "duplicate" => Self::DUPLICATE,
            "false_positive" => Self::FALSE_POSITIVE,
            "no_fix" => Self::NO_FIX,
            "other" => Self::OTHER,
            "pending_fix" => Self::PENDING_FIX,
            "risk_accepted" => Self::RISK_ACCEPTED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
