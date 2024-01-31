// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Markers allow you to add visual conditional formatting for your graphs.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetMarker {
    /// Combination of:
    ///   - A severity error, warning, ok, or info
    ///   - A line type: dashed, solid, or bold
    /// In this case of a Distribution widget, this can be set to be `x_axis_percentile`.
    ///
    #[serde(rename = "display_type")]
    pub display_type: Option<String>,
    /// Label to display over the marker.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Timestamp for the widget.
    #[serde(rename = "time")]
    pub time: Option<String>,
    /// Value to apply. Can be a single value y = 15 or a range of values 0 < y < 10.
    #[serde(rename = "value")]
    pub value: String,
}

impl WidgetMarker {
    pub fn new(value: String) -> WidgetMarker {
        WidgetMarker {
            display_type: None,
            label: None,
            time: None,
            value,
        }
    }

    pub fn with_display_type(&mut self, value: String) -> &mut Self {
        self.display_type = Some(value);
        self
    }

    pub fn with_label(&mut self, value: String) -> &mut Self {
        self.label = Some(value);
        self
    }

    pub fn with_time(&mut self, value: String) -> &mut Self {
        self.time = Some(value);
        self
    }
}
