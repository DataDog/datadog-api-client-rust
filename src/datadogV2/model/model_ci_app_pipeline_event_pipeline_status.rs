// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CIAppPipelineEventPipelineStatus {
    SUCCESS,
    ERROR,
    CANCELED,
    SKIPPED,
    BLOCKED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CIAppPipelineEventPipelineStatus {
    fn to_string(&self) -> String {
        match self {
            Self::SUCCESS => String::from("success"),
            Self::ERROR => String::from("error"),
            Self::CANCELED => String::from("canceled"),
            Self::SKIPPED => String::from("skipped"),
            Self::BLOCKED => String::from("blocked"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CIAppPipelineEventPipelineStatus {
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

impl<'de> Deserialize<'de> for CIAppPipelineEventPipelineStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "success" => Self::SUCCESS,
            "error" => Self::ERROR,
            "canceled" => Self::CANCELED,
            "skipped" => Self::SKIPPED,
            "blocked" => Self::BLOCKED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
