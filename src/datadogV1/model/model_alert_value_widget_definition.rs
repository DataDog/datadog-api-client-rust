// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Alert values are query values showing the current value of the metric in any monitor defined on your system.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlertValueWidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
    /// Number of decimal to show. If not defined, will use the raw value.
    #[serde(rename = "precision")]
    pub precision: Option<i64>,
    /// How to align the text on the widget.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of value in the widget.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the alert value widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::AlertValueWidgetDefinitionType,
    /// Unit to display with the value.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AlertValueWidgetDefinition {
    pub fn new(
        alert_id: String,
        type_: crate::datadogV1::model::AlertValueWidgetDefinitionType,
    ) -> AlertValueWidgetDefinition {
        AlertValueWidgetDefinition {
            alert_id,
            precision: None,
            text_align: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            unit: None,
            _unparsed: false,
        }
    }

    pub fn precision(mut self, value: i64) -> Self {
        self.precision = Some(value);
        self
    }

    pub fn text_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
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

    pub fn unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for AlertValueWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertValueWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for AlertValueWidgetDefinitionVisitor {
            type Value = AlertValueWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_id: Option<String> = None;
                let mut precision: Option<i64> = None;
                let mut text_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::AlertValueWidgetDefinitionType> =
                    None;
                let mut unit: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_id" => {
                            alert_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "precision" => {
                            if v.is_null() {
                                continue;
                            }
                            precision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::AlertValueWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let alert_id = alert_id.ok_or_else(|| M::Error::missing_field("alert_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AlertValueWidgetDefinition {
                    alert_id,
                    precision,
                    text_align,
                    title,
                    title_align,
                    title_size,
                    type_,
                    unit,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AlertValueWidgetDefinitionVisitor)
    }
}
