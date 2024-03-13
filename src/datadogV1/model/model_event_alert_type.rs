// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventAlertType {
    ERROR,
    WARNING,
    INFO,
    SUCCESS,
    USER_UPDATE,
    RECOMMENDATION,
    SNAPSHOT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for EventAlertType {
    fn to_string(&self) -> String {
        match self {
            Self::ERROR => String::from("error"),
            Self::WARNING => String::from("warning"),
            Self::INFO => String::from("info"),
            Self::SUCCESS => String::from("success"),
            Self::USER_UPDATE => String::from("user_update"),
            Self::RECOMMENDATION => String::from("recommendation"),
            Self::SNAPSHOT => String::from("snapshot"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for EventAlertType {
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

impl<'de> Deserialize<'de> for EventAlertType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "error" => Self::ERROR,
            "warning" => Self::WARNING,
            "info" => Self::INFO,
            "success" => Self::SUCCESS,
            "user_update" => Self::USER_UPDATE,
            "recommendation" => Self::RECOMMENDATION,
            "snapshot" => Self::SNAPSHOT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
