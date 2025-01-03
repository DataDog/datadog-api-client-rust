// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `DeploymentMeta` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeploymentMeta {
    /// The `DeploymentMeta` `created_at`.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The `DeploymentMeta` `user_id`.
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
    /// The `DeploymentMeta` `user_name`.
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    /// The `DeploymentMeta` `user_uuid`.
    #[serde(rename = "user_uuid")]
    pub user_uuid: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DeploymentMeta {
    pub fn new() -> DeploymentMeta {
        DeploymentMeta {
            created_at: None,
            user_id: None,
            user_name: None,
            user_uuid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn user_name(mut self, value: String) -> Self {
        self.user_name = Some(value);
        self
    }

    pub fn user_uuid(mut self, value: uuid::Uuid) -> Self {
        self.user_uuid = Some(value);
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

impl Default for DeploymentMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DeploymentMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeploymentMetaVisitor;
        impl<'a> Visitor<'a> for DeploymentMetaVisitor {
            type Value = DeploymentMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut user_id: Option<i64> = None;
                let mut user_name: Option<String> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
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
                        "user_id" => {
                            if v.is_null() {
                                continue;
                            }
                            user_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_name" => {
                            if v.is_null() {
                                continue;
                            }
                            user_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DeploymentMeta {
                    created_at,
                    user_id,
                    user_name,
                    user_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DeploymentMetaVisitor)
    }
}
