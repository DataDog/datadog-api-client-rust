// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// OpenID Connect provider metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCDiscoveryDocument {
    /// URL of the OAuth2 authorization endpoint.
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    /// Signing algorithms supported for ID tokens.
    #[serde(rename = "id_token_signing_alg_values_supported")]
    pub id_token_signing_alg_values_supported: Vec<String>,
    /// URL identifying the OpenID Connect issuer.
    #[serde(rename = "issuer")]
    pub issuer: String,
    /// URL of the JSON Web Key Set used to verify ID token signatures.
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: String,
    /// OAuth2 response types supported by the provider.
    #[serde(rename = "response_types_supported")]
    pub response_types_supported: Vec<String>,
    /// Subject identifier types supported by the provider.
    #[serde(rename = "subject_types_supported")]
    pub subject_types_supported: Vec<String>,
    /// URL of the OAuth2 token endpoint.
    #[serde(rename = "token_endpoint")]
    pub token_endpoint: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCDiscoveryDocument {
    pub fn new(
        authorization_endpoint: String,
        id_token_signing_alg_values_supported: Vec<String>,
        issuer: String,
        jwks_uri: String,
        response_types_supported: Vec<String>,
        subject_types_supported: Vec<String>,
        token_endpoint: String,
    ) -> OIDCDiscoveryDocument {
        OIDCDiscoveryDocument {
            authorization_endpoint,
            id_token_signing_alg_values_supported,
            issuer,
            jwks_uri,
            response_types_supported,
            subject_types_supported,
            token_endpoint,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for OIDCDiscoveryDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCDiscoveryDocumentVisitor;
        impl<'a> Visitor<'a> for OIDCDiscoveryDocumentVisitor {
            type Value = OIDCDiscoveryDocument;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authorization_endpoint: Option<String> = None;
                let mut id_token_signing_alg_values_supported: Option<Vec<String>> = None;
                let mut issuer: Option<String> = None;
                let mut jwks_uri: Option<String> = None;
                let mut response_types_supported: Option<Vec<String>> = None;
                let mut subject_types_supported: Option<Vec<String>> = None;
                let mut token_endpoint: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authorization_endpoint" => {
                            authorization_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id_token_signing_alg_values_supported" => {
                            id_token_signing_alg_values_supported =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuer" => {
                            issuer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jwks_uri" => {
                            jwks_uri = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response_types_supported" => {
                            response_types_supported =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject_types_supported" => {
                            subject_types_supported =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token_endpoint" => {
                            token_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let authorization_endpoint = authorization_endpoint
                    .ok_or_else(|| M::Error::missing_field("authorization_endpoint"))?;
                let id_token_signing_alg_values_supported = id_token_signing_alg_values_supported
                    .ok_or_else(|| {
                    M::Error::missing_field("id_token_signing_alg_values_supported")
                })?;
                let issuer = issuer.ok_or_else(|| M::Error::missing_field("issuer"))?;
                let jwks_uri = jwks_uri.ok_or_else(|| M::Error::missing_field("jwks_uri"))?;
                let response_types_supported = response_types_supported
                    .ok_or_else(|| M::Error::missing_field("response_types_supported"))?;
                let subject_types_supported = subject_types_supported
                    .ok_or_else(|| M::Error::missing_field("subject_types_supported"))?;
                let token_endpoint =
                    token_endpoint.ok_or_else(|| M::Error::missing_field("token_endpoint"))?;

                let content = OIDCDiscoveryDocument {
                    authorization_endpoint,
                    id_token_signing_alg_values_supported,
                    issuer,
                    jwks_uri,
                    response_types_supported,
                    subject_types_supported,
                    token_endpoint,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OIDCDiscoveryDocumentVisitor)
    }
}
