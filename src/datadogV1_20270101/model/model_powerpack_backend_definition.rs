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
pub struct PowerpackBackendDefinition {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "powerpack_id")]
    pub powerpack_id: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "show_title")]
    pub show_title: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV1_20270101::model::PowerpackState>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariables>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::PowerpackBackendDefinitionType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PowerpackBackendDefinition {
    pub fn new(powerpack_id: String, type_: crate::datadogV1_20270101::model::PowerpackBackendDefinitionType) -> PowerpackBackendDefinition {
        PowerpackBackendDefinition {
            background_color: None,
            banner_img: None,
            powerpack_id,
            show_title: None,
            state: None,
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
    
    pub fn state(mut self, value: crate::datadogV1_20270101::model::PowerpackState) -> Self {
        self.state = Some(value);
        self
    }
    
    pub fn template_variables(mut self, value: crate::datadogV1_20270101::model::PowerpackBackendTemplateVariables) -> Self {
        self.template_variables = Some(value);
        self
    }
    
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for PowerpackBackendDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackBackendDefinitionVisitor;
        impl<'a> Visitor<'a> for PowerpackBackendDefinitionVisitor {
            type Value = PowerpackBackendDefinition;

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
                let mut state: Option<crate::datadogV1_20270101::model::PowerpackState> = None;
                let mut template_variables: Option<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariables> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::PowerpackBackendDefinitionType> = None;
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
                        "powerpack_id" => {
                            powerpack_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "show_title" => {
                            if v.is_null() {
                                continue;
                            }
                            show_title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV1_20270101::model::PowerpackState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1_20270101::model::PowerpackBackendDefinitionType::UnparsedObject(_type_) => {
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
                let powerpack_id = powerpack_id.ok_or_else(|| M::Error::missing_field("powerpack_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = PowerpackBackendDefinition {
                    background_color,
                    banner_img,
                    powerpack_id,
                    show_title,
                    state,
                    template_variables,
                    title,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackBackendDefinitionVisitor)
    }
}
