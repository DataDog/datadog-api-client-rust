// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAggregateBucketValueTimeseriesPoint {
    /// The time value for this point.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: String,
    /// The value for this point.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: f64,
}

