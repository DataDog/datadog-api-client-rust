// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetFormatType {
    #[serde(rename = "auto")]
    AUTO,
    #[serde(rename = "string")]
    STRING,
    #[serde(rename = "integer")]
    INTEGER,
    #[serde(rename = "double")]
    DOUBLE,
}

impl ToString for TargetFormatType {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::STRING => String::from("string"),
            Self::INTEGER => String::from("integer"),
            Self::DOUBLE => String::from("double"),
        }
    }
}
