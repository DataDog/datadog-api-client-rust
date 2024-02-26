// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsPatchTestOperationName {
    #[serde(rename = "add")]
    ADD,
    #[serde(rename = "remove")]
    REMOVE,
    #[serde(rename = "replace")]
    REPLACE,
    #[serde(rename = "move")]
    MOVE,
    #[serde(rename = "copy")]
    COPY,
    #[serde(rename = "test")]
    TEST,
}

impl ToString for SyntheticsPatchTestOperationName {
    fn to_string(&self) -> String {
        match self {
            Self::ADD => String::from("add"),
            Self::REMOVE => String::from("remove"),
            Self::REPLACE => String::from("replace"),
            Self::MOVE => String::from("move"),
            Self::COPY => String::from("copy"),
            Self::TEST => String::from("test"),
        }
    }
}
