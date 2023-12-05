// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DetailedFindingType {
    #[serde(rename = "detailed_finding")]
    DETAILED_FINDING,
}

impl ToString for DetailedFindingType {
    fn to_string(&self) -> String {
        match self {
            Self::DETAILED_FINDING => String::from("detailed_finding"),
        }
    }
}

impl Default for DetailedFindingType {
    fn default() -> DetailedFindingType {
        Self::DETAILED_FINDING
    }
}