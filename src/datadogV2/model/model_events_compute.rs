// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The instructions for what to compute for this query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCompute {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::EventsAggregation,
    /// Interval for compute in milliseconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// The "measure" attribute on which to perform the computation.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
}

impl EventsCompute {
    pub fn new(aggregation: crate::datadogV2::model::EventsAggregation) -> EventsCompute {
        EventsCompute {
            aggregation,
            interval: None,
            metric: None,
        }
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }
}
