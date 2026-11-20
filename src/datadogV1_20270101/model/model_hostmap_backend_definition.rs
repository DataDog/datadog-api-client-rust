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
pub struct HostmapBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group")]
    pub group: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "node_type")]
    pub node_type: Option<crate::datadogV1_20270101::model::HostmapBackendDefinitionNodeType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "requests")]
    pub requests: crate::datadogV1_20270101::model::HostmapBackendDefinition20270101Requests,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::HostmapBackendDefinitionStyle>,
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
    pub type_: crate::datadogV1_20270101::model::HostmapBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapBackendDefinition {
    pub fn new(requests: crate::datadogV1_20270101::model::HostmapBackendDefinition20270101Requests, type_: crate::datadogV1_20270101::model::HostmapBackendDefinitionType) -> HostmapBackendDefinition {
        HostmapBackendDefinition {
            custom_links: None,
            description: None,
            group: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type: None,
            notes: None,
            requests,
            scope: None,
            style: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
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
    
    pub fn group(mut self, value: Vec<String>) -> Self {
        self.group = Some(value);
        self
    }
    
    pub fn no_group_hosts(mut self, value: bool) -> Self {
        self.no_group_hosts = Some(value);
        self
    }
    
    pub fn no_metric_hosts(mut self, value: bool) -> Self {
        self.no_metric_hosts = Some(value);
        self
    }
    
    pub fn node_type(mut self, value: crate::datadogV1_20270101::model::HostmapBackendDefinitionNodeType) -> Self {
        self.node_type = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn scope(mut self, value: Vec<String>) -> Self {
        self.scope = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::HostmapBackendDefinitionStyle) -> Self {
        self.style = Some(value);
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



impl<'de> Deserialize<'de> for HostmapBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for HostmapBackendDefinitionVisitor {
            type Value = HostmapBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1_20270101::model::WidgetCustomLink>> = None;
                let mut description: Option<String> = None;
                let mut group: Option<Vec<String>> = None;
                let mut no_group_hosts: Option<bool> = None;
                let mut no_metric_hosts: Option<bool> = None;
                let mut node_type: Option<crate::datadogV1_20270101::model::HostmapBackendDefinitionNodeType> = None;
                let mut notes: Option<String> = None;
                let mut requests: Option<crate::datadogV1_20270101::model::HostmapBackendDefinition20270101Requests> = None;
                let mut scope: Option<Vec<String>> = None;
                let mut style: Option<crate::datadogV1_20270101::model::HostmapBackendDefinitionStyle> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::HostmapBackendDefinitionType> = None;
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
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "no_group_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_group_hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "no_metric_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_metric_hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "node_type" => {
                            if v.is_null() {
                                continue;
                            }
                            node_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _node_type) = node_type {
                                match _node_type {
                                    crate::datadogV1_20270101::model::HostmapBackendDefinitionNodeType::UnparsedObject(_node_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _requests) = requests {
                                match _requests {
                                    crate::datadogV1_20270101::model::HostmapBackendDefinition20270101Requests::UnparsedObject(_requests) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::HostmapBackendDefinitionType::UnparsedObject(_type_) => {
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
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = HostmapBackendDefinition {
                    custom_links,
                    description,
                    group,
                    no_group_hosts,
                    no_metric_hosts,
                    node_type,
                    notes,
                    requests,
                    scope,
                    style,
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

        deserializer.deserialize_any(HostmapBackendDefinitionVisitor)
    }
}
