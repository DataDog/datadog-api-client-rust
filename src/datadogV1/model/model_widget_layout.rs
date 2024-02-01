// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The layout for a widget on a `free` or **new dashboard layout** dashboard.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetLayout {
    /// The height of the widget. Should be a non-negative integer.
    #[serde(rename = "height")]
    pub height: i64,
    /// Whether the widget should be the first one on the second column in high density or not.
    /// **Note**: Only for the **new dashboard layout** and only one widget in the dashboard should have this property set to `true`.
    #[serde(rename = "is_column_break")]
    pub is_column_break: Option<bool>,
    /// The width of the widget. Should be a non-negative integer.
    #[serde(rename = "width")]
    pub width: i64,
    /// The position of the widget on the x (horizontal) axis. Should be a non-negative integer.
    #[serde(rename = "x")]
    pub x: i64,
    /// The position of the widget on the y (vertical) axis. Should be a non-negative integer.
    #[serde(rename = "y")]
    pub y: i64,
}

impl WidgetLayout {
    pub fn new(height: i64, width: i64, x: i64, y: i64) -> WidgetLayout {
        WidgetLayout {
            height,
            is_column_break: None,
            width,
            x,
            y,
        }
    }

    pub fn is_column_break(&mut self, value: bool) -> &mut Self {
        self.is_column_break = Some(value);
        self
    }
}
