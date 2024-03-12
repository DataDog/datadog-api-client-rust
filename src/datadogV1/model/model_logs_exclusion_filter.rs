// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Exclusion filter is defined by a query, a sampling rule, and a active/inactive toggle.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsExclusionFilter {
    /// Default query is `*`, meaning all logs flowing in the index would be excluded.
    /// Scope down exclusion filter to only a subset of logs with a log query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Sample rate to apply to logs going through this exclusion filter,
    /// a value of 1.0 excludes all logs matching the query.
    #[serde(rename = "sample_rate")]
    pub sample_rate: f64,
}

impl LogsExclusionFilter {
    pub fn new(sample_rate: f64) -> LogsExclusionFilter {
        LogsExclusionFilter {
            query: None,
            sample_rate,
        }
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}
