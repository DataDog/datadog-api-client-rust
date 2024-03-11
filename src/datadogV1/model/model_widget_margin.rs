// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetMargin {
    SM,
    MD,
    LG,
    SMALL,
    LARGE,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for WidgetMargin {
    fn to_string(&self) -> String {
        match self {
            Self::SM => String::from("sm"),
            Self::MD => String::from("md"),
            Self::LG => String::from("lg"),
            Self::SMALL => String::from("small"),
            Self::LARGE => String::from("large"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetMargin {
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

impl<'de> Deserialize<'de> for WidgetMargin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "sm" => Self::SM,
            "md" => Self::MD,
            "lg" => Self::LG,
            "small" => Self::SMALL,
            "large" => Self::LARGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
