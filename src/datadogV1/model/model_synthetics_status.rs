// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsStatus {
    #[serde(rename = "passed")]
    PASSED,
    #[serde(rename = "skipped")]
    skipped,
    #[serde(rename = "failed")]
    failed,
}

impl ToString for SyntheticsStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PASSED => String::from("passed"),
            Self::skipped => String::from("skipped"),
            Self::failed => String::from("failed"),
        }
    }
}
