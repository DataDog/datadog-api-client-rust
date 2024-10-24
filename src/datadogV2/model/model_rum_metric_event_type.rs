// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RumMetricEventType {
    SESSION,
    VIEW,
    ACTION,
    ERROR,
    RESOURCE,
    LONG_TASK,
    VITAL,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RumMetricEventType {
    fn to_string(&self) -> String {
        match self {
            Self::SESSION => String::from("session"),
            Self::VIEW => String::from("view"),
            Self::ACTION => String::from("action"),
            Self::ERROR => String::from("error"),
            Self::RESOURCE => String::from("resource"),
            Self::LONG_TASK => String::from("long_task"),
            Self::VITAL => String::from("vital"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RumMetricEventType {
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

impl<'de> Deserialize<'de> for RumMetricEventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "session" => Self::SESSION,
            "view" => Self::VIEW,
            "action" => Self::ACTION,
            "error" => Self::ERROR,
            "resource" => Self::RESOURCE,
            "long_task" => Self::LONG_TASK,
            "vital" => Self::VITAL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
