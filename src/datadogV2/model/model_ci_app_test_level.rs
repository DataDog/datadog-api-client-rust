// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CIAppTestLevel {
    #[serde(rename = "session")]
    SESSION,
    #[serde(rename = "module")]
    MODULE,
    #[serde(rename = "suite")]
    SUITE,
    #[serde(rename = "test")]
    TEST,
}

impl ToString for CIAppTestLevel {
    fn to_string(&self) -> String {
        match self {
            Self::SESSION => String::from("session"),
            Self::MODULE => String::from("module"),
            Self::SUITE => String::from("suite"),
            Self::TEST => String::from("test"),
        }
    }
}
