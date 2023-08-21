// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricCustomAggregation {
    /// A space aggregation for use in query.
    #[serde(rename = "space", skip_serializing_if = "Option::is_none")]
    pub space: MetricCustomSpaceAggregation,
    /// A time aggregation for use in query.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: MetricCustomTimeAggregation,
}

