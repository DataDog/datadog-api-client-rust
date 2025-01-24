// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleSeverity {
    CRITICAL,
    HIGH,
    MEDIUM,
    LOW,
    UNKNOWN,
    INFO,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RuleSeverity {
    fn to_string(&self) -> String {
        match self {
            Self::CRITICAL => String::from("critical"),
            Self::HIGH => String::from("high"),
            Self::MEDIUM => String::from("medium"),
            Self::LOW => String::from("low"),
            Self::UNKNOWN => String::from("unknown"),
            Self::INFO => String::from("info"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RuleSeverity {
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

impl<'de> Deserialize<'de> for RuleSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "critical" => Self::CRITICAL,
            "high" => Self::HIGH,
            "medium" => Self::MEDIUM,
            "low" => Self::LOW,
            "unknown" => Self::UNKNOWN,
            "info" => Self::INFO,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
