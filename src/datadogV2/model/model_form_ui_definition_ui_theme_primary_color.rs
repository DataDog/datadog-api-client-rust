// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormUiDefinitionUiThemePrimaryColor {
    GRAY,
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    LIGHT_BLUE,
    DARK_BLUE,
    MAGENTA,
    INDIGO,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FormUiDefinitionUiThemePrimaryColor {
    fn to_string(&self) -> String {
        match self {
            Self::GRAY => String::from("gray"),
            Self::RED => String::from("red"),
            Self::ORANGE => String::from("orange"),
            Self::YELLOW => String::from("yellow"),
            Self::GREEN => String::from("green"),
            Self::LIGHT_BLUE => String::from("light-blue"),
            Self::DARK_BLUE => String::from("dark-blue"),
            Self::MAGENTA => String::from("magenta"),
            Self::INDIGO => String::from("indigo"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormUiDefinitionUiThemePrimaryColor {
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

impl<'de> Deserialize<'de> for FormUiDefinitionUiThemePrimaryColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "gray" => Self::GRAY,
            "red" => Self::RED,
            "orange" => Self::ORANGE,
            "yellow" => Self::YELLOW,
            "green" => Self::GREEN,
            "light-blue" => Self::LIGHT_BLUE,
            "dark-blue" => Self::DARK_BLUE,
            "magenta" => Self::MAGENTA,
            "indigo" => Self::INDIGO,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
