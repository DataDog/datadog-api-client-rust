// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentsBanditStatus {
    DRAFT,
    SCHEDULED,
    RUNNING,
    PAUSED,
    STOPPED,
    COMPLETED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ExperimentsBanditStatus {
    fn to_string(&self) -> String {
        match self {
            Self::DRAFT => String::from("DRAFT"),
            Self::SCHEDULED => String::from("SCHEDULED"),
            Self::RUNNING => String::from("RUNNING"),
            Self::PAUSED => String::from("PAUSED"),
            Self::STOPPED => String::from("STOPPED"),
            Self::COMPLETED => String::from("COMPLETED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ExperimentsBanditStatus {
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

impl<'de> Deserialize<'de> for ExperimentsBanditStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DRAFT" => Self::DRAFT,
            "SCHEDULED" => Self::SCHEDULED,
            "RUNNING" => Self::RUNNING,
            "PAUSED" => Self::PAUSED,
            "STOPPED" => Self::STOPPED,
            "COMPLETED" => Self::COMPLETED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
