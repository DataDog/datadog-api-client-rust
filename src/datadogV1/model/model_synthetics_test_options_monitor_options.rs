// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the options for a Synthetic test as a monitor
/// (for example, renotification).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptionsMonitorOptions {
    /// Time interval before renotifying if the test is still failing
    /// (in minutes).
    #[serde(rename = "renotify_interval")]
    pub renotify_interval: Option<i64>,
}

impl SyntheticsTestOptionsMonitorOptions {
    pub fn new() -> SyntheticsTestOptionsMonitorOptions {
        SyntheticsTestOptionsMonitorOptions {
            renotify_interval: None,
        }
    }

    pub fn with_renotify_interval(&mut self, value: i64) -> &mut Self {
        self.renotify_interval = Some(value);
        self
    }
}
impl Default for SyntheticsTestOptionsMonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}
