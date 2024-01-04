// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsTestProcessStatus {
    #[serde(rename = "not_scheduled")]
    NOT_SCHEDULED,
    #[serde(rename = "scheduled")]
    SCHEDULED,
    #[serde(rename = "finished")]
    FINISHED,
    #[serde(rename = "finished_with_error")]
    FINISHED_WITH_ERROR,
}

impl ToString for SyntheticsTestProcessStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NOT_SCHEDULED => String::from("not_scheduled"),
            Self::SCHEDULED => String::from("scheduled"),
            Self::FINISHED => String::from("finished"),
            Self::FINISHED_WITH_ERROR => String::from("finished_with_error"),
        }
    }
}
