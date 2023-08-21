// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricUpdateCompute {
    /// Toggle to include or exclude percentile aggregations for distribution metrics.
Only present when the `aggregation_type` is `distribution`.
    #[serde(rename = "include_percentiles", skip_serializing_if = "Option::is_none")]
    pub include_percentiles: bool,
}

