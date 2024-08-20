// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The host map widget graphs any metric across your hosts using the same visualization available from the main Host Map page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HostMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// List of tag prefixes to group by.
    #[serde(rename = "group")]
    pub group: Option<Vec<String>>,
    /// Whether to show the hosts that don’t fit in a group.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Whether to show the hosts with no metrics.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Which type of node to use in the map.
    #[serde(rename = "node_type")]
    pub node_type: Option<crate::datadogV1::model::WidgetNodeType>,
    /// Notes on the title.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// List of definitions.
    #[serde(rename = "requests")]
    pub requests: crate::datadogV1::model::HostMapWidgetDefinitionRequests,
    /// List of tags used to filter the map.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// The style to apply to the widget.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1::model::HostMapWidgetDefinitionStyle>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the host map widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::HostMapWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HostMapWidgetDefinition {
    pub fn new(
        requests: crate::datadogV1::model::HostMapWidgetDefinitionRequests,
        type_: crate::datadogV1::model::HostMapWidgetDefinitionType,
    ) -> HostMapWidgetDefinition {
        HostMapWidgetDefinition {
            custom_links: None,
            group: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type: None,
            notes: None,
            requests,
            scope: None,
            style: None,
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

    pub fn node_type(mut self, value: crate::datadogV1::model::WidgetNodeType) -> Self {
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

    pub fn style(mut self, value: crate::datadogV1::model::HostMapWidgetDefinitionStyle) -> Self {
        self.style = Some(value);
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

impl<'de> Deserialize<'de> for HostMapWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostMapWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for HostMapWidgetDefinitionVisitor {
            type Value = HostMapWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut group: Option<Vec<String>> = None;
                let mut no_group_hosts: Option<bool> = None;
                let mut no_metric_hosts: Option<bool> = None;
                let mut node_type: Option<crate::datadogV1::model::WidgetNodeType> = None;
                let mut notes: Option<String> = None;
                let mut requests: Option<crate::datadogV1::model::HostMapWidgetDefinitionRequests> =
                    None;
                let mut scope: Option<Vec<String>> = None;
                let mut style: Option<crate::datadogV1::model::HostMapWidgetDefinitionStyle> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::HostMapWidgetDefinitionType> = None;
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
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_group_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_group_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_metric_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_metric_hosts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "node_type" => {
                            if v.is_null() {
                                continue;
                            }
                            node_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _node_type) = node_type {
                                match _node_type {
                                    crate::datadogV1::model::WidgetNodeType::UnparsedObject(
                                        _node_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::HostMapWidgetDefinitionType::UnparsedObject(_type_) => {
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

                let content = HostMapWidgetDefinition {
                    custom_links,
                    group,
                    no_group_hosts,
                    no_metric_hosts,
                    node_type,
                    notes,
                    requests,
                    scope,
                    style,
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

        deserializer.deserialize_any(HostMapWidgetDefinitionVisitor)
    }
}
