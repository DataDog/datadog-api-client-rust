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
    /// Creation date of the personal access token.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Expiration date of the personal access token.
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expires_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Date the personal access token was last used.
    #[serde(
        rename = "last_used_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_used_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Date of last modification of the personal access token.
    #[serde(
        rename = "modified_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Name of the personal access token.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The public portion of the personal access token.
    #[serde(rename = "public_portion")]
    pub public_portion: Option<String>,
    /// Array of scopes granted to the personal access token.
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PersonalAccessTokenAttributes {
    pub fn new() -> PersonalAccessTokenAttributes {
        PersonalAccessTokenAttributes {
            created_at: None,
            expires_at: None,
            last_used_at: None,
            modified_at: None,
            name: None,
            public_portion: None,
            scopes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn last_used_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_used_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn public_portion(mut self, value: String) -> Self {
        self.public_portion = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
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

impl Default for PersonalAccessTokenAttributes {
    fn default() -> Self {
        Self::new()
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
                let mut expires_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut last_used_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut modified_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut name: Option<String> = None;
                let mut public_portion: Option<String> = None;
                let mut scopes: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expires_at" => {
                            expires_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_used_at" => {
                            last_used_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_portion" => {
                            if v.is_null() {
                                continue;
                            }
                            public_portion =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            if v.is_null() {
                                continue;
                            }
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = PersonalAccessTokenAttributes {
                    created_at,
                    expires_at,
                    last_used_at,
                    modified_at,
                    name,
                    public_portion,
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
