// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetHorizontalAlign {
    #[serde(rename = "center")]
    CENTER,
    #[serde(rename = "left")]
    LEFT,
    #[serde(rename = "right")]
    RIGHT,
}
impl ToString for WidgetHorizontalAlign {
    fn to_string(&self) -> String {
        match self {
            Self::CENTER => String::from("center"),
            Self::LEFT => String::from("left"),
            Self::RIGHT => String::from("right"),
        }
    }
}
