// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SLOState {
    #[serde(rename = "breached")]
    BREACHED,
    #[serde(rename = "warning")]
    WARNING,
    #[serde(rename = "ok")]
    OK,
    #[serde(rename = "no_data")]
    NO_DATA,
}

impl ToString for SLOState {
    fn to_string(&self) -> String {
        match self {
            Self::BREACHED => String::from("breached"),
            Self::WARNING => String::from("warning"),
            Self::OK => String::from("ok"),
            Self::NO_DATA => String::from("no_data"),
        }
    }
}
