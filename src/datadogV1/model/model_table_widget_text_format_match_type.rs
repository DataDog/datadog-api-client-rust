// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TableWidgetTextFormatMatchType {
    IS,
    IS_NOT,
    CONTAINS,
    DOES_NOT_CONTAIN,
    STARTS_WITH,
    ENDS_WITH,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TableWidgetTextFormatMatchType {
    fn to_string(&self) -> String {
        match self {
            Self::IS => String::from("is"),
            Self::IS_NOT => String::from("is_not"),
            Self::CONTAINS => String::from("contains"),
            Self::DOES_NOT_CONTAIN => String::from("does_not_contain"),
            Self::STARTS_WITH => String::from("starts_with"),
            Self::ENDS_WITH => String::from("ends_with"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TableWidgetTextFormatMatchType {
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

impl<'de> Deserialize<'de> for TableWidgetTextFormatMatchType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "is" => Self::IS,
            "is_not" => Self::IS_NOT,
            "contains" => Self::CONTAINS,
            "does_not_contain" => Self::DOES_NOT_CONTAIN,
            "starts_with" => Self::STARTS_WITH,
            "ends_with" => Self::ENDS_WITH,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
