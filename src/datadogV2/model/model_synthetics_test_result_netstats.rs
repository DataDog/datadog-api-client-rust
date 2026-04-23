// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Aggregated network statistics from the test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultNetstats {
    /// Statistics about the number of hops for a network test.
    #[serde(rename = "hops")]
    pub hops: Option<crate::datadogV2::model::SyntheticsTestResultNetstatsHops>,
    /// Network jitter in milliseconds.
    #[serde(rename = "jitter")]
    pub jitter: Option<f64>,
    /// Latency statistics for a network probe.
    #[serde(rename = "latency")]
    pub latency: Option<crate::datadogV2::model::SyntheticsTestResultNetworkLatency>,
    /// Percentage of probe packets lost.
    #[serde(rename = "packet_loss_percentage")]
    pub packet_loss_percentage: Option<f64>,
    /// Number of probe packets received.
    #[serde(rename = "packets_received")]
    pub packets_received: Option<i64>,
    /// Number of probe packets sent.
    #[serde(rename = "packets_sent")]
    pub packets_sent: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultNetstats {
    pub fn new() -> SyntheticsTestResultNetstats {
        SyntheticsTestResultNetstats {
            hops: None,
            jitter: None,
            latency: None,
            packet_loss_percentage: None,
            packets_received: None,
            packets_sent: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hops(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultNetstatsHops,
    ) -> Self {
        self.hops = Some(value);
        self
    }

    pub fn jitter(mut self, value: f64) -> Self {
        self.jitter = Some(value);
        self
    }

    pub fn latency(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultNetworkLatency,
    ) -> Self {
        self.latency = Some(value);
        self
    }

    pub fn packet_loss_percentage(mut self, value: f64) -> Self {
        self.packet_loss_percentage = Some(value);
        self
    }

    pub fn packets_received(mut self, value: i64) -> Self {
        self.packets_received = Some(value);
        self
    }

    pub fn packets_sent(mut self, value: i64) -> Self {
        self.packets_sent = Some(value);
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

impl Default for SyntheticsTestResultNetstats {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultNetstats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultNetstatsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultNetstatsVisitor {
            type Value = SyntheticsTestResultNetstats;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hops: Option<crate::datadogV2::model::SyntheticsTestResultNetstatsHops> =
                    None;
                let mut jitter: Option<f64> = None;
                let mut latency: Option<
                    crate::datadogV2::model::SyntheticsTestResultNetworkLatency,
                > = None;
                let mut packet_loss_percentage: Option<f64> = None;
                let mut packets_received: Option<i64> = None;
                let mut packets_sent: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hops" => {
                            if v.is_null() {
                                continue;
                            }
                            hops = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jitter" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            jitter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "latency" => {
                            if v.is_null() {
                                continue;
                            }
                            latency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "packet_loss_percentage" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            packet_loss_percentage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "packets_received" => {
                            if v.is_null() {
                                continue;
                            }
                            packets_received =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "packets_sent" => {
                            if v.is_null() {
                                continue;
                            }
                            packets_sent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultNetstats {
                    hops,
                    jitter,
                    latency,
                    packet_loss_percentage,
                    packets_received,
                    packets_sent,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultNetstatsVisitor)
    }
}
