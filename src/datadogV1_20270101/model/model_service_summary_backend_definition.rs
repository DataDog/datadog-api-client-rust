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
pub struct ServiceSummaryBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_format")]
    pub display_format: Option<crate::datadogV1_20270101::model::ServiceSummaryLayout>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "env")]
    pub env: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "service")]
    pub service: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_breakdown")]
    pub show_breakdown: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_distribution")]
    pub show_distribution: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_errors")]
    pub show_errors: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_hits")]
    pub show_hits: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_latency")]
    pub show_latency: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_resource_list")]
    pub show_resource_list: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "size_format")]
    pub size_format: Option<crate::datadogV1_20270101::model::ServiceSummarySize>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "span_name")]
    pub span_name: String,
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
    pub type_: crate::datadogV1_20270101::model::ServiceSummaryBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ServiceSummaryBackendDefinition {
    pub fn new(env: String, service: String, span_name: String, type_: crate::datadogV1_20270101::model::ServiceSummaryBackendDefinitionType) -> ServiceSummaryBackendDefinition {
        ServiceSummaryBackendDefinition {
            description: None,
            display_format: None,
            env,
            notes: None,
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
            _unparsed: false,
        }
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn display_format(mut self, value: crate::datadogV1_20270101::model::ServiceSummaryLayout) -> Self {
        self.display_format = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
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
    
    pub fn size_format(mut self, value: crate::datadogV1_20270101::model::ServiceSummarySize) -> Self {
        self.size_format = Some(value);
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
    
}



impl<'de> Deserialize<'de> for ServiceSummaryBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceSummaryBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for ServiceSummaryBackendDefinitionVisitor {
            type Value = ServiceSummaryBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut display_format: Option<crate::datadogV1_20270101::model::ServiceSummaryLayout> = None;
                let mut env: Option<String> = None;
                let mut notes: Option<String> = None;
                let mut service: Option<String> = None;
                let mut show_breakdown: Option<bool> = None;
                let mut show_distribution: Option<bool> = None;
                let mut show_errors: Option<bool> = None;
                let mut show_hits: Option<bool> = None;
                let mut show_latency: Option<bool> = None;
                let mut show_resource_list: Option<bool> = None;
                let mut size_format: Option<crate::datadogV1_20270101::model::ServiceSummarySize> = None;
                let mut span_name: Option<String> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::ServiceSummaryBackendDefinitionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "display_format" => {
                            if v.is_null() {
                                continue;
                            }
                            display_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_format) = display_format {
                                match _display_format {
                                    crate::datadogV1_20270101::model::ServiceSummaryLayout::UnparsedObject(_display_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_breakdown" => {
                            if v.is_null() {
                                continue;
                            }
                            show_breakdown = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_distribution" => {
                            if v.is_null() {
                                continue;
                            }
                            show_distribution = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_errors" => {
                            if v.is_null() {
                                continue;
                            }
                            show_errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_hits" => {
                            if v.is_null() {
                                continue;
                            }
                            show_hits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_latency" => {
                            if v.is_null() {
                                continue;
                            }
                            show_latency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_resource_list" => {
                            if v.is_null() {
                                continue;
                            }
                            show_resource_list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "size_format" => {
                            if v.is_null() {
                                continue;
                            }
                            size_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _size_format) = size_format {
                                match _size_format {
                                    crate::datadogV1_20270101::model::ServiceSummarySize::UnparsedObject(_size_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "span_name" => {
                            span_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::ServiceSummaryBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let span_name = span_name.ok_or_else(|| M::Error::missing_field("span_name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ServiceSummaryBackendDefinition {
                    description,
                    display_format,
                    env,
                    notes,
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
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceSummaryBackendDefinitionVisitor)
    }
}
