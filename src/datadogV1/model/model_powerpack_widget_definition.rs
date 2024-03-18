// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The powerpack widget allows you to keep similar graphs together on your timeboard. Each group has a custom header, can hold one to many graphs, and is collapsible.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackWidgetDefinition {
    /// Background color of the powerpack title.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// URL of image to display as a banner for the powerpack.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// UUID of the associated powerpack.
    #[serde(rename = "powerpack_id")]
    pub powerpack_id: String,
    /// Whether to show the title or not.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Powerpack template variables.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<crate::datadogV1::model::PowerpackTemplateVariables>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Type of the powerpack widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::PowerpackWidgetDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PowerpackWidgetDefinition {
    pub fn new(
        powerpack_id: String,
        type_: crate::datadogV1::model::PowerpackWidgetDefinitionType,
    ) -> PowerpackWidgetDefinition {
        PowerpackWidgetDefinition {
            background_color: None,
            banner_img: None,
            powerpack_id,
            show_title: None,
            template_variables: None,
            title: None,
            type_,
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

    pub fn show_title(mut self, value: bool) -> Self {
        self.show_title = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: crate::datadogV1::model::PowerpackTemplateVariables,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for PowerpackWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for PowerpackWidgetDefinitionVisitor {
            type Value = PowerpackWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut background_color: Option<String> = None;
                let mut banner_img: Option<String> = None;
                let mut powerpack_id: Option<String> = None;
                let mut show_title: Option<bool> = None;
                let mut template_variables: Option<
                    crate::datadogV1::model::PowerpackTemplateVariables,
                > = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::PowerpackWidgetDefinitionType> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "background_color" => {
                            if v.is_null() {
                                continue;
                            }
                            background_color =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "banner_img" => {
                            if v.is_null() {
                                continue;
                            }
                            banner_img = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "powerpack_id" => {
                            powerpack_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_title" => {
                            if v.is_null() {
                                continue;
                            }
                            show_title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::PowerpackWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let powerpack_id =
                    powerpack_id.ok_or_else(|| M::Error::missing_field("powerpack_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = PowerpackWidgetDefinition {
                    background_color,
                    banner_img,
                    powerpack_id,
                    show_title,
                    template_variables,
                    title,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackWidgetDefinitionVisitor)
    }
}
