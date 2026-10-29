// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPagesSort {
    CREATED_AT_ASCENDING,
    CREATED_AT_DESCENDING,
    MODIFIED_AT_ASCENDING,
    MODIFIED_AT_DESCENDING,
    STATUS_ASCENDING,
    STATUS_DESCENDING,
    PRIORITY_ASCENDING,
    PRIORITY_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPagesSort {
    fn to_string(&self) -> String {
        match self {
            Self::CREATED_AT_ASCENDING => String::from("created_at"),
            Self::CREATED_AT_DESCENDING => String::from("-created_at"),
            Self::MODIFIED_AT_ASCENDING => String::from("modified_at"),
            Self::MODIFIED_AT_DESCENDING => String::from("-modified_at"),
            Self::STATUS_ASCENDING => String::from("status"),
            Self::STATUS_DESCENDING => String::from("-status"),
            Self::PRIORITY_ASCENDING => String::from("priority"),
            Self::PRIORITY_DESCENDING => String::from("-priority"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPagesSort {
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

impl<'de> Deserialize<'de> for OnCallPagesSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created_at" => Self::CREATED_AT_ASCENDING,
            "-created_at" => Self::CREATED_AT_DESCENDING,
            "modified_at" => Self::MODIFIED_AT_ASCENDING,
            "-modified_at" => Self::MODIFIED_AT_DESCENDING,
            "status" => Self::STATUS_ASCENDING,
            "-status" => Self::STATUS_DESCENDING,
            "priority" => Self::PRIORITY_ASCENDING,
            "-priority" => Self::PRIORITY_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
