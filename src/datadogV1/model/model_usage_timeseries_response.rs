// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing hourly usage of timeseries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeseriesResponse {
    /// An array of objects regarding hourly usage of timeseries.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageTimeseriesHour>>,
}

impl UsageTimeseriesResponse {
    pub fn new() -> UsageTimeseriesResponse {
        UsageTimeseriesResponse { usage: None }
    }

    pub fn usage(&mut self, value: Vec<crate::datadogV1::model::UsageTimeseriesHour>) -> &mut Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageTimeseriesResponse {
    fn default() -> Self {
        Self::new()
    }
}
