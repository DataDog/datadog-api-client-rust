// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCompute {
    /// An aggregation function.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: CIAppAggregationFunction,
    /// The time buckets' size (only used for type=timeseries)
Defaults to a resolution of 150 points.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: String,
    /// The metric to use.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// The type of compute.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: CIAppComputeType,
}

