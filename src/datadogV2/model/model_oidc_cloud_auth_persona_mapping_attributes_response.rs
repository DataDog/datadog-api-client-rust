// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Stored Custom OIDC persona mapping attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCCloudAuthPersonaMappingAttributesResponse {
    /// Handle resolved when the mapping was created; not updated if the account handle changes.
    #[serde(rename = "account_identifier")]
    pub account_identifier: Option<String>,
    /// UUID of the Datadog user or service account to authenticate as.
    #[serde(rename = "account_uuid")]
    pub account_uuid: Option<String>,
    /// Every claim condition must match. Supply claim_matchers or identity_pattern, but not both.
    #[serde(rename = "claim_matchers")]
    pub claim_matchers: Option<Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>>,
    /// Time the configuration was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who created the configuration.
    #[serde(rename = "created_by")]
    pub created_by: Option<uuid::Uuid>,
    /// Legacy glob pattern matched against the provider identity claim. Supply this or claim_matchers, but not both. Prefer claim_matchers with exact mode for a single Kubernetes service account.
    #[serde(rename = "identity_pattern")]
    pub identity_pattern: Option<String>,
    /// UUID of the registered Custom OIDC provider.
    #[serde(rename = "provider_id")]
    pub provider_id: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCCloudAuthPersonaMappingAttributesResponse {
    pub fn new() -> OIDCCloudAuthPersonaMappingAttributesResponse {
        OIDCCloudAuthPersonaMappingAttributesResponse {
            account_identifier: None,
            account_uuid: None,
            claim_matchers: None,
            created_at: None,
            created_by: None,
            identity_pattern: None,
            provider_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_identifier(mut self, value: String) -> Self {
        self.account_identifier = Some(value);
        self
    }

    pub fn account_uuid(mut self, value: String) -> Self {
        self.account_uuid = Some(value);
        self
    }

    pub fn claim_matchers(
        mut self,
        value: Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>,
    ) -> Self {
        self.claim_matchers = Some(value);
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

    pub fn identity_pattern(mut self, value: String) -> Self {
        self.identity_pattern = Some(value);
        self
    }

    pub fn provider_id(mut self, value: uuid::Uuid) -> Self {
        self.provider_id = Some(value);
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

impl Default for OIDCCloudAuthPersonaMappingAttributesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OIDCCloudAuthPersonaMappingAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCCloudAuthPersonaMappingAttributesResponseVisitor;
        impl<'a> Visitor<'a> for OIDCCloudAuthPersonaMappingAttributesResponseVisitor {
            type Value = OIDCCloudAuthPersonaMappingAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_identifier: Option<String> = None;
                let mut account_uuid: Option<String> = None;
                let mut claim_matchers: Option<
                    Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcher>,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut identity_pattern: Option<String> = None;
                let mut provider_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_identifier" => {
                            if v.is_null() {
                                continue;
                            }
                            account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "account_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            account_uuid =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "claim_matchers" => {
                            if v.is_null() {
                                continue;
                            }
                            claim_matchers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "identity_pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            identity_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_id" => {
                            if v.is_null() {
                                continue;
                            }
                            provider_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OIDCCloudAuthPersonaMappingAttributesResponse {
                    account_identifier,
                    account_uuid,
                    claim_matchers,
                    created_at,
                    created_by,
                    identity_pattern,
                    provider_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OIDCCloudAuthPersonaMappingAttributesResponseVisitor)
    }
}
