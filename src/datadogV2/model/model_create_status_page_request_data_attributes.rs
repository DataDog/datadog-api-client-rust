// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The supported attributes for creating a status page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateStatusPageRequestDataAttributes {
    /// The base64-encoded image data displayed on the status page.
    #[serde(rename = "company_logo")]
    pub company_logo: Option<String>,
    /// The components displayed on the status page.
    #[serde(rename = "components")]
    pub components:
        Option<Vec<crate::datadogV2::model::CreateStatusPageRequestDataAttributesComponentsItems>>,
    /// The subdomain of the status page's url taking the form `<https://{domain_prefix}.statuspage.datadoghq.com`.> Globally unique across Datadog Status Pages.
    #[serde(rename = "domain_prefix")]
    pub domain_prefix: String,
    /// Base64-encoded image data included in email notifications sent to status page subscribers.
    #[serde(rename = "email_header_image")]
    pub email_header_image: Option<String>,
    /// Whether the status page is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Base64-encoded image data displayed in the browser tab.
    #[serde(rename = "favicon")]
    pub favicon: Option<String>,
    /// The name of the status page.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether users can subscribe to the status page.
    #[serde(rename = "subscriptions_enabled")]
    pub subscriptions_enabled: Option<bool>,
    /// The type of the status page controlling how the status page is accessed.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CreateStatusPageRequestDataAttributesType,
    /// The visualization type of the status page.
    #[serde(rename = "visualization_type")]
    pub visualization_type:
        crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateStatusPageRequestDataAttributes {
    pub fn new(
        domain_prefix: String,
        enabled: bool,
        name: String,
        type_: crate::datadogV2::model::CreateStatusPageRequestDataAttributesType,
        visualization_type: crate::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType,
    ) -> CreateStatusPageRequestDataAttributes {
        CreateStatusPageRequestDataAttributes {
            company_logo: None,
            components: None,
            domain_prefix,
            email_header_image: None,
            enabled,
            favicon: None,
            name,
            subscriptions_enabled: None,
            type_,
            visualization_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn company_logo(mut self, value: String) -> Self {
        self.company_logo = Some(value);
        self
    }

    pub fn components(
        mut self,
        value: Vec<crate::datadogV2::model::CreateStatusPageRequestDataAttributesComponentsItems>,
    ) -> Self {
        self.components = Some(value);
        self
    }

    pub fn email_header_image(mut self, value: String) -> Self {
        self.email_header_image = Some(value);
        self
    }

    pub fn favicon(mut self, value: String) -> Self {
        self.favicon = Some(value);
        self
    }

    pub fn subscriptions_enabled(mut self, value: bool) -> Self {
        self.subscriptions_enabled = Some(value);
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

impl<'de> Deserialize<'de> for CreateStatusPageRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateStatusPageRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateStatusPageRequestDataAttributesVisitor {
            type Value = CreateStatusPageRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut company_logo: Option<String> = None;
                let mut components: Option<Vec<crate::datadogV2::model::CreateStatusPageRequestDataAttributesComponentsItems>> = None;
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
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain_prefix" => {
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
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "favicon" => {
                            if v.is_null() {
                                continue;
                            }
                            favicon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
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
                let domain_prefix =
                    domain_prefix.ok_or_else(|| M::Error::missing_field("domain_prefix"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let visualization_type = visualization_type
                    .ok_or_else(|| M::Error::missing_field("visualization_type"))?;

                let content = CreateStatusPageRequestDataAttributes {
                    company_logo,
                    components,
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

        deserializer.deserialize_any(CreateStatusPageRequestDataAttributesVisitor)
    }
}
