// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansAggregateBucketAttributes {
    /// The key, value pairs for each group by.
    #[serde(rename = "by", skip_serializing_if = "Option::is_none")]
    pub by: map[string]interface{},
    /// The compute data.
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: interface{},
    /// A map of the metric name -> value for regular compute or list of values for a timeseries.
    #[serde(rename = "computes", skip_serializing_if = "Option::is_none")]
    pub computes: map[string]SpansAggregateBucketValue,
}

