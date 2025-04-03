// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an aggregated connection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SingleAggregatedConnectionResponseDataAttributes {
    /// The total number of bytes sent by the client over the given period.
    #[serde(rename = "bytes_sent_by_client")]
    pub bytes_sent_by_client: Option<i64>,
    /// The total number of bytes sent by the server over the given period.
    #[serde(rename = "bytes_sent_by_server")]
    pub bytes_sent_by_server: Option<i64>,
    /// The key, value pairs for each group by.
    #[serde(rename = "group_bys")]
    pub group_bys: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// The total number of packets sent by the client over the given period.
    #[serde(rename = "packets_sent_by_client")]
    pub packets_sent_by_client: Option<i64>,
    /// The total number of packets sent by the server over the given period.
    #[serde(rename = "packets_sent_by_server")]
    pub packets_sent_by_server: Option<i64>,
    /// Measured as TCP smoothed round trip time in microseconds (the time between a TCP frame being sent and acknowledged).
    #[serde(rename = "rtt_micro_seconds")]
    pub rtt_micro_seconds: Option<i64>,
    /// The number of TCP connections in a closed state. Measured in connections per second from the client.
    #[serde(rename = "tcp_closed_connections")]
    pub tcp_closed_connections: Option<i64>,
    /// The number of TCP connections in an established state. Measured in connections per second from the client.
    #[serde(rename = "tcp_established_connections")]
    pub tcp_established_connections: Option<i64>,
    /// The number of TCP connections that were refused by the server. Typically this indicates an attempt to connect to an IP/port that is not receiving connections, or a firewall/security misconfiguration.
    #[serde(rename = "tcp_refusals")]
    pub tcp_refusals: Option<i64>,
    /// The number of TCP connections that were reset by the server.
    #[serde(rename = "tcp_resets")]
    pub tcp_resets: Option<i64>,
    /// TCP Retransmits represent detected failures that are retransmitted to ensure delivery. Measured in count of retransmits from the client.
    #[serde(rename = "tcp_retransmits")]
    pub tcp_retransmits: Option<i64>,
    /// The number of TCP connections that timed out from the perspective of the operating system. This can indicate general connectivity and latency issues.
    #[serde(rename = "tcp_timeouts")]
    pub tcp_timeouts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SingleAggregatedConnectionResponseDataAttributes {
    pub fn new() -> SingleAggregatedConnectionResponseDataAttributes {
        SingleAggregatedConnectionResponseDataAttributes {
            bytes_sent_by_client: None,
            bytes_sent_by_server: None,
            group_bys: None,
            packets_sent_by_client: None,
            packets_sent_by_server: None,
            rtt_micro_seconds: None,
            tcp_closed_connections: None,
            tcp_established_connections: None,
            tcp_refusals: None,
            tcp_resets: None,
            tcp_retransmits: None,
            tcp_timeouts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bytes_sent_by_client(mut self, value: i64) -> Self {
        self.bytes_sent_by_client = Some(value);
        self
    }

    pub fn bytes_sent_by_server(mut self, value: i64) -> Self {
        self.bytes_sent_by_server = Some(value);
        self
    }

    pub fn group_bys(mut self, value: std::collections::BTreeMap<String, Vec<String>>) -> Self {
        self.group_bys = Some(value);
        self
    }

    pub fn packets_sent_by_client(mut self, value: i64) -> Self {
        self.packets_sent_by_client = Some(value);
        self
    }

    pub fn packets_sent_by_server(mut self, value: i64) -> Self {
        self.packets_sent_by_server = Some(value);
        self
    }

    pub fn rtt_micro_seconds(mut self, value: i64) -> Self {
        self.rtt_micro_seconds = Some(value);
        self
    }

    pub fn tcp_closed_connections(mut self, value: i64) -> Self {
        self.tcp_closed_connections = Some(value);
        self
    }

    pub fn tcp_established_connections(mut self, value: i64) -> Self {
        self.tcp_established_connections = Some(value);
        self
    }

    pub fn tcp_refusals(mut self, value: i64) -> Self {
        self.tcp_refusals = Some(value);
        self
    }

    pub fn tcp_resets(mut self, value: i64) -> Self {
        self.tcp_resets = Some(value);
        self
    }

    pub fn tcp_retransmits(mut self, value: i64) -> Self {
        self.tcp_retransmits = Some(value);
        self
    }

    pub fn tcp_timeouts(mut self, value: i64) -> Self {
        self.tcp_timeouts = Some(value);
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

impl Default for SingleAggregatedConnectionResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SingleAggregatedConnectionResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SingleAggregatedConnectionResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for SingleAggregatedConnectionResponseDataAttributesVisitor {
            type Value = SingleAggregatedConnectionResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bytes_sent_by_client: Option<i64> = None;
                let mut bytes_sent_by_server: Option<i64> = None;
                let mut group_bys: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut packets_sent_by_client: Option<i64> = None;
                let mut packets_sent_by_server: Option<i64> = None;
                let mut rtt_micro_seconds: Option<i64> = None;
                let mut tcp_closed_connections: Option<i64> = None;
                let mut tcp_established_connections: Option<i64> = None;
                let mut tcp_refusals: Option<i64> = None;
                let mut tcp_resets: Option<i64> = None;
                let mut tcp_retransmits: Option<i64> = None;
                let mut tcp_timeouts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bytes_sent_by_client" => {
                            if v.is_null() {
                                continue;
                            }
                            bytes_sent_by_client =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bytes_sent_by_server" => {
                            if v.is_null() {
                                continue;
                            }
                            bytes_sent_by_server =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_bys" => {
                            if v.is_null() {
                                continue;
                            }
                            group_bys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "packets_sent_by_client" => {
                            if v.is_null() {
                                continue;
                            }
                            packets_sent_by_client =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "packets_sent_by_server" => {
                            if v.is_null() {
                                continue;
                            }
                            packets_sent_by_server =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rtt_micro_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            rtt_micro_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_closed_connections" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_closed_connections =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_established_connections" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_established_connections =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_refusals" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_refusals =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_resets" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_resets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_retransmits" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_retransmits =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_timeouts" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_timeouts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SingleAggregatedConnectionResponseDataAttributes {
                    bytes_sent_by_client,
                    bytes_sent_by_server,
                    group_bys,
                    packets_sent_by_client,
                    packets_sent_by_server,
                    rtt_micro_seconds,
                    tcp_closed_connections,
                    tcp_established_connections,
                    tcp_refusals,
                    tcp_resets,
                    tcp_retransmits,
                    tcp_timeouts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SingleAggregatedConnectionResponseDataAttributesVisitor)
    }
}
