// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// SSL/TLS certificate information returned from an SSL test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultCertificate {
    /// Cipher used for the TLS connection.
    #[serde(rename = "cipher")]
    pub cipher: Option<String>,
    /// RSA exponent of the certificate.
    #[serde(rename = "exponent")]
    pub exponent: Option<i64>,
    /// Extended key usage extensions for the certificate.
    #[serde(rename = "ext_key_usage")]
    pub ext_key_usage: Option<Vec<String>>,
    /// SHA-1 fingerprint of the certificate.
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    /// SHA-256 fingerprint of the certificate.
    #[serde(rename = "fingerprint256")]
    pub fingerprint256: Option<String>,
    /// Certificate issuer details.
    #[serde(rename = "issuer")]
    pub issuer: Option<std::collections::BTreeMap<String, String>>,
    /// RSA modulus of the certificate.
    #[serde(rename = "modulus")]
    pub modulus: Option<String>,
    /// TLS protocol used (for example, `TLSv1.2`).
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// Serial number of the certificate.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    /// Certificate subject details.
    #[serde(rename = "subject")]
    pub subject: Option<std::collections::BTreeMap<String, String>>,
    /// TLS protocol version.
    #[serde(rename = "tls_version")]
    pub tls_version: Option<f64>,
    /// Validity window of a certificate.
    #[serde(rename = "valid")]
    pub valid: Option<crate::datadogV2::model::SyntheticsTestResultCertificateValidity>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultCertificate {
    pub fn new() -> SyntheticsTestResultCertificate {
        SyntheticsTestResultCertificate {
            cipher: None,
            exponent: None,
            ext_key_usage: None,
            fingerprint: None,
            fingerprint256: None,
            issuer: None,
            modulus: None,
            protocol: None,
            serial_number: None,
            subject: None,
            tls_version: None,
            valid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cipher(mut self, value: String) -> Self {
        self.cipher = Some(value);
        self
    }

    pub fn exponent(mut self, value: i64) -> Self {
        self.exponent = Some(value);
        self
    }

    pub fn ext_key_usage(mut self, value: Vec<String>) -> Self {
        self.ext_key_usage = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: String) -> Self {
        self.fingerprint = Some(value);
        self
    }

    pub fn fingerprint256(mut self, value: String) -> Self {
        self.fingerprint256 = Some(value);
        self
    }

    pub fn issuer(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.issuer = Some(value);
        self
    }

    pub fn modulus(mut self, value: String) -> Self {
        self.modulus = Some(value);
        self
    }

    pub fn protocol(mut self, value: String) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn serial_number(mut self, value: String) -> Self {
        self.serial_number = Some(value);
        self
    }

    pub fn subject(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn tls_version(mut self, value: f64) -> Self {
        self.tls_version = Some(value);
        self
    }

    pub fn valid(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCertificateValidity,
    ) -> Self {
        self.valid = Some(value);
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

impl Default for SyntheticsTestResultCertificate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultCertificate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultCertificateVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultCertificateVisitor {
            type Value = SyntheticsTestResultCertificate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cipher: Option<String> = None;
                let mut exponent: Option<i64> = None;
                let mut ext_key_usage: Option<Vec<String>> = None;
                let mut fingerprint: Option<String> = None;
                let mut fingerprint256: Option<String> = None;
                let mut issuer: Option<std::collections::BTreeMap<String, String>> = None;
                let mut modulus: Option<String> = None;
                let mut protocol: Option<String> = None;
                let mut serial_number: Option<String> = None;
                let mut subject: Option<std::collections::BTreeMap<String, String>> = None;
                let mut tls_version: Option<f64> = None;
                let mut valid: Option<
                    crate::datadogV2::model::SyntheticsTestResultCertificateValidity,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cipher" => {
                            if v.is_null() {
                                continue;
                            }
                            cipher = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exponent" => {
                            if v.is_null() {
                                continue;
                            }
                            exponent = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ext_key_usage" => {
                            if v.is_null() {
                                continue;
                            }
                            ext_key_usage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fingerprint" => {
                            if v.is_null() {
                                continue;
                            }
                            fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fingerprint256" => {
                            if v.is_null() {
                                continue;
                            }
                            fingerprint256 =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issuer" => {
                            if v.is_null() {
                                continue;
                            }
                            issuer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modulus" => {
                            if v.is_null() {
                                continue;
                            }
                            modulus = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol" => {
                            if v.is_null() {
                                continue;
                            }
                            protocol = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serial_number" => {
                            if v.is_null() {
                                continue;
                            }
                            serial_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subject" => {
                            if v.is_null() {
                                continue;
                            }
                            subject = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tls_version" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            tls_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "valid" => {
                            if v.is_null() {
                                continue;
                            }
                            valid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultCertificate {
                    cipher,
                    exponent,
                    ext_key_usage,
                    fingerprint,
                    fingerprint256,
                    issuer,
                    modulus,
                    protocol,
                    serial_number,
                    subject,
                    tls_version,
                    valid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultCertificateVisitor)
    }
}
