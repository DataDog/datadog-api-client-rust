// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetConditionalFormat {
    /// Comparator to apply.
    #[serde(rename = "comparator", skip_serializing_if = "Option::is_none")]
    pub comparator: WidgetComparator,
    /// Color palette to apply to the background, same values available as palette.
    #[serde(rename = "custom_bg_color", skip_serializing_if = "Option::is_none")]
    pub custom_bg_color: String,
    /// Color palette to apply to the foreground, same values available as palette.
    #[serde(rename = "custom_fg_color", skip_serializing_if = "Option::is_none")]
    pub custom_fg_color: String,
    /// True hides values.
    #[serde(rename = "hide_value", skip_serializing_if = "Option::is_none")]
    pub hide_value: bool,
    /// Displays an image as the background.
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: String,
    /// Metric from the request to correlate this conditional format with.
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: String,
    /// Color palette to apply.
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: WidgetPalette,
    /// Defines the displayed timeframe.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: String,
    /// Value for the comparator.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: f64,
}

