// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetTickEdge {
    #[serde(rename = "bottom")]
    BOTTOM,
    #[serde(rename = "left")]
    LEFT,
    #[serde(rename = "right")]
    RIGHT,
    #[serde(rename = "top")]
    TOP,
}

impl ToString for WidgetTickEdge {
    fn to_string(&self) -> String {
        match self {
            Self::BOTTOM => String::from("bottom"),
            Self::LEFT => String::from("left"),
            Self::RIGHT => String::from("right"),
            Self::TOP => String::from("top"),
        }
    }
}
