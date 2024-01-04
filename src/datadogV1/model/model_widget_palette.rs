// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetPalette {
    #[serde(rename = "blue")]
    BLUE,
    #[serde(rename = "custom_bg")]
    CUSTOM_BACKGROUND,
    #[serde(rename = "custom_image")]
    CUSTOM_IMAGE,
    #[serde(rename = "custom_text")]
    CUSTOM_TEXT,
    #[serde(rename = "gray_on_white")]
    GRAY_ON_WHITE,
    #[serde(rename = "grey")]
    GREY,
    #[serde(rename = "green")]
    GREEN,
    #[serde(rename = "orange")]
    ORANGE,
    #[serde(rename = "red")]
    RED,
    #[serde(rename = "red_on_white")]
    RED_ON_WHITE,
    #[serde(rename = "white_on_gray")]
    WHITE_ON_GRAY,
    #[serde(rename = "white_on_green")]
    WHITE_ON_GREEN,
    #[serde(rename = "green_on_white")]
    GREEN_ON_WHITE,
    #[serde(rename = "white_on_red")]
    WHITE_ON_RED,
    #[serde(rename = "white_on_yellow")]
    WHITE_ON_YELLOW,
    #[serde(rename = "yellow_on_white")]
    YELLOW_ON_WHITE,
    #[serde(rename = "black_on_light_yellow")]
    BLACK_ON_LIGHT_YELLOW,
    #[serde(rename = "black_on_light_green")]
    BLACK_ON_LIGHT_GREEN,
    #[serde(rename = "black_on_light_red")]
    BLACK_ON_LIGHT_RED,
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
        }
    }
}
