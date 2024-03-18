// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The timeseries visualization allows you to display the evolution of one or more metrics, log events, or Indexed Spans over time.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of widget events.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV1::model::WidgetEvent>>,
    /// Columns displayed in the legend.
    #[serde(rename = "legend_columns")]
    pub legend_columns: Option<Vec<crate::datadogV1::model::TimeseriesWidgetLegendColumn>>,
    /// Layout of the legend.
    #[serde(rename = "legend_layout")]
    pub legend_layout: Option<crate::datadogV1::model::TimeseriesWidgetLegendLayout>,
    /// Available legend sizes for a widget. Should be one of "0", "2", "4", "8", "16", or "auto".
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// List of markers.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1::model::WidgetMarker>>,
    /// List of timeseries widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TimeseriesWidgetRequest>,
    /// Axis controls for the widget.
    #[serde(rename = "right_yaxis")]
    pub right_yaxis: Option<crate::datadogV1::model::WidgetAxis>,
    /// (screenboard only) Show the legend for this widget.
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the timeseries widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TimeseriesWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::WidgetAxis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TimeseriesWidgetRequest>,
        type_: crate::datadogV1::model::TimeseriesWidgetDefinitionType,
    ) -> TimeseriesWidgetDefinition {
        TimeseriesWidgetDefinition {
            custom_links: None,
            events: None,
            legend_columns: None,
            legend_layout: None,
            legend_size: None,
            markers: None,
            requests,
            right_yaxis: None,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            yaxis: None,
            _unparsed: false,
        }
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<crate::datadogV1::model::WidgetEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn legend_columns(
        mut self,
        value: Vec<crate::datadogV1::model::TimeseriesWidgetLegendColumn>,
    ) -> Self {
        self.legend_columns = Some(value);
        self
    }

    pub fn legend_layout(
        mut self,
        value: crate::datadogV1::model::TimeseriesWidgetLegendLayout,
    ) -> Self {
        self.legend_layout = Some(value);
        self
    }

    pub fn legend_size(mut self, value: String) -> Self {
        self.legend_size = Some(value);
        self
    }

    pub fn markers(mut self, value: Vec<crate::datadogV1::model::WidgetMarker>) -> Self {
        self.markers = Some(value);
        self
    }

    pub fn right_yaxis(mut self, value: crate::datadogV1::model::WidgetAxis) -> Self {
        self.right_yaxis = Some(value);
        self
    }

    pub fn show_legend(mut self, value: bool) -> Self {
        self.show_legend = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }

    pub fn yaxis(mut self, value: crate::datadogV1::model::WidgetAxis) -> Self {
        self.yaxis = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for TimeseriesWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetDefinitionVisitor {
            type Value = TimeseriesWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut events: Option<Vec<crate::datadogV1::model::WidgetEvent>> = None;
                let mut legend_columns: Option<
                    Vec<crate::datadogV1::model::TimeseriesWidgetLegendColumn>,
                > = None;
                let mut legend_layout: Option<
                    crate::datadogV1::model::TimeseriesWidgetLegendLayout,
                > = None;
                let mut legend_size: Option<String> = None;
                let mut markers: Option<Vec<crate::datadogV1::model::WidgetMarker>> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::TimeseriesWidgetRequest>> =
                    None;
                let mut right_yaxis: Option<crate::datadogV1::model::WidgetAxis> = None;
                let mut show_legend: Option<bool> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::TimeseriesWidgetDefinitionType> =
                    None;
                let mut yaxis: Option<crate::datadogV1::model::WidgetAxis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "legend_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_columns =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "legend_layout" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_layout =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend_layout) = legend_layout {
                                match _legend_layout {
                                    crate::datadogV1::model::TimeseriesWidgetLegendLayout::UnparsedObject(_legend_layout) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "legend_size" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "markers" => {
                            if v.is_null() {
                                continue;
                            }
                            markers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            right_yaxis =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_legend" => {
                            if v.is_null() {
                                continue;
                            }
                            show_legend =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::TimeseriesWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TimeseriesWidgetDefinition {
                    custom_links,
                    events,
                    legend_columns,
                    legend_layout,
                    legend_size,
                    markers,
                    requests,
                    right_yaxis,
                    show_legend,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    yaxis,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetDefinitionVisitor)
    }
}
