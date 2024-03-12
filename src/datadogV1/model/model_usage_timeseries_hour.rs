// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The hourly usage of timeseries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeseriesHour {
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Contains the number of custom metrics that are inputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_input_timeseries")]
    pub num_custom_input_timeseries: Option<i64>,
    /// Contains the number of custom metrics that are outputs for aggregations (metric configured is custom).
    #[serde(rename = "num_custom_output_timeseries")]
    pub num_custom_output_timeseries: Option<i64>,
    /// Contains sum of non-aggregation custom metrics and custom metrics that are outputs for aggregations.
    #[serde(rename = "num_custom_timeseries")]
    pub num_custom_timeseries: Option<i64>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageTimeseriesHour {
    pub fn new() -> UsageTimeseriesHour {
        UsageTimeseriesHour {
            hour: None,
            num_custom_input_timeseries: None,
            num_custom_output_timeseries: None,
            num_custom_timeseries: None,
            org_name: None,
            public_id: None,
        }
    }

    pub fn hour(mut self, value: String) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn num_custom_input_timeseries(mut self, value: i64) -> Self {
        self.num_custom_input_timeseries = Some(value);
        self
    }

    pub fn num_custom_output_timeseries(mut self, value: i64) -> Self {
        self.num_custom_output_timeseries = Some(value);
        self
    }

    pub fn num_custom_timeseries(mut self, value: i64) -> Self {
        self.num_custom_timeseries = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageTimeseriesHour {
    fn default() -> Self {
        Self::new()
    }
}
