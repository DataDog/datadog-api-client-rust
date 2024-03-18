// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Client certificate to use when performing the test request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRequestCertificate {
    /// Define a request certificate.
    #[serde(rename = "cert")]
    pub cert: Option<crate::datadogV1::model::SyntheticsTestRequestCertificateItem>,
    /// Define a request certificate.
    #[serde(rename = "key")]
    pub key: Option<crate::datadogV1::model::SyntheticsTestRequestCertificateItem>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRequestCertificate {
    pub fn new() -> SyntheticsTestRequestCertificate {
        SyntheticsTestRequestCertificate {
            cert: None,
            key: None,
            _unparsed: false,
        }
    }

    pub fn cert(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestCertificateItem,
    ) -> Self {
        self.cert = Some(value);
        self
    }

    pub fn key(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestCertificateItem,
    ) -> Self {
        self.key = Some(value);
        self
    }
}

impl Default for SyntheticsTestRequestCertificate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestCertificate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRequestCertificateVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRequestCertificateVisitor {
            type Value = SyntheticsTestRequestCertificate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cert: Option<
                    crate::datadogV1::model::SyntheticsTestRequestCertificateItem,
                > = None;
                let mut key: Option<crate::datadogV1::model::SyntheticsTestRequestCertificateItem> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cert" => {
                            if v.is_null() {
                                continue;
                            }
                            cert = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestRequestCertificate {
                    cert,
                    key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRequestCertificateVisitor)
    }
}
