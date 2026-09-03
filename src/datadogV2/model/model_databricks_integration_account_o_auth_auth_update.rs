// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Databricks OAuth machine-to-machine authentication using a service principal. Omit `client_secret` to keep the stored one; `client_id` must be provided on every submission. Omitting `azure_tenant_id` clears it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountOAuthAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType,
    /// Microsoft Entra ID tenant of the service principal, for Azure Databricks workspaces.
    #[serde(rename = "azure_tenant_id")]
    pub azure_tenant_id: Option<String>,
    /// Client ID of the Databricks service principal.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Secret of the Databricks service principal.
    #[serde(rename = "client_secret")]
    pub client_secret: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountOAuthAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType,
        client_id: String,
    ) -> DatabricksIntegrationAccountOAuthAuthUpdate {
        DatabricksIntegrationAccountOAuthAuthUpdate {
            auth_type,
            azure_tenant_id: None,
            client_id,
            client_secret: None,
            _unparsed: false,
        }
    }

    pub fn azure_tenant_id(mut self, value: String) -> Self {
        self.azure_tenant_id = Some(value);
        self
    }

    pub fn client_secret(mut self, value: String) -> Self {
        self.client_secret = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountOAuthAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountOAuthAuthUpdateVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountOAuthAuthUpdateVisitor {
            type Value = DatabricksIntegrationAccountOAuthAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType,
                > = None;
                let mut azure_tenant_id: Option<String> = None;
                let mut client_id: Option<String> = None;
                let mut client_secret: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "azure_tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;

                let content = DatabricksIntegrationAccountOAuthAuthUpdate {
                    auth_type,
                    azure_tenant_id,
                    client_id,
                    client_secret,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationAccountOAuthAuthUpdateVisitor)
    }
}
