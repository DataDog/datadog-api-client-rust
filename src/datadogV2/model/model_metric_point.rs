// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A point object is of the form `{POSIX_timestamp, numeric_value}`.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricPoint {
    /// The timestamp should be in seconds and current.
    /// Current is defined as not more than 10 minutes in the future or more than 1 hour in the past.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    /// The numeric value format should be a 64bit float gauge-type value.
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

impl MetricPoint {
    pub fn new() -> MetricPoint {
        MetricPoint {
            timestamp: None,
            value: None,
        }
    }
}
impl Default for MetricPoint {
    fn default() -> Self {
        Self::new()
    }
}
