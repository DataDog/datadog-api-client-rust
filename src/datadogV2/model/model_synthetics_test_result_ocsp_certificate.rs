// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Certificate details returned in an OCSP response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultOCSPCertificate {
    /// Reason code for the revocation, when applicable.
    #[serde(rename = "revocation_reason")]
    pub revocation_reason: Option<String>,
    /// Unix timestamp (ms) of the revocation.
    #[serde(rename = "revocation_time")]
    pub revocation_time: Option<i64>,
    /// Serial number of the certificate.
    #[serde(rename = "serial_number")]
    pub serial_number: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultOCSPCertificate {
    pub fn new() -> SyntheticsTestResultOCSPCertificate {
        SyntheticsTestResultOCSPCertificate {
            revocation_reason: None,
            revocation_time: None,
            serial_number: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn revocation_reason(mut self, value: String) -> Self {
        self.revocation_reason = Some(value);
        self
    }

    pub fn revocation_time(mut self, value: i64) -> Self {
        self.revocation_time = Some(value);
        self
    }

    pub fn serial_number(mut self, value: String) -> Self {
        self.serial_number = Some(value);
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

impl Default for SyntheticsTestResultOCSPCertificate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultOCSPCertificate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultOCSPCertificateVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultOCSPCertificateVisitor {
            type Value = SyntheticsTestResultOCSPCertificate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut revocation_reason: Option<String> = None;
                let mut revocation_time: Option<i64> = None;
                let mut serial_number: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "revocation_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            revocation_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revocation_time" => {
                            if v.is_null() {
                                continue;
                            }
                            revocation_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serial_number" => {
                            if v.is_null() {
                                continue;
                            }
                            serial_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultOCSPCertificate {
                    revocation_reason,
                    revocation_time,
                    serial_number,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultOCSPCertificateVisitor)
    }
}
