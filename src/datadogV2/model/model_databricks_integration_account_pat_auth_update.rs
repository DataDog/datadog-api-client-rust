// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Databricks personal access token authentication. Deprecated: accepted only on accounts that already use it, and never on creation. Use `databricks-oauth` or `private-action-runner` instead. An update replaces this object as a whole, so `token` must be sent with it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountPatAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
    /// Secret Databricks personal access token.
    #[serde(rename = "token")]
    pub token: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountPatAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
        token: String,
    ) -> DatabricksIntegrationAccountPatAuthUpdate {
        DatabricksIntegrationAccountPatAuthUpdate {
            auth_type,
            token,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountPatAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountPatAuthUpdateVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountPatAuthUpdateVisitor {
            type Value = DatabricksIntegrationAccountPatAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType,
                > = None;
                let mut token: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::DatabricksIntegrationAccountPatAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "token" => {
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let token = token.ok_or_else(|| M::Error::missing_field("token"))?;

                let content = DatabricksIntegrationAccountPatAuthUpdate {
                    auth_type,
                    token,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationAccountPatAuthUpdateVisitor)
    }
}
