// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RetentionFilterAllType {
    #[serde(rename = "spans-sampling-processor")]
    SPANS_SAMPLING_PROCESSOR,
    #[serde(rename = "spans-errors-sampling-processor")]
    SPANS_ERRORS_SAMPLING_PROCESSOR,
    #[serde(rename = "spans-appsec-sampling-processor")]
    SPANS_APPSEC_SAMPLING_PROCESSOR,
}

impl ToString for RetentionFilterAllType {
    fn to_string(&self) -> String {
        match self {
            Self::SPANS_SAMPLING_PROCESSOR => String::from("spans-sampling-processor"),
            Self::SPANS_ERRORS_SAMPLING_PROCESSOR => {
                String::from("spans-errors-sampling-processor")
            }
            Self::SPANS_APPSEC_SAMPLING_PROCESSOR => {
                String::from("spans-appsec-sampling-processor")
            }
        }
    }
}

impl Default for RetentionFilterAllType {
    fn default() -> RetentionFilterAllType {
        Self::SPANS_SAMPLING_PROCESSOR
    }
}
