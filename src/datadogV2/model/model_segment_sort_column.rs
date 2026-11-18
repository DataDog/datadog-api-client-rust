// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SegmentSortColumn {
    NAME,
    DESCRIPTION,
    CREATED_BY,
    CREATED_AT,
    MODIFIED_BY,
    MODIFIED_AT,
    DISABLED_BY,
    DISABLED_AT,
    UUID,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SegmentSortColumn {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::DESCRIPTION => String::from("description"),
            Self::CREATED_BY => String::from("created_by"),
            Self::CREATED_AT => String::from("created_at"),
            Self::MODIFIED_BY => String::from("modified_by"),
            Self::MODIFIED_AT => String::from("modified_at"),
            Self::DISABLED_BY => String::from("disabled_by"),
            Self::DISABLED_AT => String::from("disabled_at"),
            Self::UUID => String::from("uuid"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SegmentSortColumn {
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

impl<'de> Deserialize<'de> for SegmentSortColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "description" => Self::DESCRIPTION,
            "created_by" => Self::CREATED_BY,
            "created_at" => Self::CREATED_AT,
            "modified_by" => Self::MODIFIED_BY,
            "modified_at" => Self::MODIFIED_AT,
            "disabled_by" => Self::DISABLED_BY,
            "disabled_at" => Self::DISABLED_AT,
            "uuid" => Self::UUID,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
