// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PatchStatusPageRequestDataAttributes {
    #[serde(rename = "company_logo")]
    pub company_logo: Option<String>,
    #[serde(rename = "domain_prefix")]
    pub domain_prefix: Option<String>,
    #[serde(rename = "email_header_image")]
    pub email_header_image: Option<String>,
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "favicon")]
    pub favicon: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "subscriptions_enabled")]
    pub subscriptions_enabled: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CreateStatusPageRequestDataAttributesType>,
    #[serde(rename = "visualization_type")]
    pub visualization_type:
        Option<crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PatchStatusPageRequestDataAttributes {
    pub fn new() -> PatchStatusPageRequestDataAttributes {
        PatchStatusPageRequestDataAttributes {
            company_logo: None,
            domain_prefix: None,
            email_header_image: None,
            enabled: None,
            favicon: None,
            name: None,
            subscriptions_enabled: None,
            type_: None,
            visualization_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn company_logo(mut self, value: String) -> Self {
        self.company_logo = Some(value);
        self
    }

    pub fn domain_prefix(mut self, value: String) -> Self {
        self.domain_prefix = Some(value);
        self
    }

    pub fn email_header_image(mut self, value: String) -> Self {
        self.email_header_image = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn favicon(mut self, value: String) -> Self {
        self.favicon = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn subscriptions_enabled(mut self, value: bool) -> Self {
        self.subscriptions_enabled = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::CreateStatusPageRequestDataAttributesType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn visualization_type(
        mut self,
        value: crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType,
    ) -> Self {
        self.visualization_type = Some(value);
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

impl Default for PatchStatusPageRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PatchStatusPageRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PatchStatusPageRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for PatchStatusPageRequestDataAttributesVisitor {
            type Value = PatchStatusPageRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut company_logo: Option<String> = None;
                let mut domain_prefix: Option<String> = None;
                let mut email_header_image: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut favicon: Option<String> = None;
                let mut name: Option<String> = None;
                let mut subscriptions_enabled: Option<bool> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CreateStatusPageRequestDataAttributesType,
                > = None;
                let mut visualization_type: Option<
                    crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "company_logo" => {
                            if v.is_null() {
                                continue;
                            }
                            company_logo =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            domain_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email_header_image" => {
                            if v.is_null() {
                                continue;
                            }
                            email_header_image =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "favicon" => {
                            if v.is_null() {
                                continue;
                            }
                            favicon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subscriptions_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            subscriptions_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CreateStatusPageRequestDataAttributesType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "visualization_type" => {
                            if v.is_null() {
                                continue;
                            }
                            visualization_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _visualization_type) = visualization_type {
                                match _visualization_type {
                                    crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType::UnparsedObject(_visualization_type) => {
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

                let content = PatchStatusPageRequestDataAttributes {
                    company_logo,
                    domain_prefix,
                    email_header_image,
                    enabled,
                    favicon,
                    name,
                    subscriptions_enabled,
                    type_,
                    visualization_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PatchStatusPageRequestDataAttributesVisitor)
    }
}
