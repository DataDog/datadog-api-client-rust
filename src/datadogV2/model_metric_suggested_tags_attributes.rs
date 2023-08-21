// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSuggestedTagsAttributes {
    /// List of aggregation combinations that have been actively queried.
    #[serde(rename = "active_aggregations", skip_serializing_if = "Option::is_none")]
    pub active_aggregations: Vec<MetricCustomAggregation>,
    /// List of tag keys that have been actively queried.
    #[serde(rename = "active_tags", skip_serializing_if = "Option::is_none")]
    pub active_tags: Vec<String>,
}

