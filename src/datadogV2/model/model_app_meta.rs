// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AppMeta` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AppMeta {
    /// The `AppMeta` `created_at`.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The `AppMeta` `deleted_at`.
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
    /// The `AppMeta` `org_id`.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// The `AppMeta` `run_as_user`.
    #[serde(rename = "run_as_user")]
    pub run_as_user: Option<String>,
    /// The `AppMeta` `updated_at`.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    /// The `AppMeta` `updated_since_deployment`.
    #[serde(rename = "updated_since_deployment")]
    pub updated_since_deployment: Option<bool>,
    /// The `AppMeta` `user_id`.
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,
    /// The `AppMeta` `user_name`.
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    /// The `AppMeta` `user_uuid`.
    #[serde(rename = "user_uuid")]
    pub user_uuid: Option<uuid::Uuid>,
    /// The `AppMeta` `version`.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AppMeta {
    pub fn new() -> AppMeta {
        AppMeta {
            created_at: None,
            deleted_at: None,
            org_id: None,
            run_as_user: None,
            updated_at: None,
            updated_since_deployment: None,
            user_id: None,
            user_name: None,
            user_uuid: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: String) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn run_as_user(mut self, value: String) -> Self {
        self.run_as_user = Some(value);
        self
    }

    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_since_deployment(mut self, value: bool) -> Self {
        self.updated_since_deployment = Some(value);
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

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl Default for AppMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AppMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AppMetaVisitor;
        impl<'a> Visitor<'a> for AppMetaVisitor {
            type Value = AppMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut deleted_at: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut run_as_user: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut updated_since_deployment: Option<bool> = None;
                let mut user_id: Option<i64> = None;
                let mut user_name: Option<String> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut version: Option<i64> = None;
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
                        "deleted_at" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_as_user" => {
                            if v.is_null() {
                                continue;
                            }
                            run_as_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_since_deployment" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_since_deployment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AppMeta {
                    created_at,
                    deleted_at,
                    org_id,
                    run_as_user,
                    updated_at,
                    updated_since_deployment,
                    user_id,
                    user_name,
                    user_uuid,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AppMetaVisitor)
    }
}
