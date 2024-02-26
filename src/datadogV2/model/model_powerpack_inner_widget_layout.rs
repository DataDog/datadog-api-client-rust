// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack inner widget layout.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackInnerWidgetLayout {
    /// The height of the widget. Should be a non-negative integer.
    #[serde(rename = "height")]
    pub height: i64,
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

impl PowerpackInnerWidgetLayout {
    pub fn new(height: i64, width: i64, x: i64, y: i64) -> PowerpackInnerWidgetLayout {
        PowerpackInnerWidgetLayout {
            height,
            width,
            x,
            y,
        }
    }
}
