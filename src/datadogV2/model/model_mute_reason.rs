// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MuteReason {
    DUPLICATE,
    EXTERNAL_SOLUTION,
    FALSE_POSITIVE,
    INTERNAL_SOLUTION,
    NO_FIX_AVAILABLE,
    OTHER,
    PENDING_FIX,
    RISK_ACCEPTED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for MuteReason {
    fn to_string(&self) -> String {
        match self {
            Self::DUPLICATE => String::from("duplicate"),
            Self::EXTERNAL_SOLUTION => String::from("external_solution"),
            Self::FALSE_POSITIVE => String::from("false_positive"),
            Self::INTERNAL_SOLUTION => String::from("internal_solution"),
            Self::NO_FIX_AVAILABLE => String::from("no_fix_available"),
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
            "external_solution" => Self::EXTERNAL_SOLUTION,
            "false_positive" => Self::FALSE_POSITIVE,
            "internal_solution" => Self::INTERNAL_SOLUTION,
            "no_fix_available" => Self::NO_FIX_AVAILABLE,
            "other" => Self::OTHER,
            "pending_fix" => Self::PENDING_FIX,
            "risk_accepted" => Self::RISK_ACCEPTED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
