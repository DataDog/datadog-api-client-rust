// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single hop along a network path.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultNetpathHop {
    /// Resolved hostname of the hop.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// IP address of the hop.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// Whether this hop was reachable.
    #[serde(rename = "reachable")]
    pub reachable: Option<bool>,
    /// Round-trip time to this hop in milliseconds.
    #[serde(rename = "rtt")]
    pub rtt: Option<f64>,
    /// Time-to-live value of the probe packet at this hop.
    #[serde(rename = "ttl")]
    pub ttl: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultNetpathHop {
    pub fn new() -> SyntheticsTestResultNetpathHop {
        SyntheticsTestResultNetpathHop {
            hostname: None,
            ip_address: None,
            reachable: None,
            rtt: None,
            ttl: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hostname(mut self, value: String) -> Self {
        self.hostname = Some(value);
        self
    }

    pub fn ip_address(mut self, value: String) -> Self {
        self.ip_address = Some(value);
        self
    }

    pub fn reachable(mut self, value: bool) -> Self {
        self.reachable = Some(value);
        self
    }

    pub fn rtt(mut self, value: f64) -> Self {
        self.rtt = Some(value);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
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

impl Default for SyntheticsTestResultNetpathHop {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultNetpathHop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultNetpathHopVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultNetpathHopVisitor {
            type Value = SyntheticsTestResultNetpathHop;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hostname: Option<String> = None;
                let mut ip_address: Option<String> = None;
                let mut reachable: Option<bool> = None;
                let mut rtt: Option<f64> = None;
                let mut ttl: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hostname" => {
                            if v.is_null() {
                                continue;
                            }
                            hostname = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ip_address" => {
                            if v.is_null() {
                                continue;
                            }
                            ip_address = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reachable" => {
                            if v.is_null() {
                                continue;
                            }
                            reachable = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rtt" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            rtt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ttl" => {
                            if v.is_null() {
                                continue;
                            }
                            ttl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultNetpathHop {
                    hostname,
                    ip_address,
                    reachable,
                    rtt,
                    ttl,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultNetpathHopVisitor)
    }
}
