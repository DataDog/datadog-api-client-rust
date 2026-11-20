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
pub struct PointPlotBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "legend")]
    pub legend: Option<crate::datadogV1_20270101::model::ScatterPlotLegendType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1_20270101::model::Marker>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1_20270101::model::PointPlotDataProjectionRequest>,
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
    pub type_: crate::datadogV1_20270101::model::PointPlotBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1_20270101::model::Axis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PointPlotBackendDefinition {
    pub fn new(requests: Vec<crate::datadogV1_20270101::model::PointPlotDataProjectionRequest>, type_: crate::datadogV1_20270101::model::PointPlotBackendDefinitionType) -> PointPlotBackendDefinition {
        PointPlotBackendDefinition {
            description: None,
            legend: None,
            markers: None,
            notes: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            yaxis: None,
            _unparsed: false,
        }
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn legend(mut self, value: crate::datadogV1_20270101::model::ScatterPlotLegendType) -> Self {
        self.legend = Some(value);
        self
    }
    
    pub fn markers(mut self, value: Vec<crate::datadogV1_20270101::model::Marker>) -> Self {
        self.markers = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
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
    
    pub fn yaxis(mut self, value: crate::datadogV1_20270101::model::Axis) -> Self {
        self.yaxis = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for PointPlotBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointPlotBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for PointPlotBackendDefinitionVisitor {
            type Value = PointPlotBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut legend: Option<crate::datadogV1_20270101::model::ScatterPlotLegendType> = None;
                let mut markers: Option<Vec<crate::datadogV1_20270101::model::Marker>> = None;
                let mut notes: Option<String> = None;
                let mut requests: Option<Vec<crate::datadogV1_20270101::model::PointPlotDataProjectionRequest>> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::PointPlotBackendDefinitionType> = None;
                let mut yaxis: Option<crate::datadogV1_20270101::model::Axis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "legend" => {
                            if v.is_null() {
                                continue;
                            }
                            legend = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend) = legend {
                                match _legend {
                                    crate::datadogV1_20270101::model::ScatterPlotLegendType::UnparsedObject(_legend) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                                    crate::datadogV1_20270101::model::PointPlotBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = PointPlotBackendDefinition {
                    description,
                    legend,
                    markers,
                    notes,
                    requests,
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

        deserializer.deserialize_any(PointPlotBackendDefinitionVisitor)
    }
}
