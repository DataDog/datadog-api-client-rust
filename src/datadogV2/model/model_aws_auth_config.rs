// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Authentication config
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSAuthConfig {
    /// AWS Access Key ID
    #[serde(rename = "access_key_id")]
    pub access_key_id: Option<String>,
    /// AWS IAM External ID for associated role
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
    /// AWS IAM Role name
    #[serde(rename = "role_name")]
    pub role_name: Option<String>,
    /// AWS Secret Access Key
    #[serde(rename = "secret_access_key")]
    pub secret_access_key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSAuthConfig {
    pub fn new() -> AWSAuthConfig {
        AWSAuthConfig {
            access_key_id: None,
            external_id: None,
            role_name: None,
            secret_access_key: None,
            _unparsed: false,
        }
    }

    pub fn access_key_id(mut self, value: String) -> Self {
        self.access_key_id = Some(value);
        self
    }

    pub fn external_id(mut self, value: String) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn role_name(mut self, value: String) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn secret_access_key(mut self, value: String) -> Self {
        self.secret_access_key = Some(value);
        self
    }
}

impl Default for AWSAuthConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSAuthConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSAuthConfigVisitor;
        impl<'a> Visitor<'a> for AWSAuthConfigVisitor {
            type Value = AWSAuthConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key_id: Option<String> = None;
                let mut external_id: Option<String> = None;
                let mut role_name: Option<String> = None;
                let mut secret_access_key: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "access_key_id" => {
                            if v.is_null() {
                                continue;
                            }
                            access_key_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_id" => {
                            if v.is_null() {
                                continue;
                            }
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "role_name" => {
                            if v.is_null() {
                                continue;
                            }
                            role_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secret_access_key" => {
                            if v.is_null() {
                                continue;
                            }
                            secret_access_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSAuthConfig {
                    access_key_id,
                    external_id,
                    role_name,
                    secret_access_key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSAuthConfigVisitor)
    }
}
