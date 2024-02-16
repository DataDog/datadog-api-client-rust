// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotebookGraphSize {
    #[serde(rename = "xs")]
    EXTRA_SMALL,
    #[serde(rename = "s")]
    SMALL,
    #[serde(rename = "m")]
    MEDIUM,
    #[serde(rename = "l")]
    LARGE,
    #[serde(rename = "xl")]
    EXTRA_LARGE,
}

impl ToString for NotebookGraphSize {
    fn to_string(&self) -> String {
        match self {
            Self::EXTRA_SMALL => String::from("xs"),
            Self::SMALL => String::from("s"),
            Self::MEDIUM => String::from("m"),
            Self::LARGE => String::from("l"),
            Self::EXTRA_LARGE => String::from("xl"),
        }
    }
}
