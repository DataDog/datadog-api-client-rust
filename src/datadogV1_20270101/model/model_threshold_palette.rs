// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ThresholdPalette {
	WHITE_ON_RED,
	BLACK_ON_LIGHT_RED,
	WHITE_ON_GREEN,
	BLACK_ON_LIGHT_GREEN,
	WHITE_ON_YELLOW,
	BLACK_ON_LIGHT_YELLOW,
	WHITE_ON_GRAY,
	GREEN_ON_WHITE,
	YELLOW_ON_WHITE,
	RED_ON_WHITE,
	GRAY_ON_WHITE,
	RED,
	GREEN,
	ORANGE,
	GRAY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ThresholdPalette {
    fn to_string(&self) -> String {
        match self {
            Self::WHITE_ON_RED => String::from("white_on_red"),
            Self::BLACK_ON_LIGHT_RED => String::from("black_on_light_red"),
            Self::WHITE_ON_GREEN => String::from("white_on_green"),
            Self::BLACK_ON_LIGHT_GREEN => String::from("black_on_light_green"),
            Self::WHITE_ON_YELLOW => String::from("white_on_yellow"),
            Self::BLACK_ON_LIGHT_YELLOW => String::from("black_on_light_yellow"),
            Self::WHITE_ON_GRAY => String::from("white_on_gray"),
            Self::GREEN_ON_WHITE => String::from("green_on_white"),
            Self::YELLOW_ON_WHITE => String::from("yellow_on_white"),
            Self::RED_ON_WHITE => String::from("red_on_white"),
            Self::GRAY_ON_WHITE => String::from("gray_on_white"),
            Self::RED => String::from("red"),
            Self::GREEN => String::from("green"),
            Self::ORANGE => String::from("orange"),
            Self::GRAY => String::from("gray"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ThresholdPalette {
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

impl<'de> Deserialize<'de> for ThresholdPalette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"white_on_red" => Self::WHITE_ON_RED,"black_on_light_red" => Self::BLACK_ON_LIGHT_RED,"white_on_green" => Self::WHITE_ON_GREEN,"black_on_light_green" => Self::BLACK_ON_LIGHT_GREEN,"white_on_yellow" => Self::WHITE_ON_YELLOW,"black_on_light_yellow" => Self::BLACK_ON_LIGHT_YELLOW,"white_on_gray" => Self::WHITE_ON_GRAY,"green_on_white" => Self::GREEN_ON_WHITE,"yellow_on_white" => Self::YELLOW_ON_WHITE,"red_on_white" => Self::RED_ON_WHITE,"gray_on_white" => Self::GRAY_ON_WHITE,"red" => Self::RED,"green" => Self::GREEN,"orange" => Self::ORANGE,"gray" => Self::GRAY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
