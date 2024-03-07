// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetLineType {
    #[serde(rename = "dashed")]
    DASHED,
    #[serde(rename = "dotted")]
    DOTTED,
    #[serde(rename = "solid")]
    SOLID,
}

impl ToString for WidgetLineType {
    fn to_string(&self) -> String {
        match self {
            Self::DASHED => String::from("dashed"),
            Self::DOTTED => String::from("dotted"),
            Self::SOLID => String::from("solid"),
        }
    }
}
