// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an upsert OAuth2 scopes restriction request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertOAuthScopesRestrictionDataAttributes {
    /// OIDC scopes the client is allowed to request.
    #[serde(rename = "oidc_scopes")]
    pub oidc_scopes: Option<Vec<crate::datadogV2::model::OAuthOidcScope>>,
    /// Datadog permission scopes the client is allowed to request.
    /// Each value must be a valid permission name.
    #[serde(rename = "permission_scopes")]
    pub permission_scopes: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertOAuthScopesRestrictionDataAttributes {
    pub fn new() -> UpsertOAuthScopesRestrictionDataAttributes {
        UpsertOAuthScopesRestrictionDataAttributes {
            oidc_scopes: None,
            permission_scopes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn oidc_scopes(mut self, value: Vec<crate::datadogV2::model::OAuthOidcScope>) -> Self {
        self.oidc_scopes = Some(value);
        self
    }

    pub fn permission_scopes(mut self, value: Vec<String>) -> Self {
        self.permission_scopes = Some(value);
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

impl Default for UpsertOAuthScopesRestrictionDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpsertOAuthScopesRestrictionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertOAuthScopesRestrictionDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpsertOAuthScopesRestrictionDataAttributesVisitor {
            type Value = UpsertOAuthScopesRestrictionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut oidc_scopes: Option<Vec<crate::datadogV2::model::OAuthOidcScope>> = None;
                let mut permission_scopes: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "oidc_scopes" => {
                            if v.is_null() {
                                continue;
                            }
                            oidc_scopes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permission_scopes" => {
                            if v.is_null() {
                                continue;
                            }
                            permission_scopes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpsertOAuthScopesRestrictionDataAttributes {
                    oidc_scopes,
                    permission_scopes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertOAuthScopesRestrictionDataAttributesVisitor)
    }
}
