// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RetentionFilterType {
    #[serde(rename = "spans-sampling-processor")]
    SPANS_SAMPLING_PROCESSOR,
}

impl ToString for RetentionFilterType {
    fn to_string(&self) -> String {
        match self {
            Self::SPANS_SAMPLING_PROCESSOR => String::from("spans-sampling-processor"),
        }
    }
}

impl Default for RetentionFilterType {
    fn default() -> RetentionFilterType {
        Self::SPANS_SAMPLING_PROCESSOR
    }
}
