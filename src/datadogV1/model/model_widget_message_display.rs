// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetMessageDisplay {
    INLINE,
    EXPANDED_MEDIUM,
    EXPANDED_LARGE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for WidgetMessageDisplay {
    fn to_string(&self) -> String {
        match self {
            Self::INLINE => String::from("inline"),
            Self::EXPANDED_MEDIUM => String::from("expanded-md"),
            Self::EXPANDED_LARGE => String::from("expanded-lg"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetMessageDisplay {
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

impl<'de> Deserialize<'de> for WidgetMessageDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "inline" => Self::INLINE,
            "expanded-md" => Self::EXPANDED_MEDIUM,
            "expanded-lg" => Self::EXPANDED_LARGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
