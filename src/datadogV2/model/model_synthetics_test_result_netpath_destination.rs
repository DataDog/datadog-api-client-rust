// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Destination endpoint of a network path measurement.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultNetpathDestination {
    /// Hostname of the destination.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// IP address of the destination.
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
    /// Port of the destination service.
    #[serde(rename = "port")]
    pub port: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultNetpathDestination {
    pub fn new() -> SyntheticsTestResultNetpathDestination {
        SyntheticsTestResultNetpathDestination {
            hostname: None,
            ip_address: None,
            port: None,
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

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
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

impl Default for SyntheticsTestResultNetpathDestination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultNetpathDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultNetpathDestinationVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultNetpathDestinationVisitor {
            type Value = SyntheticsTestResultNetpathDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hostname: Option<String> = None;
                let mut ip_address: Option<String> = None;
                let mut port: Option<i64> = None;
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
                        "port" => {
                            if v.is_null() {
                                continue;
                            }
                            port = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultNetpathDestination {
                    hostname,
                    ip_address,
                    port,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultNetpathDestinationVisitor)
    }
}
