// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceTableSortType {
    UPDATED_AT,
    TABLE_NAME,
    STATUS,
    MINUS_UPDATED_AT,
    MINUS_TABLE_NAME,
    MINUS_STATUS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ReferenceTableSortType {
    fn to_string(&self) -> String {
        match self {
            Self::UPDATED_AT => String::from("updated_at"),
            Self::TABLE_NAME => String::from("table_name"),
            Self::STATUS => String::from("status"),
            Self::MINUS_UPDATED_AT => String::from("-updated_at"),
            Self::MINUS_TABLE_NAME => String::from("-table_name"),
            Self::MINUS_STATUS => String::from("-status"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ReferenceTableSortType {
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

impl<'de> Deserialize<'de> for ReferenceTableSortType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "updated_at" => Self::UPDATED_AT,
            "table_name" => Self::TABLE_NAME,
            "status" => Self::STATUS,
            "-updated_at" => Self::MINUS_UPDATED_AT,
            "-table_name" => Self::MINUS_TABLE_NAME,
            "-status" => Self::MINUS_STATUS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
