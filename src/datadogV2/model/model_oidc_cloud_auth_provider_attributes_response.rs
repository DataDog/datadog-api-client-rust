// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Stored Custom OIDC provider attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCCloudAuthProviderAttributesResponse {
    /// Expected token audiences. At least one audience is required. When `org_uuid_source` is `aud`, include datadog/{org_uuid} for the organization creating the provider.
    #[serde(rename = "audiences")]
    pub audiences: Option<Vec<String>>,
    /// Time the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who created the configuration.
    #[serde(rename = "created_by")]
    pub created_by: Option<uuid::Uuid>,
    /// How the signing keys are discovered.
    #[serde(rename = "discovery_type")]
    pub discovery_type: Option<crate::datadogV2::model::OIDCCloudAuthDiscoveryType>,
    /// Claim used by legacy identity_pattern mappings and in identity logs.
    #[serde(rename = "identity_claim_name")]
    pub identity_claim_name: Option<String>,
    /// HTTPS issuer URL that must match the token issuer. For oidc_discovery, signing keys are located through the issuer discovery document.
    #[serde(rename = "issuer_url")]
    pub issuer_url: Option<String>,
    /// HTTPS JSON Web Key Set URL. Required when `discovery_type` is `direct_jwks`.
    #[serde(rename = "jwks_url")]
    pub jwks_url: Option<String>,
    /// Optional name of the token claim containing the Datadog organization UUID.
    #[serde(rename = "org_uuid_claim_name")]
    pub org_uuid_claim_name: Option<String>,
    /// Source of the organization UUID in the signed token. The `aud` source accepts a datadog/{org_uuid} audience.
    #[serde(rename = "org_uuid_source")]
    pub org_uuid_source: Option<crate::datadogV2::model::OIDCCloudAuthOrgUUIDSource>,
    /// Name identifying the provider within the organization.
    #[serde(rename = "provider_name")]
    pub provider_name: Option<String>,
    /// Claim matchers required for every token from this provider, in addition to the persona mapping matchers. Shared issuers can require an exact tenant claim.
    #[serde(rename = "required_claims")]
    pub required_claims: Option<Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCCloudAuthProviderAttributesResponse {
    pub fn new() -> OIDCCloudAuthProviderAttributesResponse {
        OIDCCloudAuthProviderAttributesResponse {
            audiences: None,
            created_at: None,
            created_by: None,
            discovery_type: None,
            identity_claim_name: None,
            issuer_url: None,
            jwks_url: None,
            org_uuid_claim_name: None,
            org_uuid_source: None,
            provider_name: None,
            required_claims: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audiences(mut self, value: Vec<String>) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: uuid::Uuid) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn discovery_type(
        mut self,
        value: crate::datadogV2::model::OIDCCloudAuthDiscoveryType,
    ) -> Self {
        self.discovery_type = Some(value);
        self
    }

    pub fn identity_claim_name(mut self, value: String) -> Self {
        self.identity_claim_name = Some(value);
        self
    }

    pub fn issuer_url(mut self, value: String) -> Self {
        self.issuer_url = Some(value);
        self
    }

    pub fn jwks_url(mut self, value: String) -> Self {
        self.jwks_url = Some(value);
        self
    }

    pub fn org_uuid_claim_name(mut self, value: String) -> Self {
        self.org_uuid_claim_name = Some(value);
        self
    }

    pub fn org_uuid_source(
        mut self,
        value: crate::datadogV2::model::OIDCCloudAuthOrgUUIDSource,
    ) -> Self {
        self.org_uuid_source = Some(value);
        self
    }

    pub fn provider_name(mut self, value: String) -> Self {
        self.provider_name = Some(value);
        self
    }

    pub fn required_claims(
        mut self,
        value: Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>,
    ) -> Self {
        self.required_claims = Some(value);
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

impl Default for OIDCCloudAuthProviderAttributesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OIDCCloudAuthProviderAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCCloudAuthProviderAttributesResponseVisitor;
        impl<'a> Visitor<'a> for OIDCCloudAuthProviderAttributesResponseVisitor {
            type Value = OIDCCloudAuthProviderAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audiences: Option<Vec<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut discovery_type: Option<
                    crate::datadogV2::model::OIDCCloudAuthDiscoveryType,
                > = None;
                let mut identity_claim_name: Option<String> = None;
                let mut issuer_url: Option<String> = None;
                let mut jwks_url: Option<String> = None;
                let mut org_uuid_claim_name: Option<String> = None;
                let mut org_uuid_source: Option<
                    crate::datadogV2::model::OIDCCloudAuthOrgUUIDSource,
                > = None;
                let mut provider_name: Option<String> = None;
                let mut required_claims: Option<
                    Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audiences" => {
                            if v.is_null() {
                                continue;
                            }
                            audiences = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "discovery_type" => {
                            if v.is_null() {
                                continue;
                            }
                            discovery_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _discovery_type) = discovery_type {
                                match _discovery_type {
                                    crate::datadogV2::model::OIDCCloudAuthDiscoveryType::UnparsedObject(_discovery_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "identity_claim_name" => {
                            if v.is_null() {
                                continue;
                            }
                            identity_claim_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuer_url" => {
                            if v.is_null() {
                                continue;
                            }
                            issuer_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jwks_url" => {
                            if v.is_null() {
                                continue;
                            }
                            jwks_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_uuid_claim_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_uuid_claim_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_uuid_source" => {
                            if v.is_null() {
                                continue;
                            }
                            org_uuid_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _org_uuid_source) = org_uuid_source {
                                match _org_uuid_source {
                                    crate::datadogV2::model::OIDCCloudAuthOrgUUIDSource::UnparsedObject(_org_uuid_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "provider_name" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "required_claims" => {
                            if v.is_null() {
                                continue;
                            }
                            required_claims =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OIDCCloudAuthProviderAttributesResponse {
                    audiences,
                    created_at,
                    created_by,
                    discovery_type,
                    identity_claim_name,
                    issuer_url,
                    jwks_url,
                    org_uuid_claim_name,
                    org_uuid_source,
                    provider_name,
                    required_claims,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OIDCCloudAuthProviderAttributesResponseVisitor)
    }
}
