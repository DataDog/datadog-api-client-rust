// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the SSL certificate used for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsSSLCertificate {
    /// Cipher used for the connection.
    #[serde(rename = "cipher")]
    pub cipher: Option<String>,
    /// Exponent associated to the certificate.
    #[serde(rename = "exponent")]
    pub exponent: Option<f64>,
    /// Array of extensions and details used for the certificate.
    #[serde(rename = "extKeyUsage")]
    pub ext_key_usage: Option<Vec<String>>,
    /// MD5 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint")]
    pub fingerprint: Option<String>,
    /// SHA-1 digest of the DER-encoded Certificate information.
    #[serde(rename = "fingerprint256")]
    pub fingerprint256: Option<String>,
    /// Object describing the issuer of a SSL certificate.
    #[serde(rename = "issuer")]
    pub issuer: Option<crate::datadogV1::model::SyntheticsSSLCertificateIssuer>,
    /// Modulus associated to the SSL certificate private key.
    #[serde(rename = "modulus")]
    pub modulus: Option<String>,
    /// TLS protocol used for the test.
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// Serial Number assigned by Symantec to the SSL certificate.
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,
    /// Object describing the SSL certificate used for the test.
    #[serde(rename = "subject")]
    pub subject: Option<crate::datadogV1::model::SyntheticsSSLCertificateSubject>,
    /// Date from which the SSL certificate is valid.
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    /// Date until which the SSL certificate is valid.
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsSSLCertificate {
    pub fn new() -> SyntheticsSSLCertificate {
        SyntheticsSSLCertificate {
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
            valid_from: None,
            valid_to: None,
            _unparsed: false,
        }
    }

    pub fn cipher(mut self, value: String) -> Self {
        self.cipher = Some(value);
        self
    }

    pub fn exponent(mut self, value: f64) -> Self {
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

    pub fn issuer(
        mut self,
        value: crate::datadogV1::model::SyntheticsSSLCertificateIssuer,
    ) -> Self {
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

    pub fn subject(
        mut self,
        value: crate::datadogV1::model::SyntheticsSSLCertificateSubject,
    ) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn valid_from(mut self, value: String) -> Self {
        self.valid_from = Some(value);
        self
    }

    pub fn valid_to(mut self, value: String) -> Self {
        self.valid_to = Some(value);
        self
    }
}

impl Default for SyntheticsSSLCertificate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsSSLCertificate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsSSLCertificateVisitor;
        impl<'a> Visitor<'a> for SyntheticsSSLCertificateVisitor {
            type Value = SyntheticsSSLCertificate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cipher: Option<String> = None;
                let mut exponent: Option<f64> = None;
                let mut ext_key_usage: Option<Vec<String>> = None;
                let mut fingerprint: Option<String> = None;
                let mut fingerprint256: Option<String> = None;
                let mut issuer: Option<crate::datadogV1::model::SyntheticsSSLCertificateIssuer> =
                    None;
                let mut modulus: Option<String> = None;
                let mut protocol: Option<String> = None;
                let mut serial_number: Option<String> = None;
                let mut subject: Option<crate::datadogV1::model::SyntheticsSSLCertificateSubject> =
                    None;
                let mut valid_from: Option<String> = None;
                let mut valid_to: Option<String> = None;
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
                        "extKeyUsage" => {
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
                        "serialNumber" => {
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
                        "validFrom" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validTo" => {
                            if v.is_null() {
                                continue;
                            }
                            valid_to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsSSLCertificate {
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
                    valid_from,
                    valid_to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsSSLCertificateVisitor)
    }
}
