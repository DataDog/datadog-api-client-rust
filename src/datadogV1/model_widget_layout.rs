// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayout {
    /// The height of the widget. Should be a non-negative integer.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: i64,
    /// Whether the widget should be the first one on the second column in high density or not.
**Note**: Only for the **new dashboard layout** and only one widget in the dashboard should have this property set to `true`.
    #[serde(rename = "is_column_break", skip_serializing_if = "Option::is_none")]
    pub is_column_break: bool,
    /// The width of the widget. Should be a non-negative integer.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: i64,
    /// The position of the widget on the x (horizontal) axis. Should be a non-negative integer.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: i64,
    /// The position of the widget on the y (vertical) axis. Should be a non-negative integer.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: i64,
}

