// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the MCP Cross-App Access issuer URL update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct McpCrossAppAccessIssuerUrlUpdateAttributes {
    /// The Okta OIDC issuer URL for MCP Cross-App Access, for example
    /// `<https://your-subdomain.okta.com`.> Provide an empty string to unset
    /// the issuer URL and opt the organization out of MCP Cross-App Access.
    #[serde(rename = "issuer_url")]
    pub issuer_url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl McpCrossAppAccessIssuerUrlUpdateAttributes {
    pub fn new(issuer_url: String) -> McpCrossAppAccessIssuerUrlUpdateAttributes {
        McpCrossAppAccessIssuerUrlUpdateAttributes {
            issuer_url,
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

impl<'de> Deserialize<'de> for McpCrossAppAccessIssuerUrlUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct McpCrossAppAccessIssuerUrlUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for McpCrossAppAccessIssuerUrlUpdateAttributesVisitor {
            type Value = McpCrossAppAccessIssuerUrlUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut issuer_url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "issuer_url" => {
                            issuer_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let issuer_url = issuer_url.ok_or_else(|| M::Error::missing_field("issuer_url"))?;

                let content = McpCrossAppAccessIssuerUrlUpdateAttributes {
                    issuer_url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(McpCrossAppAccessIssuerUrlUpdateAttributesVisitor)
    }
}
