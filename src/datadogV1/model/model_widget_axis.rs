// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Axis controls for the widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetAxis {
    /// Set to `true` to include zero.
    #[serde(rename = "include_zero")]
    pub include_zero: Option<bool>,
    /// The label of the axis to display on the graph. Only usable on Scatterplot Widgets.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Specifies maximum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Specifies minimum numeric value to show on the axis. Defaults to `auto`.
    #[serde(rename = "min")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear`, `log`, `sqrt`, and `pow##` (for example `pow2` or `pow0.5`).
    #[serde(rename = "scale")]
    pub scale: Option<String>,
}

impl WidgetAxis {
    pub fn new() -> WidgetAxis {
        WidgetAxis {
            include_zero: None,
            label: None,
            max: None,
            min: None,
            scale: None,
        }
    }
}
impl Default for WidgetAxis {
    fn default() -> Self {
        Self::new()
    }
}
