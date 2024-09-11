// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TableWidgetTextFormatPalette {
    WHITE_ON_RED,
    WHITE_ON_YELLOW,
    WHITE_ON_GREEN,
    BLACK_ON_LIGHT_RED,
    BLACK_ON_LIGHT_YELLOW,
    BLACK_ON_LIGHT_GREEN,
    RED_ON_WHITE,
    YELLOW_ON_WHITE,
    GREEN_ON_WHITE,
    CUSTOM_BG,
    CUSTOM_TEXT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TableWidgetTextFormatPalette {
    fn to_string(&self) -> String {
        match self {
            Self::WHITE_ON_RED => String::from("white_on_red"),
            Self::WHITE_ON_YELLOW => String::from("white_on_yellow"),
            Self::WHITE_ON_GREEN => String::from("white_on_green"),
            Self::BLACK_ON_LIGHT_RED => String::from("black_on_light_red"),
            Self::BLACK_ON_LIGHT_YELLOW => String::from("black_on_light_yellow"),
            Self::BLACK_ON_LIGHT_GREEN => String::from("black_on_light_green"),
            Self::RED_ON_WHITE => String::from("red_on_white"),
            Self::YELLOW_ON_WHITE => String::from("yellow_on_white"),
            Self::GREEN_ON_WHITE => String::from("green_on_white"),
            Self::CUSTOM_BG => String::from("custom_bg"),
            Self::CUSTOM_TEXT => String::from("custom_text"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TableWidgetTextFormatPalette {
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

impl<'de> Deserialize<'de> for TableWidgetTextFormatPalette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "white_on_red" => Self::WHITE_ON_RED,
            "white_on_yellow" => Self::WHITE_ON_YELLOW,
            "white_on_green" => Self::WHITE_ON_GREEN,
            "black_on_light_red" => Self::BLACK_ON_LIGHT_RED,
            "black_on_light_yellow" => Self::BLACK_ON_LIGHT_YELLOW,
            "black_on_light_green" => Self::BLACK_ON_LIGHT_GREEN,
            "red_on_white" => Self::RED_ON_WHITE,
            "yellow_on_white" => Self::YELLOW_ON_WHITE,
            "green_on_white" => Self::GREEN_ON_WHITE,
            "custom_bg" => Self::CUSTOM_BG,
            "custom_text" => Self::CUSTOM_TEXT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
