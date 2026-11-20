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
pub struct SplitGroupBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "has_uniform_y_axes")]
    pub has_uniform_y_axes: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "size")]
    pub size: crate::datadogV1_20270101::model::FORMSIZES,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "source_widget_definition")]
    pub source_widget_definition: crate::datadogV1_20270101::model::SplitGroupBackendDefinition20270101SourceWidgetDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "split_config")]
    pub split_config: crate::datadogV1_20270101::model::SplitConfig,
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
    pub type_: crate::datadogV1_20270101::model::SplitGroupBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SplitGroupBackendDefinition {
    pub fn new(size: crate::datadogV1_20270101::model::FORMSIZES, source_widget_definition: crate::datadogV1_20270101::model::SplitGroupBackendDefinition20270101SourceWidgetDefinition, split_config: crate::datadogV1_20270101::model::SplitConfig, type_: crate::datadogV1_20270101::model::SplitGroupBackendDefinitionType) -> SplitGroupBackendDefinition {
        SplitGroupBackendDefinition {
            description: None,
            has_uniform_y_axes: None,
            notes: None,
            size,
            source_widget_definition,
            split_config,
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
    
    pub fn has_uniform_y_axes(mut self, value: bool) -> Self {
        self.has_uniform_y_axes = Some(value);
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
    
}



impl<'de> Deserialize<'de> for SplitGroupBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitGroupBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for SplitGroupBackendDefinitionVisitor {
            type Value = SplitGroupBackendDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut has_uniform_y_axes: Option<bool> = None;
                let mut notes: Option<String> = None;
                let mut size: Option<crate::datadogV1_20270101::model::FORMSIZES> = None;
                let mut source_widget_definition: Option<crate::datadogV1_20270101::model::SplitGroupBackendDefinition20270101SourceWidgetDefinition> = None;
                let mut split_config: Option<crate::datadogV1_20270101::model::SplitConfig> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::SplitGroupBackendDefinitionType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "has_uniform_y_axes" => {
                            if v.is_null() {
                                continue;
                            }
                            has_uniform_y_axes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _size) = size {
                                match _size {
                                    crate::datadogV1_20270101::model::FORMSIZES::UnparsedObject(_size) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "source_widget_definition" => {
                            source_widget_definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source_widget_definition) = source_widget_definition {
                                match _source_widget_definition {
                                    crate::datadogV1_20270101::model::SplitGroupBackendDefinition20270101SourceWidgetDefinition::UnparsedObject(_source_widget_definition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "split_config" => {
                            split_config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::SplitGroupBackendDefinitionType::UnparsedObject(_type_) => {
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
                let size = size.ok_or_else(|| M::Error::missing_field("size"))?;
                let source_widget_definition = source_widget_definition.ok_or_else(|| M::Error::missing_field("source_widget_definition"))?;
                let split_config = split_config.ok_or_else(|| M::Error::missing_field("split_config"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SplitGroupBackendDefinition {
                    description,
                    has_uniform_y_axes,
                    notes,
                    size,
                    source_widget_definition,
                    split_config,
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

        deserializer.deserialize_any(SplitGroupBackendDefinitionVisitor)
    }
}
