// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Usage timeseries data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeSeriesObject {
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<i64>>,
}

impl UsageTimeSeriesObject {
    pub fn new() -> UsageTimeSeriesObject {
        UsageTimeSeriesObject {
            timestamp: None,
            value: None,
        }
    }

    pub fn timestamp(&mut self, value: String) -> &mut Self {
        self.timestamp = Some(value);
        self
    }

    pub fn value(&mut self, value: Option<i64>) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for UsageTimeSeriesObject {
    fn default() -> Self {
        Self::new()
    }
}
