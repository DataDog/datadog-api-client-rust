// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define computation for a log query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsQueryCompute {
    /// The aggregation method.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// Define a time interval in seconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
}

impl LogsQueryCompute {
    pub fn new(aggregation: String) -> LogsQueryCompute {
        LogsQueryCompute {
            aggregation,
            facet: None,
            interval: None,
        }
    }

    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }
}
