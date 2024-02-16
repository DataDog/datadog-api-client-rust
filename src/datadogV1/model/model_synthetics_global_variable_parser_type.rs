// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsGlobalVariableParserType {
    #[serde(rename = "raw")]
    RAW,
    #[serde(rename = "json_path")]
    JSON_PATH,
    #[serde(rename = "regex")]
    REGEX,
    #[serde(rename = "x_path")]
    X_PATH,
}

impl ToString for SyntheticsGlobalVariableParserType {
    fn to_string(&self) -> String {
        match self {
            Self::RAW => String::from("raw"),
            Self::JSON_PATH => String::from("json_path"),
            Self::REGEX => String::from("regex"),
            Self::X_PATH => String::from("x_path"),
        }
    }
}
