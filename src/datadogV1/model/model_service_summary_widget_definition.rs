// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The service summary displays the graphs of a chosen service in your screenboard. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceSummaryWidgetDefinition {
    /// Number of columns to display.
    #[serde(rename = "display_format")]
    pub display_format: Option<crate::datadogV1::model::WidgetServiceSummaryDisplayFormat>,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// APM service.
    #[serde(rename = "service")]
    pub service: String,
    /// Whether to show the latency breakdown or not.
    #[serde(rename = "show_breakdown")]
    pub show_breakdown: Option<bool>,
    /// Whether to show the latency distribution or not.
    #[serde(rename = "show_distribution")]
    pub show_distribution: Option<bool>,
    /// Whether to show the error metrics or not.
    #[serde(rename = "show_errors")]
    pub show_errors: Option<bool>,
    /// Whether to show the hits metrics or not.
    #[serde(rename = "show_hits")]
    pub show_hits: Option<bool>,
    /// Whether to show the latency metrics or not.
    #[serde(rename = "show_latency")]
    pub show_latency: Option<bool>,
    /// Whether to show the resource list or not.
    #[serde(rename = "show_resource_list")]
    pub show_resource_list: Option<bool>,
    /// Size of the widget.
    #[serde(rename = "size_format")]
    pub size_format: Option<crate::datadogV1::model::WidgetSizeFormat>,
    /// APM span name.
    #[serde(rename = "span_name")]
    pub span_name: String,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the service summary widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ServiceSummaryWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceSummaryWidgetDefinition {
    pub fn new(
        env: String,
        service: String,
        span_name: String,
        type_: crate::datadogV1::model::ServiceSummaryWidgetDefinitionType,
    ) -> ServiceSummaryWidgetDefinition {
        ServiceSummaryWidgetDefinition {
            display_format: None,
            env,
            service,
            show_breakdown: None,
            show_distribution: None,
            show_errors: None,
            show_hits: None,
            show_latency: None,
            show_resource_list: None,
            size_format: None,
            span_name,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_format(
        mut self,
        value: crate::datadogV1::model::WidgetServiceSummaryDisplayFormat,
    ) -> Self {
        self.display_format = Some(value);
        self
    }

    pub fn show_breakdown(mut self, value: bool) -> Self {
        self.show_breakdown = Some(value);
        self
    }

    pub fn show_distribution(mut self, value: bool) -> Self {
        self.show_distribution = Some(value);
        self
    }

    pub fn show_errors(mut self, value: bool) -> Self {
        self.show_errors = Some(value);
        self
    }

    pub fn show_hits(mut self, value: bool) -> Self {
        self.show_hits = Some(value);
        self
    }

    pub fn show_latency(mut self, value: bool) -> Self {
        self.show_latency = Some(value);
        self
    }

    pub fn show_resource_list(mut self, value: bool) -> Self {
        self.show_resource_list = Some(value);
        self
    }

    pub fn size_format(mut self, value: crate::datadogV1::model::WidgetSizeFormat) -> Self {
        self.size_format = Some(value);
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

impl<'de> Deserialize<'de> for ServiceSummaryWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceSummaryWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for ServiceSummaryWidgetDefinitionVisitor {
            type Value = ServiceSummaryWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_format: Option<
                    crate::datadogV1::model::WidgetServiceSummaryDisplayFormat,
                > = None;
                let mut env: Option<String> = None;
                let mut service: Option<String> = None;
                let mut show_breakdown: Option<bool> = None;
                let mut show_distribution: Option<bool> = None;
                let mut show_errors: Option<bool> = None;
                let mut show_hits: Option<bool> = None;
                let mut show_latency: Option<bool> = None;
                let mut show_resource_list: Option<bool> = None;
                let mut size_format: Option<crate::datadogV1::model::WidgetSizeFormat> = None;
                let mut span_name: Option<String> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::ServiceSummaryWidgetDefinitionType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_format" => {
                            if v.is_null() {
                                continue;
                            }
                            display_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_format) = display_format {
                                match _display_format {
                                    crate::datadogV1::model::WidgetServiceSummaryDisplayFormat::UnparsedObject(_display_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_breakdown" => {
                            if v.is_null() {
                                continue;
                            }
                            show_breakdown =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_distribution" => {
                            if v.is_null() {
                                continue;
                            }
                            show_distribution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_errors" => {
                            if v.is_null() {
                                continue;
                            }
                            show_errors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_hits" => {
                            if v.is_null() {
                                continue;
                            }
                            show_hits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_latency" => {
                            if v.is_null() {
                                continue;
                            }
                            show_latency =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_resource_list" => {
                            if v.is_null() {
                                continue;
                            }
                            show_resource_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size_format" => {
                            if v.is_null() {
                                continue;
                            }
                            size_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _size_format) = size_format {
                                match _size_format {
                                    crate::datadogV1::model::WidgetSizeFormat::UnparsedObject(
                                        _size_format,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "span_name" => {
                            span_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::ServiceSummaryWidgetDefinitionType::UnparsedObject(_type_) => {
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
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let span_name = span_name.ok_or_else(|| M::Error::missing_field("span_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ServiceSummaryWidgetDefinition {
                    display_format,
                    env,
                    service,
                    show_breakdown,
                    show_distribution,
                    show_errors,
                    show_hits,
                    show_latency,
                    show_resource_list,
                    size_format,
                    span_name,
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

        deserializer.deserialize_any(ServiceSummaryWidgetDefinitionVisitor)
    }
}
