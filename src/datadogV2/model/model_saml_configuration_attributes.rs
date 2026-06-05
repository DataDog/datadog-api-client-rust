// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a SAML configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SAMLConfigurationAttributes {
    /// The assertion consumer service (ACS) URLs that the identity provider posts SAML responses to.
    #[serde(rename = "assertion_consumer_service")]
    pub assertion_consumer_service: Option<Vec<String>>,
    /// Creation time of the SAML configuration.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The service provider entity ID Datadog presents to the identity provider.
    #[serde(rename = "entity_id")]
    pub entity_id: Option<String>,
    /// Expiration time of the uploaded identity provider metadata.
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expires_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Whether identity-provider-initiated login is enabled for the organization.
    #[serde(rename = "idp_initiated")]
    pub idp_initiated: Option<bool>,
    /// Email domains for which users are automatically provisioned on first SAML login
    /// (just-in-time provisioning).
    #[serde(rename = "jit_domains")]
    pub jit_domains: Option<Vec<String>>,
    /// Time of the last SAML configuration modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The single sign-on URL users can visit to start a SAML login.
    /// Returns `null` when the organization is identity-provider-initiated and has no subdomain.
    #[serde(
        rename = "sso_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub sso_url: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SAMLConfigurationAttributes {
    pub fn new() -> SAMLConfigurationAttributes {
        SAMLConfigurationAttributes {
            assertion_consumer_service: None,
            created_at: None,
            entity_id: None,
            expires_at: None,
            idp_initiated: None,
            jit_domains: None,
            modified_at: None,
            sso_url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assertion_consumer_service(mut self, value: Vec<String>) -> Self {
        self.assertion_consumer_service = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn entity_id(mut self, value: String) -> Self {
        self.entity_id = Some(value);
        self
    }

    pub fn expires_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn idp_initiated(mut self, value: bool) -> Self {
        self.idp_initiated = Some(value);
        self
    }

    pub fn jit_domains(mut self, value: Vec<String>) -> Self {
        self.jit_domains = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn sso_url(mut self, value: Option<String>) -> Self {
        self.sso_url = Some(value);
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

impl Default for SAMLConfigurationAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SAMLConfigurationAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SAMLConfigurationAttributesVisitor;
        impl<'a> Visitor<'a> for SAMLConfigurationAttributesVisitor {
            type Value = SAMLConfigurationAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assertion_consumer_service: Option<Vec<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut entity_id: Option<String> = None;
                let mut expires_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut idp_initiated: Option<bool> = None;
                let mut jit_domains: Option<Vec<String>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut sso_url: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assertion_consumer_service" => {
                            if v.is_null() {
                                continue;
                            }
                            assertion_consumer_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_id" => {
                            if v.is_null() {
                                continue;
                            }
                            entity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "idp_initiated" => {
                            if v.is_null() {
                                continue;
                            }
                            idp_initiated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jit_domains" => {
                            if v.is_null() {
                                continue;
                            }
                            jit_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sso_url" => {
                            sso_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SAMLConfigurationAttributes {
                    assertion_consumer_service,
                    created_at,
                    entity_id,
                    expires_at,
                    idp_initiated,
                    jit_domains,
                    modified_at,
                    sso_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SAMLConfigurationAttributesVisitor)
    }
}
