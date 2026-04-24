// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Network Path test result capturing the path between source and destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultNetpath {
    /// Destination endpoint of a network path measurement.
    #[serde(rename = "destination")]
    pub destination: Option<crate::datadogV2::model::SyntheticsTestResultNetpathDestination>,
    /// Hops along the network path.
    #[serde(rename = "hops")]
    pub hops: Option<Vec<crate::datadogV2::model::SyntheticsTestResultNetpathHop>>,
    /// Origin of the network path (for example, probe source).
    #[serde(rename = "origin")]
    pub origin: Option<String>,
    /// Identifier of the path trace.
    #[serde(rename = "pathtrace_id")]
    pub pathtrace_id: Option<String>,
    /// Protocol used for the path trace (for example, `tcp`, `udp`, `icmp`).
    #[serde(rename = "protocol")]
    pub protocol: Option<String>,
    /// Source endpoint of a network path measurement.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV2::model::SyntheticsTestResultNetpathEndpoint>,
    /// Tags associated with the network path measurement.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Unix timestamp (ms) of the network path measurement.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultNetpath {
    pub fn new() -> SyntheticsTestResultNetpath {
        SyntheticsTestResultNetpath {
            destination: None,
            hops: None,
            origin: None,
            pathtrace_id: None,
            protocol: None,
            source: None,
            tags: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn destination(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultNetpathDestination,
    ) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn hops(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultNetpathHop>,
    ) -> Self {
        self.hops = Some(value);
        self
    }

    pub fn origin(mut self, value: String) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn pathtrace_id(mut self, value: String) -> Self {
        self.pathtrace_id = Some(value);
        self
    }

    pub fn protocol(mut self, value: String) -> Self {
        self.protocol = Some(value);
        self
    }

    pub fn source(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultNetpathEndpoint,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
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

impl Default for SyntheticsTestResultNetpath {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultNetpath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultNetpathVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultNetpathVisitor {
            type Value = SyntheticsTestResultNetpath;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination: Option<
                    crate::datadogV2::model::SyntheticsTestResultNetpathDestination,
                > = None;
                let mut hops: Option<Vec<crate::datadogV2::model::SyntheticsTestResultNetpathHop>> =
                    None;
                let mut origin: Option<String> = None;
                let mut pathtrace_id: Option<String> = None;
                let mut protocol: Option<String> = None;
                let mut source: Option<
                    crate::datadogV2::model::SyntheticsTestResultNetpathEndpoint,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination" => {
                            if v.is_null() {
                                continue;
                            }
                            destination =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hops" => {
                            if v.is_null() {
                                continue;
                            }
                            hops = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "origin" => {
                            if v.is_null() {
                                continue;
                            }
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pathtrace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            pathtrace_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol" => {
                            if v.is_null() {
                                continue;
                            }
                            protocol = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultNetpath {
                    destination,
                    hops,
                    origin,
                    pathtrace_id,
                    protocol,
                    source,
                    tags,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultNetpathVisitor)
    }
}
