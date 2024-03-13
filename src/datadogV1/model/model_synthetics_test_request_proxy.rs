// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The proxy to perform the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRequestProxy {
    /// Headers to include when performing the test.
    #[serde(rename = "headers")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    /// URL of the proxy to perform the test.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRequestProxy {
    pub fn new(url: String) -> SyntheticsTestRequestProxy {
        SyntheticsTestRequestProxy {
            headers: None,
            url,
            _unparsed: false,
        }
    }

    pub fn headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRequestProxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRequestProxyVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRequestProxyVisitor {
            type Value = SyntheticsTestRequestProxy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = SyntheticsTestRequestProxy {
                    headers,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRequestProxyVisitor)
    }
}
