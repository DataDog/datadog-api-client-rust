// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an action connection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConnectionDataAttributesResponse {
    /// Indicates if the acting user can resolve the connection.
    #[serde(rename = "acting_user_can_resolve")]
    pub acting_user_can_resolve: bool,
    /// The creation timestamp of the connection.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Information about the user who created the resource.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::CreatedBy>,
    /// The description of the connection.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// External secrets manager configuration.
    #[serde(rename = "external_secrets_manager")]
    pub external_secrets_manager: crate::datadogV2::model::ExternalSecretsManager,
    /// Integration configuration details.
    #[serde(rename = "integration")]
    pub integration: std::collections::BTreeMap<String, serde_json::Value>,
    /// Indicates if the connection is marked as favorite.
    #[serde(rename = "is_favorite")]
    pub is_favorite: bool,
    /// The last updated timestamp of the connection.
    #[serde(rename = "last_updated")]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the connection.
    #[serde(rename = "name")]
    pub name: String,
    /// Information about the private actions runner.
    #[serde(rename = "private_actions_runner")]
    pub private_actions_runner: Option<crate::datadogV2::model::PrivateActionsRunner>,
    /// The ID of the runner associated with the connection.
    #[serde(rename = "runner_id")]
    pub runner_id: Option<String>,
    /// Tags associated with the connection.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConnectionDataAttributesResponse {
    pub fn new(
        acting_user_can_resolve: bool,
        external_secrets_manager: crate::datadogV2::model::ExternalSecretsManager,
        integration: std::collections::BTreeMap<String, serde_json::Value>,
        is_favorite: bool,
        name: String,
    ) -> ConnectionDataAttributesResponse {
        ConnectionDataAttributesResponse {
            acting_user_can_resolve,
            created_at: None,
            created_by: None,
            description: None,
            external_secrets_manager,
            integration,
            is_favorite,
            last_updated: None,
            name,
            private_actions_runner: None,
            runner_id: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: crate::datadogV2::model::CreatedBy) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn last_updated(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn private_actions_runner(
        mut self,
        value: crate::datadogV2::model::PrivateActionsRunner,
    ) -> Self {
        self.private_actions_runner = Some(value);
        self
    }

    pub fn runner_id(mut self, value: String) -> Self {
        self.runner_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for ConnectionDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for ConnectionDataAttributesResponseVisitor {
            type Value = ConnectionDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut acting_user_can_resolve: Option<bool> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::CreatedBy> = None;
                let mut description: Option<String> = None;
                let mut external_secrets_manager: Option<
                    crate::datadogV2::model::ExternalSecretsManager,
                > = None;
                let mut integration: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut is_favorite: Option<bool> = None;
                let mut last_updated: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut private_actions_runner: Option<
                    crate::datadogV2::model::PrivateActionsRunner,
                > = None;
                let mut runner_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "acting_user_can_resolve" => {
                            acting_user_can_resolve =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_secrets_manager" => {
                            external_secrets_manager =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _external_secrets_manager) = external_secrets_manager {
                                match _external_secrets_manager {
                                    crate::datadogV2::model::ExternalSecretsManager::UnparsedObject(_external_secrets_manager) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "integration" => {
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorite" => {
                            is_favorite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_actions_runner" => {
                            if v.is_null() {
                                continue;
                            }
                            private_actions_runner =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_id" => {
                            if v.is_null() {
                                continue;
                            }
                            runner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let acting_user_can_resolve = acting_user_can_resolve
                    .ok_or_else(|| M::Error::missing_field("acting_user_can_resolve"))?;
                let external_secrets_manager = external_secrets_manager
                    .ok_or_else(|| M::Error::missing_field("external_secrets_manager"))?;
                let integration =
                    integration.ok_or_else(|| M::Error::missing_field("integration"))?;
                let is_favorite =
                    is_favorite.ok_or_else(|| M::Error::missing_field("is_favorite"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ConnectionDataAttributesResponse {
                    acting_user_can_resolve,
                    created_at,
                    created_by,
                    description,
                    external_secrets_manager,
                    integration,
                    is_favorite,
                    last_updated,
                    name,
                    private_actions_runner,
                    runner_id,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConnectionDataAttributesResponseVisitor)
    }
}
