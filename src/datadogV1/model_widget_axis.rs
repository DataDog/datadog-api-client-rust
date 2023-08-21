// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetAxis {
    /// Set to `true` to include zero.
    #[serde(rename = "include_zero", skip_serializing_if = "Option::is_none")]
    pub include_zero: bool,
    /// The label of the axis to display on the graph. Only usable on Scatterplot Widgets.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: String,
    /// Specifies maximum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: String,
    /// Specifies minimum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: String,
    /// Specifies the scale type. Possible values are `linear`, `log`, `sqrt`, and `pow##` (for example `pow2` or `pow0.5`).
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: String,
}

