// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansGroupByHistogram {
    /// The bin size of the histogram buckets.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: f64,
    /// The maximum value for the measure used in the histogram
(values greater than this one are filtered out).
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: f64,
    /// The minimum value for the measure used in the histogram
(values smaller than this one are filtered out).
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: f64,
}

