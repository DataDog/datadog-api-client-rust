// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Alert graphs are timeseries graphs showing the current status of any monitor defined on your system.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlertGraphWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// The title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the alert graph widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::AlertGraphWidgetDefinitionType,
    /// Whether to display the Alert Graph as a timeseries or a top list.
    #[serde(rename = "viz_type")]
    pub viz_type: crate::datadogV1::model::WidgetVizType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AlertGraphWidgetDefinition {
    pub fn new(
        alert_id: String,
        type_: crate::datadogV1::model::AlertGraphWidgetDefinitionType,
        viz_type: crate::datadogV1::model::WidgetVizType,
    ) -> AlertGraphWidgetDefinition {
        AlertGraphWidgetDefinition {
            alert_id,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            viz_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for AlertGraphWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertGraphWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for AlertGraphWidgetDefinitionVisitor {
            type Value = AlertGraphWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_id: Option<String> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::AlertGraphWidgetDefinitionType> =
                    None;
                let mut viz_type: Option<crate::datadogV1::model::WidgetVizType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_id" => {
                            alert_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::AlertGraphWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "viz_type" => {
                            viz_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _viz_type) = viz_type {
                                match _viz_type {
                                    crate::datadogV1::model::WidgetVizType::UnparsedObject(
                                        _viz_type,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let alert_id = alert_id.ok_or_else(|| M::Error::missing_field("alert_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let viz_type = viz_type.ok_or_else(|| M::Error::missing_field("viz_type"))?;

                let content = AlertGraphWidgetDefinition {
                    alert_id,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    viz_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AlertGraphWidgetDefinitionVisitor)
    }
}
