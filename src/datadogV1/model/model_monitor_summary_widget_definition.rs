// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The monitor summary widget displays a summary view of all your Datadog monitors, or a subset based on a query. Only available on FREE layout dashboards.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSummaryWidgetDefinition {
    /// Which color to use on the widget.
    #[serde(rename = "color_preference")]
    pub color_preference: Option<crate::datadogV1::model::WidgetColorPreference>,
    /// The number of monitors to display.
    #[deprecated]
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// What to display on the widget.
    #[serde(rename = "display_format")]
    pub display_format: Option<crate::datadogV1::model::WidgetMonitorSummaryDisplayFormat>,
    /// Whether to show counts of 0 or not.
    #[serde(rename = "hide_zero_counts")]
    pub hide_zero_counts: Option<bool>,
    /// Query to filter the monitors with.
    #[serde(rename = "query")]
    pub query: String,
    /// Whether to show the time that has elapsed since the monitor/group triggered.
    #[serde(rename = "show_last_triggered")]
    pub show_last_triggered: Option<bool>,
    /// Whether to show the priorities column.
    #[serde(rename = "show_priority")]
    pub show_priority: Option<bool>,
    /// Widget sorting methods.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::WidgetMonitorSummarySort>,
    /// The start of the list. Typically 0.
    #[deprecated]
    #[serde(rename = "start")]
    pub start: Option<i64>,
    /// Which summary type should be used.
    #[serde(rename = "summary_type")]
    pub summary_type: Option<crate::datadogV1::model::WidgetSummaryType>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the monitor summary widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::MonitorSummaryWidgetDefinitionType,
}

impl MonitorSummaryWidgetDefinition {
    pub fn new(
        query: String,
        type_: crate::datadogV1::model::MonitorSummaryWidgetDefinitionType,
    ) -> MonitorSummaryWidgetDefinition {
        #[allow(deprecated)]
        MonitorSummaryWidgetDefinition {
            color_preference: None,
            count: None,
            display_format: None,
            hide_zero_counts: None,
            query,
            show_last_triggered: None,
            show_priority: None,
            sort: None,
            start: None,
            summary_type: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
        }
    }

    #[allow(deprecated)]
    pub fn color_preference(
        &mut self,
        value: crate::datadogV1::model::WidgetColorPreference,
    ) -> &mut Self {
        self.color_preference = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn count(&mut self, value: i64) -> &mut Self {
        self.count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn display_format(
        &mut self,
        value: crate::datadogV1::model::WidgetMonitorSummaryDisplayFormat,
    ) -> &mut Self {
        self.display_format = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn hide_zero_counts(&mut self, value: bool) -> &mut Self {
        self.hide_zero_counts = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_last_triggered(&mut self, value: bool) -> &mut Self {
        self.show_last_triggered = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_priority(&mut self, value: bool) -> &mut Self {
        self.show_priority = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sort(&mut self, value: crate::datadogV1::model::WidgetMonitorSummarySort) -> &mut Self {
        self.sort = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn summary_type(&mut self, value: crate::datadogV1::model::WidgetSummaryType) -> &mut Self {
        self.summary_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(&mut self, value: String) -> &mut Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_align(&mut self, value: crate::datadogV1::model::WidgetTextAlign) -> &mut Self {
        self.title_align = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_size(&mut self, value: String) -> &mut Self {
        self.title_size = Some(value);
        self
    }
}
