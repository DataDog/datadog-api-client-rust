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
pub struct StatusPageDataAttributes {
    #[serde(
        rename = "company_logo",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub company_logo: Option<Option<String>>,
    #[serde(rename = "components")]
    pub components: Option<Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItems>>,
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "custom_domain",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_domain: Option<Option<String>>,
    #[serde(rename = "custom_domain_enabled")]
    pub custom_domain_enabled: Option<bool>,
    #[serde(rename = "domain_prefix")]
    pub domain_prefix: Option<String>,
    #[serde(
        rename = "email_header_image",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub email_header_image: Option<Option<String>>,
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "favicon",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub favicon: Option<Option<String>>,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "page_url")]
    pub page_url: Option<String>,
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

impl StatusPageDataAttributes {
    pub fn new() -> StatusPageDataAttributes {
        StatusPageDataAttributes {
            company_logo: None,
            components: None,
            created_at: None,
            custom_domain: None,
            custom_domain_enabled: None,
            domain_prefix: None,
            email_header_image: None,
            enabled: None,
            favicon: None,
            modified_at: None,
            name: None,
            page_url: None,
            subscriptions_enabled: None,
            type_: None,
            visualization_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn company_logo(mut self, value: Option<String>) -> Self {
        self.company_logo = Some(value);
        self
    }

    pub fn components(
        mut self,
        value: Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItems>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn custom_domain(mut self, value: Option<String>) -> Self {
        self.custom_domain = Some(value);
        self
    }

    pub fn custom_domain_enabled(mut self, value: bool) -> Self {
        self.custom_domain_enabled = Some(value);
        self
    }

    pub fn domain_prefix(mut self, value: String) -> Self {
        self.domain_prefix = Some(value);
        self
    }

    pub fn email_header_image(mut self, value: Option<String>) -> Self {
        self.email_header_image = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn favicon(mut self, value: Option<String>) -> Self {
        self.favicon = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn page_url(mut self, value: String) -> Self {
        self.page_url = Some(value);
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

impl Default for StatusPageDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StatusPageDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageDataAttributesVisitor;
        impl<'a> Visitor<'a> for StatusPageDataAttributesVisitor {
            type Value = StatusPageDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut company_logo: Option<Option<String>> = None;
                let mut components: Option<
                    Vec<crate::datadogV2::model::StatusPageDataAttributesComponentsItems>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut custom_domain: Option<Option<String>> = None;
                let mut custom_domain_enabled: Option<bool> = None;
                let mut domain_prefix: Option<String> = None;
                let mut email_header_image: Option<Option<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut favicon: Option<Option<String>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut page_url: Option<String> = None;
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
                            company_logo =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_domain" => {
                            custom_domain =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "custom_domain_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_domain_enabled =
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
                            favicon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_url" => {
                            if v.is_null() {
                                continue;
                            }
                            page_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = StatusPageDataAttributes {
                    company_logo,
                    components,
                    created_at,
                    custom_domain,
                    custom_domain_enabled,
                    domain_prefix,
                    email_header_image,
                    enabled,
                    favicon,
                    modified_at,
                    name,
                    page_url,
                    subscriptions_enabled,
                    type_,
                    visualization_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageDataAttributesVisitor)
    }
}
