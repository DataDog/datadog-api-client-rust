// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Sunbursts are spot on to highlight how groups contribute to the total of a query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SunburstWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Show the total value in this widget.
    #[serde(rename = "hide_total")]
    pub hide_total: Option<bool>,
    /// Configuration of the legend.
    #[serde(rename = "legend")]
    pub legend: Option<crate::datadogV1::model::SunburstWidgetLegend>,
    /// List of sunburst widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::SunburstWidgetRequest>,
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
    /// Type of the Sunburst widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SunburstWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::SunburstWidgetRequest>,
        type_: crate::datadogV1::model::SunburstWidgetDefinitionType,
    ) -> SunburstWidgetDefinition {
        SunburstWidgetDefinition {
            custom_links: None,
            hide_total: None,
            legend: None,
            requests,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn hide_total(mut self, value: bool) -> Self {
        self.hide_total = Some(value);
        self
    }

    pub fn legend(mut self, value: crate::datadogV1::model::SunburstWidgetLegend) -> Self {
        self.legend = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SunburstWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SunburstWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for SunburstWidgetDefinitionVisitor {
            type Value = SunburstWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut hide_total: Option<bool> = None;
                let mut legend: Option<crate::datadogV1::model::SunburstWidgetLegend> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::SunburstWidgetRequest>> =
                    None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SunburstWidgetDefinitionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "hide_total" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "legend" => {
                            if v.is_null() {
                                continue;
                            }
                            legend = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend) = legend {
                                match _legend {
                                    crate::datadogV1::model::SunburstWidgetLegend::UnparsedObject(_legend) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                                    crate::datadogV1::model::SunburstWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = SunburstWidgetDefinition {
                    custom_links,
                    hide_total,
                    legend,
                    requests,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SunburstWidgetDefinitionVisitor)
    }
}
