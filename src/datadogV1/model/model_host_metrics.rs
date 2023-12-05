// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Host Metrics collected.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostMetrics {
    /// The percent of CPU used (everything but idle).
    #[serde(rename = "cpu")]
    pub cpu: Option<f64>,
    /// The percent of CPU spent waiting on the IO (not reported for all platforms).
    #[serde(rename = "iowait")]
    pub iowait: Option<f64>,
    /// The system load over the last 15 minutes.
    #[serde(rename = "load")]
    pub load: Option<f64>,
}

impl HostMetrics {
    pub fn new() -> HostMetrics {
        HostMetrics {
            cpu: None,
            iowait: None,
            load: None,
        }
    }
}