// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query values display the current value of a given metric, APM, or log query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct QueryValueWidgetDefinition {
    /// Whether to use auto-scaling or not.
    #[serde(rename = "autoscale")]
    pub autoscale: Option<bool>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Display a unit of your choice on the widget.
    #[serde(rename = "custom_unit")]
    pub custom_unit: Option<String>,
    /// Number of decimals to show. If not defined, the widget uses the raw value.
    #[serde(rename = "precision")]
    pub precision: Option<i64>,
    /// Widget definition.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::QueryValueWidgetRequest>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Set a timeseries on the widget background.
    #[serde(rename = "timeseries_background")]
    pub timeseries_background: Option<crate::datadogV1::model::TimeseriesBackground>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the query value widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::QueryValueWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl QueryValueWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::QueryValueWidgetRequest>,
        type_: crate::datadogV1::model::QueryValueWidgetDefinitionType,
    ) -> QueryValueWidgetDefinition {
        QueryValueWidgetDefinition {
            autoscale: None,
            custom_links: None,
            custom_unit: None,
            precision: None,
            requests,
            text_align: None,
            time: None,
            timeseries_background: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn autoscale(mut self, value: bool) -> Self {
        self.autoscale = Some(value);
        self
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn custom_unit(mut self, value: String) -> Self {
        self.custom_unit = Some(value);
        self
    }

    pub fn precision(mut self, value: i64) -> Self {
        self.precision = Some(value);
        self
    }

    pub fn text_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    pub fn timeseries_background(
        mut self,
        value: crate::datadogV1::model::TimeseriesBackground,
    ) -> Self {
        self.timeseries_background = Some(value);
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

impl<'de> Deserialize<'de> for QueryValueWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryValueWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for QueryValueWidgetDefinitionVisitor {
            type Value = QueryValueWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut autoscale: Option<bool> = None;
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut custom_unit: Option<String> = None;
                let mut precision: Option<i64> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::QueryValueWidgetRequest>> =
                    None;
                let mut text_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut timeseries_background: Option<
                    crate::datadogV1::model::TimeseriesBackground,
                > = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::QueryValueWidgetDefinitionType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "autoscale" => {
                            if v.is_null() {
                                continue;
                            }
                            autoscale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_unit" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_unit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "precision" => {
                            if v.is_null() {
                                continue;
                            }
                            precision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _text_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "timeseries_background" => {
                            if v.is_null() {
                                continue;
                            }
                            timeseries_background =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::QueryValueWidgetDefinitionType::UnparsedObject(_type_) => {
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

                let content = QueryValueWidgetDefinition {
                    autoscale,
                    custom_links,
                    custom_unit,
                    precision,
                    requests,
                    text_align,
                    time,
                    timeseries_background,
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

        deserializer.deserialize_any(QueryValueWidgetDefinitionVisitor)
    }
}
