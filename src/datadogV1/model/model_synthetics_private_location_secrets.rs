// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Secrets for the private location. Only present in the response when creating the private location.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsPrivateLocationSecrets {
    /// Authentication part of the secrets.
    #[serde(rename = "authentication")]
    pub authentication:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationSecretsAuthentication>,
    /// Private key for the private location.
    #[serde(rename = "config_decryption")]
    pub config_decryption:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationSecretsConfigDecryption>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsPrivateLocationSecrets {
    pub fn new() -> SyntheticsPrivateLocationSecrets {
        SyntheticsPrivateLocationSecrets {
            authentication: None,
            config_decryption: None,
            _unparsed: false,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationSecretsAuthentication,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn config_decryption(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationSecretsConfigDecryption,
    ) -> Self {
        self.config_decryption = Some(value);
        self
    }
}

impl Default for SyntheticsPrivateLocationSecrets {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsPrivateLocationSecrets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsPrivateLocationSecretsVisitor;
        impl<'a> Visitor<'a> for SyntheticsPrivateLocationSecretsVisitor {
            type Value = SyntheticsPrivateLocationSecrets;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut authentication: Option<
                    crate::datadogV1::model::SyntheticsPrivateLocationSecretsAuthentication,
                > = None;
                let mut config_decryption: Option<
                    crate::datadogV1::model::SyntheticsPrivateLocationSecretsConfigDecryption,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "authentication" => {
                            if v.is_null() {
                                continue;
                            }
                            authentication =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_decryption" => {
                            if v.is_null() {
                                continue;
                            }
                            config_decryption =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsPrivateLocationSecrets {
                    authentication,
                    config_decryption,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsPrivateLocationSecretsVisitor)
    }
}
