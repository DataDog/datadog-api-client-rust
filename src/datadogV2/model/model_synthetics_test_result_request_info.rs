// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the outgoing request made during the test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultRequestInfo {
    /// Whether insecure certificates are allowed for this request.
    #[serde(rename = "allow_insecure")]
    pub allow_insecure: Option<bool>,
    /// Body sent with the request.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// gRPC call type (for example, `unary`, `healthCheck`, or `reflection`).
    #[serde(rename = "call_type")]
    pub call_type: Option<String>,
    /// Destination service for a Network Path test.
    #[serde(rename = "destination_service")]
    pub destination_service: Option<String>,
    /// DNS server used to resolve the target host.
    #[serde(rename = "dns_server")]
    pub dns_server: Option<String>,
    /// Port of the DNS server used for resolution.
    #[serde(rename = "dns_server_port")]
    pub dns_server_port: Option<i64>,
    /// Number of end-to-end probe queries issued.
    #[serde(rename = "e2e_queries")]
    pub e2e_queries: Option<i64>,
    /// Files attached to the request.
    #[serde(rename = "files")]
    pub files: Option<Vec<crate::datadogV2::model::SyntheticsTestResultFileRef>>,
    /// Headers sent with the request.
    #[serde(rename = "headers")]
    pub headers: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Host targeted by the request.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Maximum TTL for network probe packets.
    #[serde(rename = "max_ttl")]
    pub max_ttl: Option<i64>,
    /// Message sent with the request (for WebSocket/TCP/UDP tests).
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// HTTP method used for the request.
    #[serde(rename = "method")]
    pub method: Option<String>,
    /// Whether the response body was not saved.
    #[serde(rename = "no_saving_response_body")]
    pub no_saving_response_body: Option<bool>,
    /// Port targeted by the request. Can be a number or a string variable reference.
    #[serde(rename = "port")]
    pub port: Option<serde_json::Value>,
    /// Service name targeted by the request (for gRPC tests).
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Source service for a Network Path test.
    #[serde(rename = "source_service")]
    pub source_service: Option<String>,
    /// Request timeout in milliseconds.
    #[serde(rename = "timeout")]
    pub timeout: Option<i64>,
    /// Name of the MCP tool called (MCP tests only).
    #[serde(rename = "tool_name")]
    pub tool_name: Option<String>,
    /// Number of traceroute probe queries issued.
    #[serde(rename = "traceroute_queries")]
    pub traceroute_queries: Option<i64>,
    /// URL targeted by the request.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultRequestInfo {
    pub fn new() -> SyntheticsTestResultRequestInfo {
        SyntheticsTestResultRequestInfo {
            allow_insecure: None,
            body: None,
            call_type: None,
            destination_service: None,
            dns_server: None,
            dns_server_port: None,
            e2e_queries: None,
            files: None,
            headers: None,
            host: None,
            max_ttl: None,
            message: None,
            method: None,
            no_saving_response_body: None,
            port: None,
            service: None,
            source_service: None,
            timeout: None,
            tool_name: None,
            traceroute_queries: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_insecure(mut self, value: bool) -> Self {
        self.allow_insecure = Some(value);
        self
    }

    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }

    pub fn call_type(mut self, value: String) -> Self {
        self.call_type = Some(value);
        self
    }

    pub fn destination_service(mut self, value: String) -> Self {
        self.destination_service = Some(value);
        self
    }

    pub fn dns_server(mut self, value: String) -> Self {
        self.dns_server = Some(value);
        self
    }

    pub fn dns_server_port(mut self, value: i64) -> Self {
        self.dns_server_port = Some(value);
        self
    }

    pub fn e2e_queries(mut self, value: i64) -> Self {
        self.e2e_queries = Some(value);
        self
    }

    pub fn files(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultFileRef>,
    ) -> Self {
        self.files = Some(value);
        self
    }

    pub fn headers(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn max_ttl(mut self, value: i64) -> Self {
        self.max_ttl = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn method(mut self, value: String) -> Self {
        self.method = Some(value);
        self
    }

    pub fn no_saving_response_body(mut self, value: bool) -> Self {
        self.no_saving_response_body = Some(value);
        self
    }

    pub fn port(mut self, value: serde_json::Value) -> Self {
        self.port = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn source_service(mut self, value: String) -> Self {
        self.source_service = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn tool_name(mut self, value: String) -> Self {
        self.tool_name = Some(value);
        self
    }

    pub fn traceroute_queries(mut self, value: i64) -> Self {
        self.traceroute_queries = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for SyntheticsTestResultRequestInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultRequestInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultRequestInfoVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultRequestInfoVisitor {
            type Value = SyntheticsTestResultRequestInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_insecure: Option<bool> = None;
                let mut body: Option<String> = None;
                let mut call_type: Option<String> = None;
                let mut destination_service: Option<String> = None;
                let mut dns_server: Option<String> = None;
                let mut dns_server_port: Option<i64> = None;
                let mut e2e_queries: Option<i64> = None;
                let mut files: Option<Vec<crate::datadogV2::model::SyntheticsTestResultFileRef>> =
                    None;
                let mut headers: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut host: Option<String> = None;
                let mut max_ttl: Option<i64> = None;
                let mut message: Option<String> = None;
                let mut method: Option<String> = None;
                let mut no_saving_response_body: Option<bool> = None;
                let mut port: Option<serde_json::Value> = None;
                let mut service: Option<String> = None;
                let mut source_service: Option<String> = None;
                let mut timeout: Option<i64> = None;
                let mut tool_name: Option<String> = None;
                let mut traceroute_queries: Option<i64> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_insecure" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_insecure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "call_type" => {
                            if v.is_null() {
                                continue;
                            }
                            call_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "destination_service" => {
                            if v.is_null() {
                                continue;
                            }
                            destination_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_server" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_server = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_server_port" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_server_port =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "e2e_queries" => {
                            if v.is_null() {
                                continue;
                            }
                            e2e_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "files" => {
                            if v.is_null() {
                                continue;
                            }
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "max_ttl" => {
                            if v.is_null() {
                                continue;
                            }
                            max_ttl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            if v.is_null() {
                                continue;
                            }
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "no_saving_response_body" => {
                            if v.is_null() {
                                continue;
                            }
                            no_saving_response_body =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "port" => {
                            if v.is_null() {
                                continue;
                            }
                            port = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_service" => {
                            if v.is_null() {
                                continue;
                            }
                            source_service =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceroute_queries" => {
                            if v.is_null() {
                                continue;
                            }
                            traceroute_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultRequestInfo {
                    allow_insecure,
                    body,
                    call_type,
                    destination_service,
                    dns_server,
                    dns_server_port,
                    e2e_queries,
                    files,
                    headers,
                    host,
                    max_ttl,
                    message,
                    method,
                    no_saving_response_body,
                    port,
                    service,
                    source_service,
                    timeout,
                    tool_name,
                    traceroute_queries,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultRequestInfoVisitor)
    }
}
