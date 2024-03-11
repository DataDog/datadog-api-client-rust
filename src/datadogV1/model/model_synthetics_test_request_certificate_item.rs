// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define a request certificate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRequestCertificateItem {
    /// Content of the certificate or key.
    #[serde(rename = "content")]
    pub content: Option<String>,
    /// File name for the certificate or key.
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    /// Date of update of the certificate or key, ISO format.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRequestCertificateItem {
    pub fn new() -> SyntheticsTestRequestCertificateItem {
        SyntheticsTestRequestCertificateItem {
            content: None,
            filename: None,
            updated_at: None,
            _unparsed: false,
        }
    }

    pub fn content(&mut self, value: String) -> &mut Self {
        self.content = Some(value);
        self
    }

    pub fn filename(&mut self, value: String) -> &mut Self {
        self.filename = Some(value);
        self
    }

    pub fn updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }
}

impl Default for SyntheticsTestRequestCertificateItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestCertificateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRequestCertificateItemVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRequestCertificateItemVisitor {
            type Value = SyntheticsTestRequestCertificateItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<String> = None;
                let mut filename: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filename" => {
                            if v.is_null() {
                                continue;
                            }
                            filename = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updatedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestRequestCertificateItem {
                    content,
                    filename,
                    updated_at,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRequestCertificateItemVisitor)
    }
}
