// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDistinctVolumeAttributes {
    /// Distinct volume for the given metric.
    #[serde(rename = "distinct_volume", skip_serializing_if = "Option::is_none")]
    pub distinct_volume: i64,
}

