// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Custom OIDC persona mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCCloudAuthPersonaMappingCreateAttributes {
    /// UUID of the Datadog user or service account to authenticate as.
    #[serde(rename = "account_uuid")]
    pub account_uuid: String,
    /// Every claim condition must match. Supply claim_matchers or identity_pattern, but not both.
    #[serde(rename = "claim_matchers")]
    pub claim_matchers: Option<Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>>,
    /// Legacy glob pattern, limited to 500 UTF-8 bytes when creating a mapping, matched against the provider identity claim. Supply this or claim_matchers, but not both. Prefer claim_matchers with exact mode for a single Kubernetes service account.
    #[serde(rename = "identity_pattern")]
    pub identity_pattern: Option<String>,
    /// UUID of the registered Custom OIDC provider.
    #[serde(rename = "provider_id")]
    pub provider_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCCloudAuthPersonaMappingCreateAttributes {
    pub fn new(
        account_uuid: String,
        provider_id: uuid::Uuid,
    ) -> OIDCCloudAuthPersonaMappingCreateAttributes {
        OIDCCloudAuthPersonaMappingCreateAttributes {
            account_uuid,
            claim_matchers: None,
            identity_pattern: None,
            provider_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn claim_matchers(
        mut self,
        value: Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>,
    ) -> Self {
        self.claim_matchers = Some(value);
        self
    }

    pub fn identity_pattern(mut self, value: String) -> Self {
        self.identity_pattern = Some(value);
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

impl<'de> Deserialize<'de> for OIDCCloudAuthPersonaMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCCloudAuthPersonaMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for OIDCCloudAuthPersonaMappingCreateAttributesVisitor {
            type Value = OIDCCloudAuthPersonaMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_uuid: Option<String> = None;
                let mut claim_matchers: Option<
                    Vec<crate::datadogV2::model::OIDCCloudAuthClaimMatcherCreate>,
                > = None;
                let mut identity_pattern: Option<String> = None;
                let mut provider_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_uuid" => {
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
                        "identity_pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            identity_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider_id" => {
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
                let account_uuid =
                    account_uuid.ok_or_else(|| M::Error::missing_field("account_uuid"))?;
                let provider_id =
                    provider_id.ok_or_else(|| M::Error::missing_field("provider_id"))?;

                let content = OIDCCloudAuthPersonaMappingCreateAttributes {
                    account_uuid,
                    claim_matchers,
                    identity_pattern,
                    provider_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OIDCCloudAuthPersonaMappingCreateAttributesVisitor)
    }
}
