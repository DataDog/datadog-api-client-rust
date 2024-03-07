// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ToplistWidgetLegend {
    #[serde(rename = "automatic")]
    AUTOMATIC,
    #[serde(rename = "inline")]
    INLINE,
    #[serde(rename = "none")]
    NONE,
}
impl ToString for ToplistWidgetLegend {
    fn to_string(&self) -> String {
        match self {
            Self::AUTOMATIC => String::from("automatic"),
            Self::INLINE => String::from("inline"),
            Self::NONE => String::from("none"),
        }
    }
}
