// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Custom OIDC provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCCloudAuthProviderCreateAttributes {
    /// Expected token audiences. At least one audience is required. When `org_uuid_source` is `aud`, include datadog/{org_uuid} for the organization creating the provider.
    #[serde(rename = "audiences")]
    pub audiences: Vec<String>,
    /// How the signing keys are discovered.
    #[serde(rename = "discovery_type")]
    pub discovery_type: crate::datadogV2::model::OIDCCloudAuthDiscoveryType,
    /// Claim used by legacy identity_pattern mappings and in identity logs.
    #[serde(rename = "identity_claim_name")]
    pub identity_claim_name: Option<String>,
    /// HTTPS issuer URL that must match the token issuer. For oidc_discovery, signing keys are located through the issuer discovery document.
    #[serde(rename = "issuer_url")]
    pub issuer_url: String,
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
    pub provider_name: String,
    /// Claim matchers required for every token from this provider, in addition to the persona mapping matchers. Shared issuers can require an exact tenant claim.
    #[serde(rename = "required_claims")]
    pub required_claims: Option<Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCCloudAuthProviderCreateAttributes {
    pub fn new(
        audiences: Vec<String>,
        discovery_type: crate::datadogV2::model::OIDCCloudAuthDiscoveryType,
        issuer_url: String,
        provider_name: String,
    ) -> OIDCCloudAuthProviderCreateAttributes {
        OIDCCloudAuthProviderCreateAttributes {
            audiences,
            discovery_type,
            identity_claim_name: None,
            issuer_url,
            jwks_url: None,
            org_uuid_claim_name: None,
            org_uuid_source: None,
            provider_name,
            required_claims: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn identity_claim_name(mut self, value: String) -> Self {
        self.identity_claim_name = Some(value);
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

    pub fn required_claims(
        mut self,
        value: Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>,
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

impl<'de> Deserialize<'de> for OIDCCloudAuthProviderCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCCloudAuthProviderCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OIDCCloudAuthProviderCreateAttributesVisitor {
            type Value = OIDCCloudAuthProviderCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audiences: Option<Vec<String>> = None;
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
                    Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audiences" => {
                            audiences = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "discovery_type" => {
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
                let audiences = audiences.ok_or_else(|| M::Error::missing_field("audiences"))?;
                let discovery_type =
                    discovery_type.ok_or_else(|| M::Error::missing_field("discovery_type"))?;
                let issuer_url = issuer_url.ok_or_else(|| M::Error::missing_field("issuer_url"))?;
                let provider_name =
                    provider_name.ok_or_else(|| M::Error::missing_field("provider_name"))?;

                let content = OIDCCloudAuthProviderCreateAttributes {
                    audiences,
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

        deserializer.deserialize_any(OIDCCloudAuthProviderCreateAttributesVisitor)
    }
}
