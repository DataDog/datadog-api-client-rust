// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Average timing breakdown per network phase for a resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AggregatedResourceTimingBreakdown {
    /// Average TCP connect duration in milliseconds.
    #[serde(rename = "avg_connect_ms")]
    pub avg_connect_ms: f64,
    /// Average DNS resolution duration in milliseconds.
    #[serde(rename = "avg_dns_ms")]
    pub avg_dns_ms: f64,
    /// Average download phase duration in milliseconds.
    #[serde(rename = "avg_download_ms")]
    pub avg_download_ms: f64,
    /// Average time to first byte in milliseconds.
    #[serde(rename = "avg_first_byte_ms")]
    pub avg_first_byte_ms: f64,
    /// Average redirect phase duration in milliseconds.
    #[serde(rename = "avg_redirect_ms")]
    pub avg_redirect_ms: f64,
    /// Average SSL handshake duration in milliseconds.
    #[serde(rename = "avg_ssl_ms")]
    pub avg_ssl_ms: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AggregatedResourceTimingBreakdown {
    pub fn new(
        avg_connect_ms: f64,
        avg_dns_ms: f64,
        avg_download_ms: f64,
        avg_first_byte_ms: f64,
        avg_redirect_ms: f64,
        avg_ssl_ms: f64,
    ) -> AggregatedResourceTimingBreakdown {
        AggregatedResourceTimingBreakdown {
            avg_connect_ms,
            avg_dns_ms,
            avg_download_ms,
            avg_first_byte_ms,
            avg_redirect_ms,
            avg_ssl_ms,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AggregatedResourceTimingBreakdown {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregatedResourceTimingBreakdownVisitor;
        impl<'a> Visitor<'a> for AggregatedResourceTimingBreakdownVisitor {
            type Value = AggregatedResourceTimingBreakdown;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut avg_connect_ms: Option<f64> = None;
                let mut avg_dns_ms: Option<f64> = None;
                let mut avg_download_ms: Option<f64> = None;
                let mut avg_first_byte_ms: Option<f64> = None;
                let mut avg_redirect_ms: Option<f64> = None;
                let mut avg_ssl_ms: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "avg_connect_ms" => {
                            avg_connect_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_dns_ms" => {
                            avg_dns_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_download_ms" => {
                            avg_download_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_first_byte_ms" => {
                            avg_first_byte_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_redirect_ms" => {
                            avg_redirect_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "avg_ssl_ms" => {
                            avg_ssl_ms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let avg_connect_ms =
                    avg_connect_ms.ok_or_else(|| M::Error::missing_field("avg_connect_ms"))?;
                let avg_dns_ms = avg_dns_ms.ok_or_else(|| M::Error::missing_field("avg_dns_ms"))?;
                let avg_download_ms =
                    avg_download_ms.ok_or_else(|| M::Error::missing_field("avg_download_ms"))?;
                let avg_first_byte_ms = avg_first_byte_ms
                    .ok_or_else(|| M::Error::missing_field("avg_first_byte_ms"))?;
                let avg_redirect_ms =
                    avg_redirect_ms.ok_or_else(|| M::Error::missing_field("avg_redirect_ms"))?;
                let avg_ssl_ms = avg_ssl_ms.ok_or_else(|| M::Error::missing_field("avg_ssl_ms"))?;

                let content = AggregatedResourceTimingBreakdown {
                    avg_connect_ms,
                    avg_dns_ms,
                    avg_download_ms,
                    avg_first_byte_ms,
                    avg_redirect_ms,
                    avg_ssl_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AggregatedResourceTimingBreakdownVisitor)
    }
}
