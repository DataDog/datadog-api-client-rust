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
pub struct GroupBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "default_state")]
    pub default_state: Option<crate::datadogV1_20270101::model::GroupBackendDefinitionDefaultState>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "layout_type")]
    pub layout_type: crate::datadogV1_20270101::model::LayoutType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::GroupBackendDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::datadogV1_20270101::model::DashboardBackendWidget>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl GroupBackendDefinition {
    pub fn new(layout_type: crate::datadogV1_20270101::model::LayoutType, type_: crate::datadogV1_20270101::model::GroupBackendDefinitionType, widgets: Vec<crate::datadogV1_20270101::model::DashboardBackendWidget>) -> GroupBackendDefinition {
        GroupBackendDefinition {
            background_color: None,
            banner_img: None,
            default_state: None,
            description: None,
            layout_type,
            show_title: None,
            title: None,
            type_,
            widgets,
            _unparsed: false,
        }
    }
    
    pub fn background_color(mut self, value: String) -> Self {
        self.background_color = Some(value);
        self
    }
    
    pub fn banner_img(mut self, value: String) -> Self {
        self.banner_img = Some(value);
        self
    }
    
    pub fn default_state(mut self, value: crate::datadogV1_20270101::model::GroupBackendDefinitionDefaultState) -> Self {
        self.default_state = Some(value);
        self
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn show_title(mut self, value: bool) -> Self {
        self.show_title = Some(value);
        self
    }
    
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for GroupBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GroupBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for GroupBackendDefinitionVisitor {
            type Value = GroupBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut banner_img: Option<String> = None;
                let mut default_state: Option<crate::datadogV1_20270101::model::GroupBackendDefinitionDefaultState> = None;
                let mut description: Option<String> = None;
                let mut layout_type: Option<crate::datadogV1_20270101::model::LayoutType> = None;
                let mut show_title: Option<bool> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::GroupBackendDefinitionType> = None;
                let mut widgets: Option<Vec<crate::datadogV1_20270101::model::DashboardBackendWidget>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "background_color" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "banner_img" => {
                            if v.is_null() {
                                continue;
                            }
                            banner_img = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "default_state" => {
                            if v.is_null() {
                                continue;
                            }
                            default_state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _default_state) = default_state {
                                match _default_state {
                                    crate::datadogV1_20270101::model::GroupBackendDefinitionDefaultState::UnparsedObject(_default_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "layout_type" => {
                            layout_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout_type) = layout_type {
                                match _layout_type {
                                    crate::datadogV1_20270101::model::LayoutType::UnparsedObject(_layout_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "show_title" => {
                            if v.is_null() {
                                continue;
                            }
                            show_title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::GroupBackendDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "widgets" => {
                            widgets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let layout_type = layout_type.ok_or_else(|| M::Error::missing_field("layout_type"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let widgets = widgets.ok_or_else(|| M::Error::missing_field("widgets"))?;

                let content = GroupBackendDefinition {
                    background_color,
                    banner_img,
                    default_state,
                    description,
                    layout_type,
                    show_title,
                    title,
                    type_,
                    widgets,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GroupBackendDefinitionVisitor)
    }
}
