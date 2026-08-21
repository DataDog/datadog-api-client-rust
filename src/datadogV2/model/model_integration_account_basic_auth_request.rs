// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Username and password authentication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationAccountBasicAuthRequest {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::IntegrationAccountBasicAuthType,
    /// Secret password or private key.
    #[serde(rename = "password")]
    pub password: String,
    /// Non-secret username or public identifier for the credential pair.
    #[serde(rename = "username")]
    pub username: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationAccountBasicAuthRequest {
    pub fn new(
        auth_type: crate::datadogV2::model::IntegrationAccountBasicAuthType,
        password: String,
        username: String,
    ) -> IntegrationAccountBasicAuthRequest {
        IntegrationAccountBasicAuthRequest {
            auth_type,
            password,
            username,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IntegrationAccountBasicAuthRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationAccountBasicAuthRequestVisitor;
        impl<'a> Visitor<'a> for IntegrationAccountBasicAuthRequestVisitor {
            type Value = IntegrationAccountBasicAuthRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::IntegrationAccountBasicAuthType,
                > = None;
                let mut password: Option<String> = None;
                let mut username: Option<String> = None;
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
                                    crate::datadogV2::model::IntegrationAccountBasicAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "password" => {
                            password = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "username" => {
                            username = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auth_type = auth_type.ok_or_else(|| M::Error::missing_field("auth_type"))?;
                let password = password.ok_or_else(|| M::Error::missing_field("password"))?;
                let username = username.ok_or_else(|| M::Error::missing_field("username"))?;

                let content = IntegrationAccountBasicAuthRequest {
                    auth_type,
                    password,
                    username,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationAccountBasicAuthRequestVisitor)
    }
}
