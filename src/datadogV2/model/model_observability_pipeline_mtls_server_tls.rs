// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for enabling TLS encryption between the pipeline component and external connecting clients.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineMtlsServerTls {
    /// Path to the Certificate Authority (CA) file used to validate connecting clients' TLS certificates.
    #[serde(rename = "ca_file")]
    pub ca_file: Option<String>,
    /// Path to the TLS server certificate file used to used to identify the pipeline component to connecting clients.
    #[serde(rename = "crt_file")]
    pub crt_file: String,
    /// Path to the private key file associated with the TLS server certificate.
    #[serde(rename = "key_file")]
    pub key_file: Option<String>,
    /// Name of the environment variable or secret that holds the passphrase for the private key file.
    #[serde(rename = "key_pass_key")]
    pub key_pass_key: Option<String>,
    /// When `true`, requires client connections to present a valid certificate, enabling mutual TLS authentication.
    #[serde(rename = "verify_certificate")]
    pub verify_certificate: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineMtlsServerTls {
    pub fn new(crt_file: String) -> ObservabilityPipelineMtlsServerTls {
        ObservabilityPipelineMtlsServerTls {
            ca_file: None,
            crt_file,
            key_file: None,
            key_pass_key: None,
            verify_certificate: None,
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

    pub fn verify_certificate(mut self, value: bool) -> Self {
        self.verify_certificate = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineMtlsServerTls {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineMtlsServerTlsVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineMtlsServerTlsVisitor {
            type Value = ObservabilityPipelineMtlsServerTls;

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
                let mut verify_certificate: Option<bool> = None;
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
                        "verify_certificate" => {
                            if v.is_null() {
                                continue;
                            }
                            verify_certificate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let crt_file = crt_file.ok_or_else(|| M::Error::missing_field("crt_file"))?;

                let content = ObservabilityPipelineMtlsServerTls {
                    ca_file,
                    crt_file,
                    key_file,
                    key_pass_key,
                    verify_certificate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineMtlsServerTlsVisitor)
    }
}
