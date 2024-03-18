// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The split graph widget allows you to create repeating units of a graph - one for each value in a group (for example: one per service)
///
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplitGraphWidgetDefinition {
    /// Normalize y axes across graphs
    #[serde(rename = "has_uniform_y_axes")]
    pub has_uniform_y_axes: Option<bool>,
    /// Size of the individual graphs in the split.
    #[serde(rename = "size")]
    pub size: crate::datadogV1::model::SplitGraphVizSize,
    /// The original widget we are splitting on.
    #[serde(rename = "source_widget_definition")]
    pub source_widget_definition: crate::datadogV1::model::SplitGraphSourceWidgetDefinition,
    /// Encapsulates all user choices about how to split a graph.
    #[serde(rename = "split_config")]
    pub split_config: crate::datadogV1::model::SplitConfig,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the split graph widget
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SplitGraphWidgetDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SplitGraphWidgetDefinition {
    pub fn new(
        size: crate::datadogV1::model::SplitGraphVizSize,
        source_widget_definition: crate::datadogV1::model::SplitGraphSourceWidgetDefinition,
        split_config: crate::datadogV1::model::SplitConfig,
        type_: crate::datadogV1::model::SplitGraphWidgetDefinitionType,
    ) -> SplitGraphWidgetDefinition {
        SplitGraphWidgetDefinition {
            has_uniform_y_axes: None,
            size,
            source_widget_definition,
            split_config,
            time: None,
            title: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn has_uniform_y_axes(mut self, value: bool) -> Self {
        self.has_uniform_y_axes = Some(value);
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
}

impl<'de> Deserialize<'de> for SplitGraphWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitGraphWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for SplitGraphWidgetDefinitionVisitor {
            type Value = SplitGraphWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_uniform_y_axes: Option<bool> = None;
                let mut size: Option<crate::datadogV1::model::SplitGraphVizSize> = None;
                let mut source_widget_definition: Option<
                    crate::datadogV1::model::SplitGraphSourceWidgetDefinition,
                > = None;
                let mut split_config: Option<crate::datadogV1::model::SplitConfig> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SplitGraphWidgetDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_uniform_y_axes" => {
                            if v.is_null() {
                                continue;
                            }
                            has_uniform_y_axes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _size) = size {
                                match _size {
                                    crate::datadogV1::model::SplitGraphVizSize::UnparsedObject(
                                        _size,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "source_widget_definition" => {
                            source_widget_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source_widget_definition) = source_widget_definition {
                                match _source_widget_definition {
                                    crate::datadogV1::model::SplitGraphSourceWidgetDefinition::UnparsedObject(_source_widget_definition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "split_config" => {
                            split_config =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SplitGraphWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let size = size.ok_or_else(|| M::Error::missing_field("size"))?;
                let source_widget_definition = source_widget_definition
                    .ok_or_else(|| M::Error::missing_field("source_widget_definition"))?;
                let split_config =
                    split_config.ok_or_else(|| M::Error::missing_field("split_config"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SplitGraphWidgetDefinition {
                    has_uniform_y_axes,
                    size,
                    source_widget_definition,
                    split_config,
                    time,
                    title,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitGraphWidgetDefinitionVisitor)
    }
}
