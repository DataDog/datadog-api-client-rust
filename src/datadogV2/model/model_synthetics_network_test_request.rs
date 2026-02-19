// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the request for a Network Path test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsNetworkTestRequest {
    /// An optional label displayed for the destination host in the Network Path visualization.
    #[serde(rename = "destination_service")]
    pub destination_service: Option<String>,
    /// The number of packets sent to probe the destination to measure packet loss, latency and jitter.
    #[serde(rename = "e2e_queries")]
    pub e2e_queries: i64,
    /// Host name to query.
    #[serde(rename = "host")]
    pub host: String,
    /// The maximum time-to-live (max number of hops) used in outgoing probe packets.
    #[serde(rename = "max_ttl")]
    pub max_ttl: i64,
    /// For TCP or UDP tests, the port to use when performing the test.
    /// If not set on a UDP test, a random port is assigned, which may affect the results.
    #[serde(rename = "port")]
    pub port: Option<i64>,
    /// An optional label displayed for the source host in the Network Path visualization.
    #[serde(rename = "source_service")]
    pub source_service: Option<String>,
    /// For TCP tests, the TCP traceroute strategy.
    #[serde(rename = "tcp_method")]
    pub tcp_method: Option<crate::datadogV2::model::SyntheticsNetworkTestRequestTCPMethod>,
    /// Timeout in seconds.
    #[serde(rename = "timeout")]
    pub timeout: Option<i64>,
    /// The number of traceroute path tracings.
    #[serde(rename = "traceroute_queries")]
    pub traceroute_queries: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsNetworkTestRequest {
    pub fn new(
        e2e_queries: i64,
        host: String,
        max_ttl: i64,
        traceroute_queries: i64,
    ) -> SyntheticsNetworkTestRequest {
        SyntheticsNetworkTestRequest {
            destination_service: None,
            e2e_queries,
            host,
            max_ttl,
            port: None,
            source_service: None,
            tcp_method: None,
            timeout: None,
            traceroute_queries,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn destination_service(mut self, value: String) -> Self {
        self.destination_service = Some(value);
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn source_service(mut self, value: String) -> Self {
        self.source_service = Some(value);
        self
    }

    pub fn tcp_method(
        mut self,
        value: crate::datadogV2::model::SyntheticsNetworkTestRequestTCPMethod,
    ) -> Self {
        self.tcp_method = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsNetworkTestRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsNetworkTestRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsNetworkTestRequestVisitor {
            type Value = SyntheticsNetworkTestRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut destination_service: Option<String> = None;
                let mut e2e_queries: Option<i64> = None;
                let mut host: Option<String> = None;
                let mut max_ttl: Option<i64> = None;
                let mut port: Option<i64> = None;
                let mut source_service: Option<String> = None;
                let mut tcp_method: Option<
                    crate::datadogV2::model::SyntheticsNetworkTestRequestTCPMethod,
                > = None;
                let mut timeout: Option<i64> = None;
                let mut traceroute_queries: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "destination_service" => {
                            if v.is_null() {
                                continue;
                            }
                            destination_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "e2e_queries" => {
                            e2e_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_ttl" => {
                            max_ttl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "port" => {
                            if v.is_null() {
                                continue;
                            }
                            port = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_service" => {
                            if v.is_null() {
                                continue;
                            }
                            source_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tcp_method" => {
                            if v.is_null() {
                                continue;
                            }
                            tcp_method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _tcp_method) = tcp_method {
                                match _tcp_method {
                                    crate::datadogV2::model::SyntheticsNetworkTestRequestTCPMethod::UnparsedObject(_tcp_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceroute_queries" => {
                            traceroute_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let e2e_queries =
                    e2e_queries.ok_or_else(|| M::Error::missing_field("e2e_queries"))?;
                let host = host.ok_or_else(|| M::Error::missing_field("host"))?;
                let max_ttl = max_ttl.ok_or_else(|| M::Error::missing_field("max_ttl"))?;
                let traceroute_queries = traceroute_queries
                    .ok_or_else(|| M::Error::missing_field("traceroute_queries"))?;

                let content = SyntheticsNetworkTestRequest {
                    destination_service,
                    e2e_queries,
                    host,
                    max_ttl,
                    port,
                    source_service,
                    tcp_method,
                    timeout,
                    traceroute_queries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsNetworkTestRequestVisitor)
    }
}
