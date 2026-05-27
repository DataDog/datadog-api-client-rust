// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Allowlist of OIDC and permission scopes enforced for the OAuth2 client.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OAuthScopesRestriction {
    /// OIDC scopes the client is restricted to.
    #[serde(rename = "oidc_scopes")]
    pub oidc_scopes: Vec<crate::datadogV2::model::OAuthOidcScope>,
    /// Datadog permission scopes the client is restricted to.
    #[serde(rename = "permission_scopes")]
    pub permission_scopes: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OAuthScopesRestriction {
    pub fn new(
        oidc_scopes: Vec<crate::datadogV2::model::OAuthOidcScope>,
        permission_scopes: Vec<String>,
    ) -> OAuthScopesRestriction {
        OAuthScopesRestriction {
            oidc_scopes,
            permission_scopes,
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

impl<'de> Deserialize<'de> for OAuthScopesRestriction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OAuthScopesRestrictionVisitor;
        impl<'a> Visitor<'a> for OAuthScopesRestrictionVisitor {
            type Value = OAuthScopesRestriction;

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
                            oidc_scopes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "permission_scopes" => {
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
                let oidc_scopes =
                    oidc_scopes.ok_or_else(|| M::Error::missing_field("oidc_scopes"))?;
                let permission_scopes = permission_scopes
                    .ok_or_else(|| M::Error::missing_field("permission_scopes"))?;

                let content = OAuthScopesRestriction {
                    oidc_scopes,
                    permission_scopes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OAuthScopesRestrictionVisitor)
    }
}
