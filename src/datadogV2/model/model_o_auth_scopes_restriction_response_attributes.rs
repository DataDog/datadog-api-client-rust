// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an OAuth2 client scopes restriction.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OAuthScopesRestrictionResponseAttributes {
    /// Permission scopes automatically required for this client (for example, mobile-app permission scopes).
    /// Returns `null` when no scopes are required.
    #[serialize_always]
    #[serde(rename = "required_permission_scopes")]
    pub required_permission_scopes: Option<Vec<String>>,
    /// Allowlist of OIDC and permission scopes enforced for the OAuth2 client.
    #[serialize_always]
    #[serde(rename = "scopes_restriction")]
    pub scopes_restriction: Option<crate::datadogV2::model::OAuthScopesRestriction>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OAuthScopesRestrictionResponseAttributes {
    pub fn new(
        required_permission_scopes: Option<Vec<String>>,
        scopes_restriction: Option<crate::datadogV2::model::OAuthScopesRestriction>,
    ) -> OAuthScopesRestrictionResponseAttributes {
        OAuthScopesRestrictionResponseAttributes {
            required_permission_scopes,
            scopes_restriction,
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

impl<'de> Deserialize<'de> for OAuthScopesRestrictionResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OAuthScopesRestrictionResponseAttributesVisitor;
        impl<'a> Visitor<'a> for OAuthScopesRestrictionResponseAttributesVisitor {
            type Value = OAuthScopesRestrictionResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut required_permission_scopes: Option<Option<Vec<String>>> = None;
                let mut scopes_restriction: Option<
                    Option<crate::datadogV2::model::OAuthScopesRestriction>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "required_permission_scopes" => {
                            required_permission_scopes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes_restriction" => {
                            scopes_restriction =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let required_permission_scopes = required_permission_scopes
                    .ok_or_else(|| M::Error::missing_field("required_permission_scopes"))?;
                let scopes_restriction = scopes_restriction
                    .ok_or_else(|| M::Error::missing_field("scopes_restriction"))?;

                let content = OAuthScopesRestrictionResponseAttributes {
                    required_permission_scopes,
                    scopes_restriction,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OAuthScopesRestrictionResponseAttributesVisitor)
    }
}
