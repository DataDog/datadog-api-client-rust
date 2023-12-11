// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignalTriageState {
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "archived")]
    ARCHIVED,
    #[serde(rename = "under_review")]
    UNDER_REVIEW,
}

impl ToString for SignalTriageState {
    fn to_string(&self) -> String {
        match self {
            Self::OPEN => String::from("open"),
            Self::ARCHIVED => String::from("archived"),
            Self::UNDER_REVIEW => String::from("under_review"),
        }
    }
}

impl Default for SignalTriageState {
    fn default() -> SignalTriageState {
        Self::OPEN
    }
}
