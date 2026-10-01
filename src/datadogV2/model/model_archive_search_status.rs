// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArchiveSearchStatus {
    RUNNING,
    COMPLETED,
    FAILED,
    CANCELLED,
    QUOTA_REACHED,
    EXPIRED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ArchiveSearchStatus {
    fn to_string(&self) -> String {
        match self {
            Self::RUNNING => String::from("RUNNING"),
            Self::COMPLETED => String::from("COMPLETED"),
            Self::FAILED => String::from("FAILED"),
            Self::CANCELLED => String::from("CANCELLED"),
            Self::QUOTA_REACHED => String::from("QUOTA_REACHED"),
            Self::EXPIRED => String::from("EXPIRED"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ArchiveSearchStatus {
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

impl<'de> Deserialize<'de> for ArchiveSearchStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "RUNNING" => Self::RUNNING,
            "COMPLETED" => Self::COMPLETED,
            "FAILED" => Self::FAILED,
            "CANCELLED" => Self::CANCELLED,
            "QUOTA_REACHED" => Self::QUOTA_REACHED,
            "EXPIRED" => Self::EXPIRED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
