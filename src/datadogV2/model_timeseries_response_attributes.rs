// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesResponseAttributes {
    /// Array of response series. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Vec<TimeseriesResponseSeries>,
    /// Array of times, 1-1 match with individual values arrays.
    #[serde(rename = "times", skip_serializing_if = "Option::is_none")]
    pub times: Vec<i64>,
    /// Array of value-arrays. The index here corresponds to the index in the `formulas` or `queries` array from the request.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Vec<Vec<*f64>>,
}

