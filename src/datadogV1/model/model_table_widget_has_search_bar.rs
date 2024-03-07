// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TableWidgetHasSearchBar {
    #[serde(rename = "always")]
    ALWAYS,
    #[serde(rename = "never")]
    NEVER,
    #[serde(rename = "auto")]
    AUTO,
}
impl ToString for TableWidgetHasSearchBar {
    fn to_string(&self) -> String {
        match self {
            Self::ALWAYS => String::from("always"),
            Self::NEVER => String::from("never"),
            Self::AUTO => String::from("auto"),
        }
    }
}
