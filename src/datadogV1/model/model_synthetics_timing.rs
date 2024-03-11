// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing all metrics and their values collected for a Synthetic API test.
/// See the [Synthetic Monitoring Metrics documentation](<https://docs.datadoghq.com/synthetics/metrics/>).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTiming {
    /// The duration in millisecond of the DNS lookup.
    #[serde(rename = "dns")]
    pub dns: Option<f64>,
    /// The time in millisecond to download the response.
    #[serde(rename = "download")]
    pub download: Option<f64>,
    /// The time in millisecond to first byte.
    #[serde(rename = "firstByte")]
    pub first_byte: Option<f64>,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "handshake")]
    pub handshake: Option<f64>,
    /// The time in millisecond spent during redirections.
    #[serde(rename = "redirect")]
    pub redirect: Option<f64>,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "ssl")]
    pub ssl: Option<f64>,
    /// Time in millisecond to establish the TCP connection.
    #[serde(rename = "tcp")]
    pub tcp: Option<f64>,
    /// The overall time in millisecond the request took to be processed.
    #[serde(rename = "total")]
    pub total: Option<f64>,
    /// Time spent in millisecond waiting for a response.
    #[serde(rename = "wait")]
    pub wait: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTiming {
    pub fn new() -> SyntheticsTiming {
        SyntheticsTiming {
            dns: None,
            download: None,
            first_byte: None,
            handshake: None,
            redirect: None,
            ssl: None,
            tcp: None,
            total: None,
            wait: None,
            _unparsed: false,
        }
    }

    pub fn dns(&mut self, value: f64) -> &mut Self {
        self.dns = Some(value);
        self
    }

    pub fn download(&mut self, value: f64) -> &mut Self {
        self.download = Some(value);
        self
    }

    pub fn first_byte(&mut self, value: f64) -> &mut Self {
        self.first_byte = Some(value);
        self
    }

    pub fn handshake(&mut self, value: f64) -> &mut Self {
        self.handshake = Some(value);
        self
    }

    pub fn redirect(&mut self, value: f64) -> &mut Self {
        self.redirect = Some(value);
        self
    }

    pub fn ssl(&mut self, value: f64) -> &mut Self {
        self.ssl = Some(value);
        self
    }

    pub fn tcp(&mut self, value: f64) -> &mut Self {
        self.tcp = Some(value);
        self
    }

    pub fn total(&mut self, value: f64) -> &mut Self {
        self.total = Some(value);
        self
    }

    pub fn wait(&mut self, value: f64) -> &mut Self {
        self.wait = Some(value);
        self
    }
}

impl Default for SyntheticsTiming {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTiming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTimingVisitor;
        impl<'a> Visitor<'a> for SyntheticsTimingVisitor {
            type Value = SyntheticsTiming;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dns: Option<f64> = None;
                let mut download: Option<f64> = None;
                let mut first_byte: Option<f64> = None;
                let mut handshake: Option<f64> = None;
                let mut redirect: Option<f64> = None;
                let mut ssl: Option<f64> = None;
                let mut tcp: Option<f64> = None;
                let mut total: Option<f64> = None;
                let mut wait: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dns" => {
                            if v.is_null() {
                                continue;
                            }
                            dns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "download" => {
                            if v.is_null() {
                                continue;
                            }
                            download = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "firstByte" => {
                            if v.is_null() {
                                continue;
                            }
                            first_byte = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handshake" => {
                            if v.is_null() {
                                continue;
                            }
                            handshake = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirect" => {
                            if v.is_null() {
                                continue;
                            }
                            redirect = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ssl" => {
                            if v.is_null() {
                                continue;
                            }
                            ssl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "wait" => {
                            if v.is_null() {
                                continue;
                            }
                            wait = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTiming {
                    dns,
                    download,
                    first_byte,
                    handshake,
                    redirect,
                    ssl,
                    tcp,
                    total,
                    wait,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTimingVisitor)
    }
}
