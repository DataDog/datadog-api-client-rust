// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetRequestStyle {
    /// Type of lines displayed.
    #[serde(rename = "line_type", skip_serializing_if = "Option::is_none")]
    pub line_type: WidgetLineType,
    /// Width of line displayed.
    #[serde(rename = "line_width", skip_serializing_if = "Option::is_none")]
    pub line_width: WidgetLineWidth,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: String,
}

