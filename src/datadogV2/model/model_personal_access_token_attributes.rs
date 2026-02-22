// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a personal access token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PersonalAccessTokenAttributes {
    /// Creation timestamp of the personal access token.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Expiration timestamp of the personal access token.
    #[serde(rename = "expires_at")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Last modification timestamp of the personal access token.
    #[serde(
        rename = "modified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Name of the personal access token.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of scopes granted to the personal access token. These define what
    /// permissions the token has.
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PersonalAccessTokenAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        expires_at: chrono::DateTime<chrono::Utc>,
        name: String,
        scopes: Vec<String>,
    ) -> PersonalAccessTokenAttributes {
        PersonalAccessTokenAttributes {
            created_at,
            expires_at,
            modified_at: None,
            name,
            scopes,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn modified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.modified_at = Some(value);
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

impl<'de> Deserialize<'de> for PersonalAccessTokenAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PersonalAccessTokenAttributesVisitor;
        impl<'a> Visitor<'a> for PersonalAccessTokenAttributesVisitor {
            type Value = PersonalAccessTokenAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut expires_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut name: Option<String> = None;
                let mut scopes: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let expires_at = expires_at.ok_or_else(|| M::Error::missing_field("expires_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let scopes = scopes.ok_or_else(|| M::Error::missing_field("scopes"))?;

                let content = PersonalAccessTokenAttributes {
                    created_at,
                    expires_at,
                    modified_at,
                    name,
                    scopes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PersonalAccessTokenAttributesVisitor)
    }
}
