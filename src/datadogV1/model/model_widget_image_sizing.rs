// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetImageSizing {
    #[serde(rename = "fill")]
    FILL,
    #[serde(rename = "contain")]
    CONTAIN,
    #[serde(rename = "cover")]
    COVER,
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "scale-down")]
    SCALEDOWN,
    #[serde(rename = "zoom")]
    ZOOM,
    #[serde(rename = "fit")]
    FIT,
    #[serde(rename = "center")]
    CENTER,
}

impl ToString for WidgetImageSizing {
    fn to_string(&self) -> String {
        match self {
            Self::FILL => String::from("fill"),
            Self::CONTAIN => String::from("contain"),
            Self::COVER => String::from("cover"),
            Self::NONE => String::from("none"),
            Self::SCALEDOWN => String::from("scale-down"),
            Self::ZOOM => String::from("zoom"),
            Self::FIT => String::from("fit"),
            Self::CENTER => String::from("center"),
        }
    }
}
