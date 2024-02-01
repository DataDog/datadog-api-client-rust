// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A metric to submit to Datadog.
/// See [Datadog metrics](<https://docs.datadoghq.com/developers/metrics/#custom-metrics-properties>).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Series {
    /// The name of the host that produced the metric.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// If the type of the metric is rate or count, define the corresponding interval.
    #[serde(
        rename = "interval",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub interval: Option<Option<i64>>,
    /// The name of the timeseries.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Points relating to a metric. All points must be tuples with timestamp and a scalar value (cannot be a string). Timestamps should be in POSIX time in seconds, and cannot be more than ten minutes in the future or more than one hour in the past.
    #[serde(rename = "points")]
    pub points: Vec<Vec<Option<f64>>>,
    /// A list of tags associated with the metric.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the metric. Valid types are "",`count`, `gauge`, and `rate`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl Series {
    pub fn new(metric: String, points: Vec<Vec<Option<f64>>>) -> Series {
        Series {
            host: None,
            interval: None,
            metric,
            points,
            tags: None,
            type_: None,
        }
    }

    pub fn host(&mut self, value: String) -> &mut Self {
        self.host = Some(value);
        self
    }

    pub fn interval(&mut self, value: Option<i64>) -> &mut Self {
        self.interval = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
