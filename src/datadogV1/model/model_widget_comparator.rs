// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetComparator {
    EQUAL_TO,
    GREATER_THAN,
    GREATER_THAN_OR_EQUAL_TO,
    LESS_THAN,
    LESS_THAN_OR_EQUAL_TO,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for WidgetComparator {
    fn to_string(&self) -> String {
        match self {
            Self::EQUAL_TO => String::from("="),
            Self::GREATER_THAN => String::from(">"),
            Self::GREATER_THAN_OR_EQUAL_TO => String::from(">="),
            Self::LESS_THAN => String::from("<"),
            Self::LESS_THAN_OR_EQUAL_TO => String::from("<="),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetComparator {
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

impl<'de> Deserialize<'de> for WidgetComparator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "=" => Self::EQUAL_TO,
            ">" => Self::GREATER_THAN,
            ">=" => Self::GREATER_THAN_OR_EQUAL_TO,
            "<" => Self::LESS_THAN,
            "<=" => Self::LESS_THAN_OR_EQUAL_TO,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
