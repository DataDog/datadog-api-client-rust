// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Private Action Runner authentication method configured on the account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAccountPrivateActionRunnerAuthResponse {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthType,
    /// Unique identifier of the Private Action Runner connection holding the credentials.
    #[serde(rename = "connection_id")]
    pub connection_id: String,
    /// Path of the credential inside the secret backend configured on the runner.
    #[serde(rename = "secret_path")]
    pub secret_path: Option<String>,
    /// Unique identifier of the user the Private Action Runner connection belongs to.
    #[serde(rename = "user_uuid")]
    pub user_uuid: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAccountPrivateActionRunnerAuthResponse {
    pub fn new(
        auth_type: crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthType,
        connection_id: String,
        user_uuid: String,
    ) -> IntegrationAccountPrivateActionRunnerAuthResponse {
        IntegrationAccountPrivateActionRunnerAuthResponse {
            auth_type,
            connection_id,
            secret_path: None,
            user_uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn secret_path(mut self, value: String) -> Self {
        self.secret_path = Some(value);
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

impl<'de> Deserialize<'de> for IntegrationAccountPrivateActionRunnerAuthResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAccountPrivateActionRunnerAuthResponseVisitor;
        impl<'a> Visitor<'a> for IntegrationAccountPrivateActionRunnerAuthResponseVisitor {
            type Value = IntegrationAccountPrivateActionRunnerAuthResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthType,
                > = None;
                let mut connection_id: Option<String> = None;
                let mut secret_path: Option<String> = None;
                let mut user_uuid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "connection_id" => {
                            connection_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_path" => {
                            if v.is_null() {
                                continue;
                            }
                            secret_path =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_uuid" => {
                            user_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let connection_id =
                    connection_id.ok_or_else(|| M::Error::missing_field("connection_id"))?;
                let user_uuid = user_uuid.ok_or_else(|| M::Error::missing_field("user_uuid"))?;

                let content = IntegrationAccountPrivateActionRunnerAuthResponse {
                    auth_type,
                    connection_id,
                    secret_path,
                    user_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAccountPrivateActionRunnerAuthResponseVisitor)
    }
}
