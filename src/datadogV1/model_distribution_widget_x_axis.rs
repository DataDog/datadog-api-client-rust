// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetXAxis {
    /// True includes zero.
    #[serde(rename = "include_zero", skip_serializing_if = "Option::is_none")]
    pub include_zero: bool,
    /// Specifies maximum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: String,
    /// Specifies minimum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: String,
    /// Specifies the scale type. Possible values are `linear`.
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: String,
}

