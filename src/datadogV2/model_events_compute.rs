// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCompute {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: EventsAggregation,
    /// Interval for compute in milliseconds.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// The "measure" attribute on which to perform the computation.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
}

