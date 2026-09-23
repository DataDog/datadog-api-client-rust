// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Bearer token authentication. The credential is a single opaque secret, a Databricks personal access token, with no accompanying non-secret identifier. The method is deprecated: Databricks accepts it only on accounts that already use it, and never on creation. Only the fields provided are changed; omit `token` to keep the stored one.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatabricksIntegrationAccountBearerTokenAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthType,
    /// Secret token used to authenticate with Databricks.
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatabricksIntegrationAccountBearerTokenAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthType,
    ) -> DatabricksIntegrationAccountBearerTokenAuthUpdate {
        DatabricksIntegrationAccountBearerTokenAuthUpdate {
            auth_type,
            token: None,
            _unparsed: false,
        }
    }

    pub fn token(mut self, value: String) -> Self {
        self.token = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountBearerTokenAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabricksIntegrationAccountBearerTokenAuthUpdateVisitor;
        impl<'a> Visitor<'a> for DatabricksIntegrationAccountBearerTokenAuthUpdateVisitor {
            type Value = DatabricksIntegrationAccountBearerTokenAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthType,
                > = None;
                let mut token: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "token" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = DatabricksIntegrationAccountBearerTokenAuthUpdate {
                    auth_type,
                    token,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatabricksIntegrationAccountBearerTokenAuthUpdateVisitor)
    }
}
