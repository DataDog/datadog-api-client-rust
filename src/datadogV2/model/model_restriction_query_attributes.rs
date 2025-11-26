// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the restriction query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RestrictionQueryAttributes {
    /// Creation time of the restriction query.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Email of the user who last modified this restriction query.
    #[serde(rename = "last_modifier_email")]
    pub last_modifier_email: Option<String>,
    /// Name of the user who last modified this restriction query.
    #[serde(rename = "last_modifier_name")]
    pub last_modifier_name: Option<String>,
    /// Time of last restriction query modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The query that defines the restriction. Only the content matching the query can be returned.
    #[serde(rename = "restriction_query")]
    pub restriction_query: Option<String>,
    /// Number of roles associated with this restriction query.
    #[serde(rename = "role_count")]
    pub role_count: Option<i64>,
    /// Number of users associated with this restriction query.
    #[serde(rename = "user_count")]
    pub user_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RestrictionQueryAttributes {
    pub fn new() -> RestrictionQueryAttributes {
        RestrictionQueryAttributes {
            created_at: None,
            last_modifier_email: None,
            last_modifier_name: None,
            modified_at: None,
            restriction_query: None,
            role_count: None,
            user_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last_modifier_email(mut self, value: String) -> Self {
        self.last_modifier_email = Some(value);
        self
    }

    pub fn last_modifier_name(mut self, value: String) -> Self {
        self.last_modifier_name = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn restriction_query(mut self, value: String) -> Self {
        self.restriction_query = Some(value);
        self
    }

    pub fn role_count(mut self, value: i64) -> Self {
        self.role_count = Some(value);
        self
    }

    pub fn user_count(mut self, value: i64) -> Self {
        self.user_count = Some(value);
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

impl Default for RestrictionQueryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RestrictionQueryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RestrictionQueryAttributesVisitor;
        impl<'a> Visitor<'a> for RestrictionQueryAttributesVisitor {
            type Value = RestrictionQueryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_modifier_email: Option<String> = None;
                let mut last_modifier_name: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut restriction_query: Option<String> = None;
                let mut role_count: Option<i64> = None;
                let mut user_count: Option<i64> = None;
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
                        "last_modifier_email" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modifier_email =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modifier_name" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modifier_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "restriction_query" => {
                            if v.is_null() {
                                continue;
                            }
                            restriction_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role_count" => {
                            if v.is_null() {
                                continue;
                            }
                            role_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_count" => {
                            if v.is_null() {
                                continue;
                            }
                            user_count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RestrictionQueryAttributes {
                    created_at,
                    last_modifier_email,
                    last_modifier_name,
                    modified_at,
                    restriction_query,
                    role_count,
                    user_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RestrictionQueryAttributesVisitor)
    }
}
