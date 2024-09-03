// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The scatter plot visualization allows you to graph a chosen scope over two different metrics with their respective aggregation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterPlotWidgetDefinition {
    /// List of groups used for colors.
    #[serde(rename = "color_by_groups")]
    pub color_by_groups: Option<Vec<String>>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Widget definition.
    #[serde(rename = "requests")]
    pub requests: crate::datadogV1::model::ScatterPlotWidgetDefinitionRequests,
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
    /// Type of the scatter plot widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ScatterPlotWidgetDefinitionType,
    /// Axis controls for the widget.
    #[serde(rename = "xaxis")]
    pub xaxis: Option<crate::datadogV1::model::WidgetAxis>,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::WidgetAxis>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScatterPlotWidgetDefinition {
    pub fn new(
        requests: crate::datadogV1::model::ScatterPlotWidgetDefinitionRequests,
        type_: crate::datadogV1::model::ScatterPlotWidgetDefinitionType,
    ) -> ScatterPlotWidgetDefinition {
        ScatterPlotWidgetDefinition {
            color_by_groups: None,
            custom_links: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            xaxis: None,
            yaxis: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn color_by_groups(mut self, value: Vec<String>) -> Self {
        self.color_by_groups = Some(value);
        self
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
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

    pub fn xaxis(mut self, value: crate::datadogV1::model::WidgetAxis) -> Self {
        self.xaxis = Some(value);
        self
    }

    pub fn yaxis(mut self, value: crate::datadogV1::model::WidgetAxis) -> Self {
        self.yaxis = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ScatterPlotWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterPlotWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for ScatterPlotWidgetDefinitionVisitor {
            type Value = ScatterPlotWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_by_groups: Option<Vec<String>> = None;
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut requests: Option<
                    crate::datadogV1::model::ScatterPlotWidgetDefinitionRequests,
                > = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::ScatterPlotWidgetDefinitionType> =
                    None;
                let mut xaxis: Option<crate::datadogV1::model::WidgetAxis> = None;
                let mut yaxis: Option<crate::datadogV1::model::WidgetAxis> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_by_groups" => {
                            if v.is_null() {
                                continue;
                            }
                            color_by_groups =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _time) = time {
                                match _time {
                                    crate::datadogV1::model::WidgetTime::UnparsedObject(_time) => {
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
                                    crate::datadogV1::model::ScatterPlotWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "xaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            xaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ScatterPlotWidgetDefinition {
                    color_by_groups,
                    custom_links,
                    requests,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    xaxis,
                    yaxis,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterPlotWidgetDefinitionVisitor)
    }
}
