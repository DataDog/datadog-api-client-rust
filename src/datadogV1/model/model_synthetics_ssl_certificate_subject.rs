// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the SSL certificate used for the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsSSLCertificateSubject {
    /// Country Name associated with the certificate.
    #[serde(rename = "C")]
    pub c: Option<String>,
    /// Common Name that associated with the certificate.
    #[serde(rename = "CN")]
    pub cn: Option<String>,
    /// Locality associated with the certificate.
    #[serde(rename = "L")]
    pub l: Option<String>,
    /// Organization associated with the certificate.
    #[serde(rename = "O")]
    pub o: Option<String>,
    /// Organizational Unit associated with the certificate.
    #[serde(rename = "OU")]
    pub ou: Option<String>,
    /// State Or Province Name associated with the certificate.
    #[serde(rename = "ST")]
    pub st: Option<String>,
    /// Subject Alternative Name associated with the certificate.
    #[serde(rename = "altName")]
    pub alt_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsSSLCertificateSubject {
    pub fn new() -> SyntheticsSSLCertificateSubject {
        SyntheticsSSLCertificateSubject {
            c: None,
            cn: None,
            l: None,
            o: None,
            ou: None,
            st: None,
            alt_name: None,
            _unparsed: false,
        }
    }

    pub fn c(mut self, value: String) -> Self {
        self.c = Some(value);
        self
    }

    pub fn cn(mut self, value: String) -> Self {
        self.cn = Some(value);
        self
    }

    pub fn l(mut self, value: String) -> Self {
        self.l = Some(value);
        self
    }

    pub fn o(mut self, value: String) -> Self {
        self.o = Some(value);
        self
    }

    pub fn ou(mut self, value: String) -> Self {
        self.ou = Some(value);
        self
    }

    pub fn st(mut self, value: String) -> Self {
        self.st = Some(value);
        self
    }

    pub fn alt_name(mut self, value: String) -> Self {
        self.alt_name = Some(value);
        self
    }
}

impl Default for SyntheticsSSLCertificateSubject {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsSSLCertificateSubject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsSSLCertificateSubjectVisitor;
        impl<'a> Visitor<'a> for SyntheticsSSLCertificateSubjectVisitor {
            type Value = SyntheticsSSLCertificateSubject;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut c: Option<String> = None;
                let mut cn: Option<String> = None;
                let mut l: Option<String> = None;
                let mut o: Option<String> = None;
                let mut ou: Option<String> = None;
                let mut st: Option<String> = None;
                let mut alt_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "C" => {
                            if v.is_null() {
                                continue;
                            }
                            c = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "CN" => {
                            if v.is_null() {
                                continue;
                            }
                            cn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "L" => {
                            if v.is_null() {
                                continue;
                            }
                            l = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "O" => {
                            if v.is_null() {
                                continue;
                            }
                            o = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "OU" => {
                            if v.is_null() {
                                continue;
                            }
                            ou = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ST" => {
                            if v.is_null() {
                                continue;
                            }
                            st = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "altName" => {
                            if v.is_null() {
                                continue;
                            }
                            alt_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsSSLCertificateSubject {
                    c,
                    cn,
                    l,
                    o,
                    ou,
                    st,
                    alt_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsSSLCertificateSubjectVisitor)
    }
}
