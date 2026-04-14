// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create a service account access token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceAccountAccessTokenCreateAttributes {
    /// Expiration date of the access token. Optional for service account tokens.
    #[serde(rename = "expires_at")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Name of the access token.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of scopes to grant the access token.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceAccountAccessTokenCreateAttributes {
    pub fn new(name: String, scopes: Vec<String>) -> ServiceAccountAccessTokenCreateAttributes {
        ServiceAccountAccessTokenCreateAttributes {
            expires_at: None,
            name,
            scopes,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expires_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.expires_at = Some(value);
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

impl<'de> Deserialize<'de> for ServiceAccountAccessTokenCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceAccountAccessTokenCreateAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceAccountAccessTokenCreateAttributesVisitor {
            type Value = ServiceAccountAccessTokenCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expires_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut scopes: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expires_at" => {
                            if v.is_null() {
                                continue;
                            }
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let scopes = scopes.ok_or_else(|| M::Error::missing_field("scopes"))?;

                let content = ServiceAccountAccessTokenCreateAttributes {
                    expires_at,
                    name,
                    scopes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceAccountAccessTokenCreateAttributesVisitor)
    }
}
