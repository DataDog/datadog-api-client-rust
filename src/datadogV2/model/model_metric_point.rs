// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricPoint {
    /// The timestamp should be in seconds and current.
    /// Current is defined as not more than 10 minutes in the future or more than 1 hour in the past.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The numeric value format should be a 64bit float gauge-type value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl MetricPoint {
    /// A point object is of the form `{POSIX_timestamp, numeric_value}`.
    pub fn new() -> MetricPoint {
        MetricPoint {
            timestamp: None,
            value: None,
        }
    }
}
