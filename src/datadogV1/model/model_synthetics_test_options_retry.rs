// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the retry strategy to apply to a Synthetic test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsRetry {
    /// Number of times a test needs to be retried before marking a
    /// location as failed. Defaults to 0.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Time interval between retries (in milliseconds). Defaults to
    /// 300ms.
    #[serde(rename = "interval")]
    pub interval: Option<f64>,
}

impl SyntheticsTestOptionsRetry {
    pub fn new() -> SyntheticsTestOptionsRetry {
        SyntheticsTestOptionsRetry {
            count: None,
            interval: None,
        }
    }
}
impl Default for SyntheticsTestOptionsRetry {
    fn default() -> Self {
        Self::new()
    }
}
