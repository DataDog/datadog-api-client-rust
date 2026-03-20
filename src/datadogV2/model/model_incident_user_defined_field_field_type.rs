// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentUserDefinedFieldFieldType {
    DROPDOWN,
    MULTISELECT,
    TEXTBOX,
    TEXTARRAY,
    METRICTAG,
    AUTOCOMPLETE,
    NUMBER,
    DATETIME,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl Serialize for IncidentUserDefinedFieldFieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            Self::DROPDOWN => serializer.serialize_i32(1),
            Self::MULTISELECT => serializer.serialize_i32(2),
            Self::TEXTBOX => serializer.serialize_i32(3),
            Self::TEXTARRAY => serializer.serialize_i32(4),
            Self::METRICTAG => serializer.serialize_i32(5),
            Self::AUTOCOMPLETE => serializer.serialize_i32(6),
            Self::NUMBER => serializer.serialize_i32(7),
            Self::DATETIME => serializer.serialize_i32(8),
        }
    }
}

impl<'de> Deserialize<'de> for IncidentUserDefinedFieldFieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: i32 = i32::deserialize(deserializer)?;
        Ok(match s {
            1 => Self::DROPDOWN,
            2 => Self::MULTISELECT,
            3 => Self::TEXTBOX,
            4 => Self::TEXTARRAY,
            5 => Self::METRICTAG,
            6 => Self::AUTOCOMPLETE,
            7 => Self::NUMBER,
            8 => Self::DATETIME,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::Number(s.into()),
            }),
        })
    }
}
