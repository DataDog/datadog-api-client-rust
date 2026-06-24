// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// TLS configuration that enables encryption and presents a client certificate for mutual TLS authentication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineWebsocketSourceTlsWithClientCert {
    /// Path to the Certificate Authority (CA) file used to validate the remote server's TLS certificate.
    #[serde(rename = "ca_file")]
    pub ca_file: Option<String>,
    /// Path to the TLS client certificate file used to identify this source to the remote server.
    #[serde(rename = "crt_file")]
    pub crt_file: String,
    /// Path to the private key file associated with the client certificate.
    #[serde(rename = "key_file")]
    pub key_file: Option<String>,
    /// Name of the environment variable or secret that holds the passphrase for the private key file.
    #[serde(rename = "key_pass_key")]
    pub key_pass_key: Option<String>,
    /// TLS mode. Must be `with_client_cert`.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCertMode,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineWebsocketSourceTlsWithClientCert {
    pub fn new(
        crt_file: String,
        mode: crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCertMode,
    ) -> ObservabilityPipelineWebsocketSourceTlsWithClientCert {
        ObservabilityPipelineWebsocketSourceTlsWithClientCert {
            ca_file: None,
            crt_file,
            key_file: None,
            key_pass_key: None,
            mode,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ca_file(mut self, value: String) -> Self {
        self.ca_file = Some(value);
        self
    }

    pub fn key_file(mut self, value: String) -> Self {
        self.key_file = Some(value);
        self
    }

    pub fn key_pass_key(mut self, value: String) -> Self {
        self.key_pass_key = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineWebsocketSourceTlsWithClientCert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineWebsocketSourceTlsWithClientCertVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineWebsocketSourceTlsWithClientCertVisitor {
            type Value = ObservabilityPipelineWebsocketSourceTlsWithClientCert;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ca_file: Option<String> = None;
                let mut crt_file: Option<String> = None;
                let mut key_file: Option<String> = None;
                let mut key_pass_key: Option<String> = None;
                let mut mode: Option<crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCertMode> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ca_file" => {
                            if v.is_null() {
                                continue;
                            }
                            ca_file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "crt_file" => {
                            crt_file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_file" => {
                            if v.is_null() {
                                continue;
                            }
                            key_file = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_pass_key" => {
                            if v.is_null() {
                                continue;
                            }
                            key_pass_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::ObservabilityPipelineWebsocketSourceTlsWithClientCertMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let crt_file = crt_file.ok_or_else(|| M::Error::missing_field("crt_file"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = ObservabilityPipelineWebsocketSourceTlsWithClientCert {
                    ca_file,
                    crt_file,
                    key_file,
                    key_pass_key,
                    mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineWebsocketSourceTlsWithClientCertVisitor)
    }
}
