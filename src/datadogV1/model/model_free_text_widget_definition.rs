// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Free text is a widget that allows you to add headings to your screenboard. Commonly used to state the overall purpose of the dashboard. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FreeTextWidgetDefinition {
    /// Color of the text.
    #[serde(rename = "color")]
    pub color: Option<String>,
    /// Size of the text.
    #[serde(rename = "font_size")]
    pub font_size: Option<String>,
    /// Text to display.
    #[serde(rename = "text")]
    pub text: String,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Type of the free text widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::FreeTextWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FreeTextWidgetDefinition {
    pub fn new(
        text: String,
        type_: crate::datadogV1::model::FreeTextWidgetDefinitionType,
    ) -> FreeTextWidgetDefinition {
        FreeTextWidgetDefinition {
            color: None,
            font_size: None,
            text,
            text_align: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn color(mut self, value: String) -> Self {
        self.color = Some(value);
        self
    }

    pub fn font_size(mut self, value: String) -> Self {
        self.font_size = Some(value);
        self
    }

    pub fn text_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
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

impl<'de> Deserialize<'de> for FreeTextWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FreeTextWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for FreeTextWidgetDefinitionVisitor {
            type Value = FreeTextWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut color: Option<String> = None;
                let mut font_size: Option<String> = None;
                let mut text: Option<String> = None;
                let mut text_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut type_: Option<crate::datadogV1::model::FreeTextWidgetDefinitionType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "color" => {
                            if v.is_null() {
                                continue;
                            }
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "font_size" => {
                            if v.is_null() {
                                continue;
                            }
                            font_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _text_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::FreeTextWidgetDefinitionType::UnparsedObject(_type_) => {
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
                let text = text.ok_or_else(|| M::Error::missing_field("text"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = FreeTextWidgetDefinition {
                    color,
                    font_size,
                    text,
                    text_align,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FreeTextWidgetDefinitionVisitor)
    }
}
