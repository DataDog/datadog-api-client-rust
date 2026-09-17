// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Username and password authentication. Only the fields provided are changed; omit `password` to keep the stored one.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudIntegrationAccountBasicAuthUpdate {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::ElasticCloudIntegrationAccountBasicAuthType,
    /// Secret password or private key.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// Non-secret username or public identifier for the credential pair.
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudIntegrationAccountBasicAuthUpdate {
    pub fn new(
        auth_type: crate::datadogV2::model::ElasticCloudIntegrationAccountBasicAuthType,
    ) -> ElasticCloudIntegrationAccountBasicAuthUpdate {
        ElasticCloudIntegrationAccountBasicAuthUpdate {
            auth_type,
            password: None,
            username: None,
            _unparsed: false,
        }
    }

    pub fn password(mut self, value: String) -> Self {
        self.password = Some(value);
        self
    }

    pub fn username(mut self, value: String) -> Self {
        self.username = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ElasticCloudIntegrationAccountBasicAuthUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudIntegrationAccountBasicAuthUpdateVisitor;
        impl<'a> Visitor<'a> for ElasticCloudIntegrationAccountBasicAuthUpdateVisitor {
            type Value = ElasticCloudIntegrationAccountBasicAuthUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::ElasticCloudIntegrationAccountBasicAuthType,
                > = None;
                let mut password: Option<String> = None;
                let mut username: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::ElasticCloudIntegrationAccountBasicAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "password" => {
                            if v.is_null() {
                                continue;
                            }
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            if v.is_null() {
                                continue;
                            }
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;

                let content = ElasticCloudIntegrationAccountBasicAuthUpdate {
                    auth_type,
                    password,
                    username,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudIntegrationAccountBasicAuthUpdateVisitor)
    }
}
