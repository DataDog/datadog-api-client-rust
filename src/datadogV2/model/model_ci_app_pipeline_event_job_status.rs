// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CIAppPipelineEventJobStatus {
    #[serde(rename = "success")]
    SUCCESS,
    #[serde(rename = "error")]
    ERROR,
    #[serde(rename = "canceled")]
    CANCELED,
    #[serde(rename = "skipped")]
    SKIPPED,
}
impl ToString for CIAppPipelineEventJobStatus {
    fn to_string(&self) -> String {
        match self {
            Self::SUCCESS => String::from("success"),
            Self::ERROR => String::from("error"),
            Self::CANCELED => String::from("canceled"),
            Self::SKIPPED => String::from("skipped"),
        }
    }
}
