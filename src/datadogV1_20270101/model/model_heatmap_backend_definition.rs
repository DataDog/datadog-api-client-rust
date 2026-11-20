// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeatmapBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "events")]
    pub events: Option<Vec<crate::datadogV1_20270101::model::LegacyEventOverlayQuery>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "legend_columns")]
    pub legend_columns: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetLegendColumn>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "legend_layout")]
    pub legend_layout: Option<crate::datadogV1_20270101::model::HeatmapBackendDefinitionLegendLayout>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1_20270101::model::WidgetMarker>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1_20270101::model::HeatmapBackendRequest>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1_20270101::model::WidgetTime>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::HeatmapBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "xaxis")]
    pub xaxis: Option<crate::datadogV1_20270101::model::HeatmapXAxis>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1_20270101::model::HeatmapBackendDefinitionYaxis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HeatmapBackendDefinition {
    pub fn new(requests: Vec<crate::datadogV1_20270101::model::HeatmapBackendRequest>, type_: crate::datadogV1_20270101::model::HeatmapBackendDefinitionType) -> HeatmapBackendDefinition {
        HeatmapBackendDefinition {
            custom_links: None,
            description: None,
            events: None,
            legend_columns: None,
            legend_layout: None,
            legend_size: None,
            markers: None,
            notes: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            xaxis: None,
            yaxis: None,
            _unparsed: false,
        }
    }
    
    pub fn custom_links(mut self, value: Vec<crate::datadogV1_20270101::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn events(mut self, value: Vec<crate::datadogV1_20270101::model::LegacyEventOverlayQuery>) -> Self {
        self.events = Some(value);
        self
    }
    
    pub fn legend_columns(mut self, value: Vec<crate::datadogV1_20270101::model::TimeseriesWidgetLegendColumn>) -> Self {
        self.legend_columns = Some(value);
        self
    }
    
    pub fn legend_layout(mut self, value: crate::datadogV1_20270101::model::HeatmapBackendDefinitionLegendLayout) -> Self {
        self.legend_layout = Some(value);
        self
    }
    
    pub fn legend_size(mut self, value: String) -> Self {
        self.legend_size = Some(value);
        self
    }
    
    pub fn markers(mut self, value: Vec<crate::datadogV1_20270101::model::WidgetMarker>) -> Self {
        self.markers = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn show_legend(mut self, value: bool) -> Self {
        self.show_legend = Some(value);
        self
    }
    
    pub fn time(mut self, value: crate::datadogV1_20270101::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }
    
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    
    pub fn title_align(mut self, value: crate::datadogV1_20270101::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }
    
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }
    
    pub fn xaxis(mut self, value: crate::datadogV1_20270101::model::HeatmapXAxis) -> Self {
        self.xaxis = Some(value);
        self
    }
    
    pub fn yaxis(mut self, value: crate::datadogV1_20270101::model::HeatmapBackendDefinitionYaxis) -> Self {
        self.yaxis = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HeatmapBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeatmapBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for HeatmapBackendDefinitionVisitor {
            type Value = HeatmapBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>> = None;
                let mut description: Option<String> = None;
                let mut events: Option<Vec<crate::datadogV1_20270101::model::LegacyEventOverlayQuery>> = None;
                let mut legend_columns: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetLegendColumn>> = None;
                let mut legend_layout: Option<crate::datadogV1_20270101::model::HeatmapBackendDefinitionLegendLayout> = None;
                let mut legend_size: Option<String> = None;
                let mut markers: Option<Vec<crate::datadogV1_20270101::model::WidgetMarker>> = None;
                let mut notes: Option<String> = None;
                let mut requests: Option<Vec<crate::datadogV1_20270101::model::HeatmapBackendRequest>> = None;
                let mut show_legend: Option<bool> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::HeatmapBackendDefinitionType> = None;
                let mut xaxis: Option<crate::datadogV1_20270101::model::HeatmapXAxis> = None;
                let mut yaxis: Option<crate::datadogV1_20270101::model::HeatmapBackendDefinitionYaxis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "events" => {
                            if v.is_null() {
                                continue;
                            }
                            events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "legend_columns" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "legend_layout" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_layout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend_layout) = legend_layout {
                                match _legend_layout {
                                    crate::datadogV1_20270101::model::HeatmapBackendDefinitionLegendLayout::UnparsedObject(_legend_layout) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "legend_size" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "markers" => {
                            if v.is_null() {
                                continue;
                            }
                            markers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_legend" => {
                            if v.is_null() {
                                continue;
                            }
                            show_legend = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    crate::datadogV1_20270101::model::WidgetTime::UnparsedObject(_time) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1_20270101::model::WidgetTextAlign::UnparsedObject(_title_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::HeatmapBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "xaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            xaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HeatmapBackendDefinition {
                    custom_links,
                    description,
                    events,
                    legend_columns,
                    legend_layout,
                    legend_size,
                    markers,
                    notes,
                    requests,
                    show_legend,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    xaxis,
                    yaxis,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HeatmapBackendDefinitionVisitor)
    }
}
