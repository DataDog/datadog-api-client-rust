// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonitorOverallStates {
    ALERT,
    IGNORED,
    NO_DATA,
    OK,
    SKIPPED,
    UNKNOWN,
    WARN,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for MonitorOverallStates {
    fn to_string(&self) -> String {
        match self {
            Self::ALERT => String::from("Alert"),
            Self::IGNORED => String::from("Ignored"),
            Self::NO_DATA => String::from("No Data"),
            Self::OK => String::from("OK"),
            Self::SKIPPED => String::from("Skipped"),
            Self::UNKNOWN => String::from("Unknown"),
            Self::WARN => String::from("Warn"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonitorOverallStates {
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

impl<'de> Deserialize<'de> for MonitorOverallStates {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Alert" => Self::ALERT,
            "Ignored" => Self::IGNORED,
            "No Data" => Self::NO_DATA,
            "OK" => Self::OK,
            "Skipped" => Self::SKIPPED,
            "Unknown" => Self::UNKNOWN,
            "Warn" => Self::WARN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
