// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBrowserVariableType {
    #[serde(rename = "element")]
    ELEMENT,
    #[serde(rename = "email")]
    EMAIL,
    #[serde(rename = "global")]
    GLOBAL,
    #[serde(rename = "javascript")]
    JAVASCRIPT,
    #[serde(rename = "text")]
    TEXT,
}

impl ToString for SyntheticsBrowserVariableType {
    fn to_string(&self) -> String {
        match self {
            Self::ELEMENT => String::from("element"),
            Self::EMAIL => String::from("email"),
            Self::GLOBAL => String::from("global"),
            Self::JAVASCRIPT => String::from("javascript"),
            Self::TEXT => String::from("text"),
        }
    }
}
