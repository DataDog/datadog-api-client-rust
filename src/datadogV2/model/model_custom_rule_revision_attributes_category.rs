// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CustomRuleRevisionAttributesCategory {
    SECURITY,
    BEST_PRACTICES,
    CODE_STYLE,
    ERROR_PRONE,
    PERFORMANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CustomRuleRevisionAttributesCategory {
    fn to_string(&self) -> String {
        match self {
            Self::SECURITY => String::from("SECURITY"),
            Self::BEST_PRACTICES => String::from("BEST_PRACTICES"),
            Self::CODE_STYLE => String::from("CODE_STYLE"),
            Self::ERROR_PRONE => String::from("ERROR_PRONE"),
            Self::PERFORMANCE => String::from("PERFORMANCE"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CustomRuleRevisionAttributesCategory {
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

impl<'de> Deserialize<'de> for CustomRuleRevisionAttributesCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "SECURITY" => Self::SECURITY,
            "BEST_PRACTICES" => Self::BEST_PRACTICES,
            "CODE_STYLE" => Self::CODE_STYLE,
            "ERROR_PRONE" => Self::ERROR_PRONE,
            "PERFORMANCE" => Self::PERFORMANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
