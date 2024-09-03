// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The treemap visualization enables you to display hierarchical and nested data. It is well suited for queries that describe part-whole relationships, such as resource usage by availability zone, data center, or team.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TreeMapWidgetDefinition {
    /// (deprecated) The attribute formerly used to determine color in the widget.
    #[deprecated]
    #[serde(rename = "color_by")]
    pub color_by: Option<crate::datadogV1::model::TreeMapColorBy>,
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// (deprecated) The attribute formerly used to group elements in the widget.
    #[deprecated]
    #[serde(rename = "group_by")]
    pub group_by: Option<crate::datadogV1::model::TreeMapGroupBy>,
    /// List of treemap widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::TreeMapWidgetRequest>,
    /// (deprecated) The attribute formerly used to determine size in the widget.
    #[deprecated]
    #[serde(rename = "size_by")]
    pub size_by: Option<crate::datadogV1::model::TreeMapSizeBy>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the treemap widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TreeMapWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TreeMapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::TreeMapWidgetRequest>,
        type_: crate::datadogV1::model::TreeMapWidgetDefinitionType,
    ) -> TreeMapWidgetDefinition {
        #[allow(deprecated)]
        TreeMapWidgetDefinition {
            color_by: None,
            custom_links: None,
            group_by: None,
            requests,
            size_by: None,
            time: None,
            title: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn color_by(mut self, value: crate::datadogV1::model::TreeMapColorBy) -> Self {
        self.color_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group_by(mut self, value: crate::datadogV1::model::TreeMapGroupBy) -> Self {
        self.group_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn size_by(mut self, value: crate::datadogV1::model::TreeMapSizeBy) -> Self {
        self.size_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl<'de> Deserialize<'de> for TreeMapWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreeMapWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for TreeMapWidgetDefinitionVisitor {
            type Value = TreeMapWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color_by: Option<crate::datadogV1::model::TreeMapColorBy> = None;
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut group_by: Option<crate::datadogV1::model::TreeMapGroupBy> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::TreeMapWidgetRequest>> = None;
                let mut size_by: Option<crate::datadogV1::model::TreeMapSizeBy> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::TreeMapWidgetDefinitionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color_by" => {
                            if v.is_null() {
                                continue;
                            }
                            color_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _color_by) = color_by {
                                match _color_by {
                                    crate::datadogV1::model::TreeMapColorBy::UnparsedObject(
                                        _color_by,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _group_by) = group_by {
                                match _group_by {
                                    crate::datadogV1::model::TreeMapGroupBy::UnparsedObject(
                                        _group_by,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size_by" => {
                            if v.is_null() {
                                continue;
                            }
                            size_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _size_by) = size_by {
                                match _size_by {
                                    crate::datadogV1::model::TreeMapSizeBy::UnparsedObject(
                                        _size_by,
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
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::TreeMapWidgetDefinitionType::UnparsedObject(_type_) => {
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

                #[allow(deprecated)]
                let content = TreeMapWidgetDefinition {
                    color_by,
                    custom_links,
                    group_by,
                    requests,
                    size_by,
                    time,
                    title,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreeMapWidgetDefinitionVisitor)
    }
}
