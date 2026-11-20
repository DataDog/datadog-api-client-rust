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
pub struct AlertValueDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "notes")]
    pub notes: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "precision")]
    pub precision: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_align")]
    pub text_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_size")]
    pub text_size: Option<String>,
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
    pub type_: crate::datadogV1_20270101::model::AlertValueDefinitionType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AlertValueDefinition {
    pub fn new(alert_id: String, type_: crate::datadogV1_20270101::model::AlertValueDefinitionType) -> AlertValueDefinition {
        AlertValueDefinition {
            alert_id,
            description: None,
            notes: None,
            precision: None,
            text_align: None,
            text_size: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            unit: None,
            _unparsed: false,
        }
    }
    
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }
    
    pub fn precision(mut self, value: f64) -> Self {
        self.precision = Some(value);
        self
    }
    
    pub fn text_align(mut self, value: crate::datadogV1_20270101::model::WidgetTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }
    
    pub fn text_size(mut self, value: String) -> Self {
        self.text_size = Some(value);
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
    
    pub fn unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for AlertValueDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertValueDefinitionVisitor;
        impl<'a> Visitor<'a> for AlertValueDefinitionVisitor {
            type Value = AlertValueDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_id: Option<String> = None;
                let mut description: Option<String> = None;
                let mut notes: Option<String> = None;
                let mut precision: Option<f64> = None;
                let mut text_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut text_size: Option<String> = None;
                let mut time: Option<crate::datadogV1_20270101::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1_20270101::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::AlertValueDefinitionType> = None;
                let mut unit: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_id" => {
                            alert_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "notes" => {
                            if v.is_null() {
                                continue;
                            }
                            notes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "precision" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            precision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "text_align" => {
                            if v.is_null() {
                                continue;
                            }
                            text_align = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _text_align) = text_align {
                                match _text_align {
                                    crate::datadogV1_20270101::model::WidgetTextAlign::UnparsedObject(_text_align) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "text_size" => {
                            if v.is_null() {
                                continue;
                            }
                            text_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::AlertValueDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let alert_id = alert_id.ok_or_else(|| M::Error::missing_field("alert_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AlertValueDefinition {
                    alert_id,
                    description,
                    notes,
                    precision,
                    text_align,
                    text_size,
                    time,
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

        deserializer.deserialize_any(AlertValueDefinitionVisitor)
    }
}
