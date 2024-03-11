// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentFieldAttributesValueType {
    MULTISELECT,
    TEXTARRAY,
    METRICTAG,
    AUTOCOMPLETE,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for IncidentFieldAttributesValueType {
    fn to_string(&self) -> String {
        match self {
            Self::MULTISELECT => String::from("multiselect"),
            Self::TEXTARRAY => String::from("textarray"),
            Self::METRICTAG => String::from("metrictag"),
            Self::AUTOCOMPLETE => String::from("autocomplete"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentFieldAttributesValueType {
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

impl<'de> Deserialize<'de> for IncidentFieldAttributesValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "multiselect" => Self::MULTISELECT,
            "textarray" => Self::TEXTARRAY,
            "metrictag" => Self::METRICTAG,
            "autocomplete" => Self::AUTOCOMPLETE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
