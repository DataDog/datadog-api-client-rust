// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetViewMode {
    #[serde(rename = "overall")]
    OVERALL,
    #[serde(rename = "component")]
    COMPONENT,
    #[serde(rename = "both")]
    BOTH,
}
impl ToString for WidgetViewMode {
    fn to_string(&self) -> String {
        match self {
            Self::OVERALL => String::from("overall"),
            Self::COMPONENT => String::from("component"),
            Self::BOTH => String::from("both"),
        }
    }
}
