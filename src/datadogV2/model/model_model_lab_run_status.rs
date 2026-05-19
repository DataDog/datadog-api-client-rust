// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelLabRunStatus {
    PENDING,
    RUNNING,
    COMPLETED,
    FAILED,
    KILLED,
    UNRESPONSIVE,
    PAUSED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ModelLabRunStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING => String::from("pending"),
            Self::RUNNING => String::from("running"),
            Self::COMPLETED => String::from("completed"),
            Self::FAILED => String::from("failed"),
            Self::KILLED => String::from("killed"),
            Self::UNRESPONSIVE => String::from("unresponsive"),
            Self::PAUSED => String::from("paused"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ModelLabRunStatus {
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

impl<'de> Deserialize<'de> for ModelLabRunStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pending" => Self::PENDING,
            "running" => Self::RUNNING,
            "completed" => Self::COMPLETED,
            "failed" => Self::FAILED,
            "killed" => Self::KILLED,
            "unresponsive" => Self::UNRESPONSIVE,
            "paused" => Self::PAUSED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
