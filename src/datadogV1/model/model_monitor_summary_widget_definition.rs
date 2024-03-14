// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The monitor summary widget displays a summary view of all your Datadog monitors, or a subset based on a query. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn color_preference(
        mut self,
        value: crate::datadogV1::model::WidgetColorPreference,
    ) -> Self {
        self.color_preference = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn display_format(
        mut self,
        value: crate::datadogV1::model::WidgetMonitorSummaryDisplayFormat,
    ) -> Self {
        self.display_format = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn hide_zero_counts(mut self, value: bool) -> Self {
        self.hide_zero_counts = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_last_triggered(mut self, value: bool) -> Self {
        self.show_last_triggered = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_priority(mut self, value: bool) -> Self {
        self.show_priority = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn sort(mut self, value: crate::datadogV1::model::WidgetMonitorSummarySort) -> Self {
        self.sort = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn summary_type(mut self, value: crate::datadogV1::model::WidgetSummaryType) -> Self {
        self.summary_type = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorSummaryWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSummaryWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorSummaryWidgetDefinitionVisitor {
            type Value = MonitorSummaryWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_preference: Option<crate::datadogV1::model::WidgetColorPreference> =
                    None;
                let mut count: Option<i64> = None;
                let mut display_format: Option<
                    crate::datadogV1::model::WidgetMonitorSummaryDisplayFormat,
                > = None;
                let mut hide_zero_counts: Option<bool> = None;
                let mut query: Option<String> = None;
                let mut show_last_triggered: Option<bool> = None;
                let mut show_priority: Option<bool> = None;
                let mut sort: Option<crate::datadogV1::model::WidgetMonitorSummarySort> = None;
                let mut start: Option<i64> = None;
                let mut summary_type: Option<crate::datadogV1::model::WidgetSummaryType> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::MonitorSummaryWidgetDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_preference" => {
                            if v.is_null() {
                                continue;
                            }
                            color_preference =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color_preference) = color_preference {
                                match _color_preference {
                                    crate::datadogV1::model::WidgetColorPreference::UnparsedObject(_color_preference) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_format" => {
                            if v.is_null() {
                                continue;
                            }
                            display_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_format) = display_format {
                                match _display_format {
                                    crate::datadogV1::model::WidgetMonitorSummaryDisplayFormat::UnparsedObject(_display_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "hide_zero_counts" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_zero_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_last_triggered" => {
                            if v.is_null() {
                                continue;
                            }
                            show_last_triggered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_priority" => {
                            if v.is_null() {
                                continue;
                            }
                            show_priority =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sort) = sort {
                                match _sort {
                                    crate::datadogV1::model::WidgetMonitorSummarySort::UnparsedObject(_sort) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "summary_type" => {
                            if v.is_null() {
                                continue;
                            }
                            summary_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _summary_type) = summary_type {
                                match _summary_type {
                                    crate::datadogV1::model::WidgetSummaryType::UnparsedObject(
                                        _summary_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _title_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::MonitorSummaryWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                #[allow(deprecated)]
                let content = MonitorSummaryWidgetDefinition {
                    color_preference,
                    count,
                    display_format,
                    hide_zero_counts,
                    query,
                    show_last_triggered,
                    show_priority,
                    sort,
                    start,
                    summary_type,
                    title,
                    title_align,
                    title_size,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSummaryWidgetDefinitionVisitor)
    }
}
