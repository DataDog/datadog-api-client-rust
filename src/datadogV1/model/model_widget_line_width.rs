// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetLineWidth {
    #[serde(rename = "normal")]
    NORMAL,
    #[serde(rename = "thick")]
    THICK,
    #[serde(rename = "thin")]
    THIN,
}
impl ToString for WidgetLineWidth {
    fn to_string(&self) -> String {
        match self {
            Self::NORMAL => String::from("normal"),
            Self::THICK => String::from("thick"),
            Self::THIN => String::from("thin"),
        }
    }
}
