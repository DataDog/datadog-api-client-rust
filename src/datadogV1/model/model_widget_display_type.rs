// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetDisplayType {
    #[serde(rename = "area")]
    AREA,
    #[serde(rename = "bars")]
    BARS,
    #[serde(rename = "line")]
    LINE,
    #[serde(rename = "overlay")]
    OVERLAY,
}
impl ToString for WidgetDisplayType {
    fn to_string(&self) -> String {
        match self {
            Self::AREA => String::from("area"),
            Self::BARS => String::from("bars"),
            Self::LINE => String::from("line"),
            Self::OVERLAY => String::from("overlay"),
        }
    }
}
