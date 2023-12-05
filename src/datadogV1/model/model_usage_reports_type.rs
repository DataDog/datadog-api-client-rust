// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageReportsType {
    #[serde(rename = "reports")]
    REPORTS,
}

impl ToString for UsageReportsType {
    fn to_string(&self) -> String {
        match self {
            Self::REPORTS => String::from("reports"),
        }
    }
}

impl Default for UsageReportsType {
    fn default() -> UsageReportsType {
        Self::REPORTS
    }
}