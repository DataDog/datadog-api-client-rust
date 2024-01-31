// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define a conditional format for the widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetConditionalFormat {
    /// Comparator to apply.
    #[serde(rename = "comparator")]
    pub comparator: crate::datadogV1::model::WidgetComparator,
    /// Color palette to apply to the background, same values available as palette.
    #[serde(rename = "custom_bg_color")]
    pub custom_bg_color: Option<String>,
    /// Color palette to apply to the foreground, same values available as palette.
    #[serde(rename = "custom_fg_color")]
    pub custom_fg_color: Option<String>,
    /// True hides values.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Displays an image as the background.
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    /// Metric from the request to correlate this conditional format with.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Color palette to apply.
    #[serde(rename = "palette")]
    pub palette: crate::datadogV1::model::WidgetPalette,
    /// Defines the displayed timeframe.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<String>,
    /// Value for the comparator.
    #[serde(rename = "value")]
    pub value: f64,
}

impl WidgetConditionalFormat {
    pub fn new(
        comparator: crate::datadogV1::model::WidgetComparator,
        palette: crate::datadogV1::model::WidgetPalette,
        value: f64,
    ) -> WidgetConditionalFormat {
        WidgetConditionalFormat {
            comparator,
            custom_bg_color: None,
            custom_fg_color: None,
            hide_value: None,
            image_url: None,
            metric: None,
            palette,
            timeframe: None,
            value,
        }
    }

    pub fn with_custom_bg_color(&mut self, value: String) -> &mut Self {
        self.custom_bg_color = Some(value);
        self
    }

    pub fn with_custom_fg_color(&mut self, value: String) -> &mut Self {
        self.custom_fg_color = Some(value);
        self
    }

    pub fn with_hide_value(&mut self, value: bool) -> &mut Self {
        self.hide_value = Some(value);
        self
    }

    pub fn with_image_url(&mut self, value: String) -> &mut Self {
        self.image_url = Some(value);
        self
    }

    pub fn with_metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    pub fn with_timeframe(&mut self, value: String) -> &mut Self {
        self.timeframe = Some(value);
        self
    }
}
