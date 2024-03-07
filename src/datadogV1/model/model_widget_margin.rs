// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetMargin {
    #[serde(rename = "sm")]
    SM,
    #[serde(rename = "md")]
    MD,
    #[serde(rename = "lg")]
    LG,
    #[serde(rename = "small")]
    SMALL,
    #[serde(rename = "large")]
    LARGE,
}
impl ToString for WidgetMargin {
    fn to_string(&self) -> String {
        match self {
            Self::SM => String::from("sm"),
            Self::MD => String::from("md"),
            Self::LG => String::from("lg"),
            Self::SMALL => String::from("small"),
            Self::LARGE => String::from("large"),
        }
    }
}
