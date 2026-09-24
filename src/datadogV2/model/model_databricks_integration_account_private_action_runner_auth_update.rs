// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Private Action Runner authentication. The runner holds the Databricks credentials, so this method carries no secrets. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthType,
    /// Unique identifier of the Private Action Runner connection holding the credentials.
    #[serde(rename = "connection_id")]
    pub connection_id: Option<uuid::Uuid>,
    /// Path of the credential inside the secret backend configured on the runner. Omit it to keep the stored path, send `null` or an empty string to remove it, or send a value to replace it.
    #[serde(
        rename = "secret_path",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub secret_path: Option<Option<String>>,
    /// Unique identifier of the user the Private Action Runner connection belongs to.
    #[serde(rename = "user_uuid")]
    pub user_uuid: Option<uuid::Uuid>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthType,
    ) -> DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
        DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
            auth_type,
            connection_id: None,
            secret_path: None,
            user_uuid: None,
            _unparsed: false,
        }
    }

    pub fn connection_id(mut self, value: uuid::Uuid) -> Self {
        self.connection_id = Some(value);
        self
    }

    pub fn secret_path(mut self, value: Option<String>) -> Self {
        self.secret_path = Some(value);
        self
    }

    pub fn user_uuid(mut self, value: uuid::Uuid) -> Self {
        self.user_uuid = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountPrivateActionRunnerAuthUpdateVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountPrivateActionRunnerAuthUpdateVisitor {
            type Value = DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthType> = None;
                let mut connection_id: Option<uuid::Uuid> = None;
                let mut secret_path: Option<Option<String>> = None;
                let mut user_uuid: Option<uuid::Uuid> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "connection_id" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_path" => {
                            secret_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;

                let content = DatabricksIntegrationAccountPrivateActionRunnerAuthUpdate {
                    auth_type,
                    connection_id,
                    secret_path,
                    user_uuid,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(DatabricksIntegrationAccountPrivateActionRunnerAuthUpdateVisitor)
    }
}
