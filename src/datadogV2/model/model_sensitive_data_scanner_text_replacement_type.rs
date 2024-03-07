// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SensitiveDataScannerTextReplacementType {
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "hash")]
    HASH,
    #[serde(rename = "replacement_string")]
    REPLACEMENT_STRING,
    #[serde(rename = "partial_replacement_from_beginning")]
    PARTIAL_REPLACEMENT_FROM_BEGINNING,
    #[serde(rename = "partial_replacement_from_end")]
    PARTIAL_REPLACEMENT_FROM_END,
}

impl ToString for SensitiveDataScannerTextReplacementType {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::HASH => String::from("hash"),
            Self::REPLACEMENT_STRING => String::from("replacement_string"),
            Self::PARTIAL_REPLACEMENT_FROM_BEGINNING => {
                String::from("partial_replacement_from_beginning")
            }
            Self::PARTIAL_REPLACEMENT_FROM_END => String::from("partial_replacement_from_end"),
        }
    }
}
