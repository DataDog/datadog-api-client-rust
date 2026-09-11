// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RSA key pair authentication, the only method Snowflake integration accounts support. Generate an RSA key pair and assign the public key to the Snowflake user named in `settings.username`. Because an update replaces this object as a whole, every required field must be sent again on each update, even when only one of them is changing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeIntegrationAccountAuthenticationRequest {
    /// The authentication method type.
    #[serde(rename = "auth_type")]
    pub auth_type: crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthType,
    /// The private key, in PEM format.
    #[serde(rename = "private_key")]
    pub private_key: String,
    /// Name that distinguishes this private key from other keys in Datadog.
    #[serde(rename = "private_key_name")]
    pub private_key_name: String,
    /// Passphrase that decrypts the private key. Provide it only when the key is encrypted.
    #[serde(rename = "private_key_passphrase")]
    pub private_key_passphrase: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SnowflakeIntegrationAccountAuthenticationRequest {
    pub fn new(
        auth_type: crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthType,
        private_key: String,
        private_key_name: String,
    ) -> SnowflakeIntegrationAccountAuthenticationRequest {
        SnowflakeIntegrationAccountAuthenticationRequest {
            auth_type,
            private_key,
            private_key_name,
            private_key_passphrase: None,
            _unparsed: false,
        }
    }

    pub fn private_key_passphrase(mut self, value: String) -> Self {
        self.private_key_passphrase = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SnowflakeIntegrationAccountAuthenticationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeIntegrationAccountAuthenticationRequestVisitor;
        impl<'a> Visitor<'a> for SnowflakeIntegrationAccountAuthenticationRequestVisitor {
            type Value = SnowflakeIntegrationAccountAuthenticationRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth_type: Option<
                    crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthType,
                > = None;
                let mut private_key: Option<String> = None;
                let mut private_key_name: Option<String> = None;
                let mut private_key_passphrase: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth_type" => {
                            auth_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _auth_type) = auth_type {
                                match _auth_type {
                                    crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthType::UnparsedObject(_auth_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "private_key" => {
                            private_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key_name" => {
                            private_key_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_key_passphrase" => {
                            if v.is_null() {
                                continue;
                            }
                            private_key_passphrase =
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
                let private_key =
                    private_key.ok_or_else(|| M::Error::missing_field("private_key"))?;
                let private_key_name =
                    private_key_name.ok_or_else(|| M::Error::missing_field("private_key_name"))?;

                let content = SnowflakeIntegrationAccountAuthenticationRequest {
                    auth_type,
                    private_key,
                    private_key_name,
                    private_key_passphrase,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeIntegrationAccountAuthenticationRequestVisitor)
    }
}
