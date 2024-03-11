// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetPalette {
    BLUE,
    CUSTOM_BACKGROUND,
    CUSTOM_IMAGE,
    CUSTOM_TEXT,
    GRAY_ON_WHITE,
    GREY,
    GREEN,
    ORANGE,
    RED,
    RED_ON_WHITE,
    WHITE_ON_GRAY,
    WHITE_ON_GREEN,
    GREEN_ON_WHITE,
    WHITE_ON_RED,
    WHITE_ON_YELLOW,
    YELLOW_ON_WHITE,
    BLACK_ON_LIGHT_YELLOW,
    BLACK_ON_LIGHT_GREEN,
    BLACK_ON_LIGHT_RED,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for WidgetPalette {
    fn to_string(&self) -> String {
        match self {
            Self::BLUE => String::from("blue"),
            Self::CUSTOM_BACKGROUND => String::from("custom_bg"),
            Self::CUSTOM_IMAGE => String::from("custom_image"),
            Self::CUSTOM_TEXT => String::from("custom_text"),
            Self::GRAY_ON_WHITE => String::from("gray_on_white"),
            Self::GREY => String::from("grey"),
            Self::GREEN => String::from("green"),
            Self::ORANGE => String::from("orange"),
            Self::RED => String::from("red"),
            Self::RED_ON_WHITE => String::from("red_on_white"),
            Self::WHITE_ON_GRAY => String::from("white_on_gray"),
            Self::WHITE_ON_GREEN => String::from("white_on_green"),
            Self::GREEN_ON_WHITE => String::from("green_on_white"),
            Self::WHITE_ON_RED => String::from("white_on_red"),
            Self::WHITE_ON_YELLOW => String::from("white_on_yellow"),
            Self::YELLOW_ON_WHITE => String::from("yellow_on_white"),
            Self::BLACK_ON_LIGHT_YELLOW => String::from("black_on_light_yellow"),
            Self::BLACK_ON_LIGHT_GREEN => String::from("black_on_light_green"),
            Self::BLACK_ON_LIGHT_RED => String::from("black_on_light_red"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetPalette {
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

impl<'de> Deserialize<'de> for WidgetPalette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "blue" => Self::BLUE,
            "custom_bg" => Self::CUSTOM_BACKGROUND,
            "custom_image" => Self::CUSTOM_IMAGE,
            "custom_text" => Self::CUSTOM_TEXT,
            "gray_on_white" => Self::GRAY_ON_WHITE,
            "grey" => Self::GREY,
            "green" => Self::GREEN,
            "orange" => Self::ORANGE,
            "red" => Self::RED,
            "red_on_white" => Self::RED_ON_WHITE,
            "white_on_gray" => Self::WHITE_ON_GRAY,
            "white_on_green" => Self::WHITE_ON_GREEN,
            "green_on_white" => Self::GREEN_ON_WHITE,
            "white_on_red" => Self::WHITE_ON_RED,
            "white_on_yellow" => Self::WHITE_ON_YELLOW,
            "yellow_on_white" => Self::YELLOW_ON_WHITE,
            "black_on_light_yellow" => Self::BLACK_ON_LIGHT_YELLOW,
            "black_on_light_green" => Self::BLACK_ON_LIGHT_GREEN,
            "black_on_light_red" => Self::BLACK_ON_LIGHT_RED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
