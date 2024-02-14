// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogsArchiveState {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "WORKING")]
    WORKING,
    #[serde(rename = "FAILING")]
    FAILING,
    #[serde(rename = "WORKING_AUTH_LEGACY")]
    WORKING_AUTH_LEGACY,
}

impl ToString for LogsArchiveState {
    fn to_string(&self) -> String {
        match self {
            Self::UNKNOWN => String::from("UNKNOWN"),
            Self::WORKING => String::from("WORKING"),
            Self::FAILING => String::from("FAILING"),
            Self::WORKING_AUTH_LEGACY => String::from("WORKING_AUTH_LEGACY"),
        }
    }
}
