// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for updating an Okta account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OktaAccountUpdateRequestAttributes {
    /// The API key of the Okta account.
    #[serde(rename = "api_key")]
    pub api_key: Option<String>,
    /// The authorization method for an Okta account.
    #[serde(rename = "auth_method")]
    pub auth_method: String,
    /// The Client ID of an Okta app integration.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// The client secret of an Okta app integration.
    #[serde(rename = "client_secret")]
    pub client_secret: Option<String>,
    /// The domain associated with an Okta account.
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OktaAccountUpdateRequestAttributes {
    pub fn new(auth_method: String, domain: String) -> OktaAccountUpdateRequestAttributes {
        OktaAccountUpdateRequestAttributes {
            api_key: None,
            auth_method,
            client_id: None,
            client_secret: None,
            domain,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn api_key(mut self, value: String) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn client_id(mut self, value: String) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn client_secret(mut self, value: String) -> Self {
        self.client_secret = Some(value);
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

impl<'de> Deserialize<'de> for OktaAccountUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OktaAccountUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for OktaAccountUpdateRequestAttributesVisitor {
            type Value = OktaAccountUpdateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut api_key: Option<String> = None;
                let mut auth_method: Option<String> = None;
                let mut client_id: Option<String> = None;
                let mut client_secret: Option<String> = None;
                let mut domain: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "api_key" => {
                            if v.is_null() {
                                continue;
                            }
                            api_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "auth_method" => {
                            auth_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            if v.is_null() {
                                continue;
                            }
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domain" => {
                            domain = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_method =
                    auth_method.ok_or_else(|| M::Error::missing_field("auth_method"))?;
                let domain = domain.ok_or_else(|| M::Error::missing_field("domain"))?;

                let content = OktaAccountUpdateRequestAttributes {
                    api_key,
                    auth_method,
                    client_id,
                    client_secret,
                    domain,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OktaAccountUpdateRequestAttributesVisitor)
    }
}
