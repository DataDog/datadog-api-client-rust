// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonitorOptionsNotificationPresets {
    SHOW_ALL,
    HIDE_QUERY,
    HIDE_HANDLES,
    HIDE_ALL,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for MonitorOptionsNotificationPresets {
    fn to_string(&self) -> String {
        match self {
            Self::SHOW_ALL => String::from("show_all"),
            Self::HIDE_QUERY => String::from("hide_query"),
            Self::HIDE_HANDLES => String::from("hide_handles"),
            Self::HIDE_ALL => String::from("hide_all"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonitorOptionsNotificationPresets {
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

impl<'de> Deserialize<'de> for MonitorOptionsNotificationPresets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "show_all" => Self::SHOW_ALL,
            "hide_query" => Self::HIDE_QUERY,
            "hide_handles" => Self::HIDE_HANDLES,
            "hide_all" => Self::HIDE_ALL,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
