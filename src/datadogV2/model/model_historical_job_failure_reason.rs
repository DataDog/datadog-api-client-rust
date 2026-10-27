// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoricalJobFailureReason {
    RULE_CONFIGURATION,
    QUERY_TOO_EXPENSIVE,
    INTERNAL_ERROR,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for HistoricalJobFailureReason {
    fn to_string(&self) -> String {
        match self {
            Self::RULE_CONFIGURATION => String::from("rule_configuration"),
            Self::QUERY_TOO_EXPENSIVE => String::from("query_too_expensive"),
            Self::INTERNAL_ERROR => String::from("internal_error"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for HistoricalJobFailureReason {
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

impl<'de> Deserialize<'de> for HistoricalJobFailureReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rule_configuration" => Self::RULE_CONFIGURATION,
            "query_too_expensive" => Self::QUERY_TOO_EXPENSIVE,
            "internal_error" => Self::INTERNAL_ERROR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
