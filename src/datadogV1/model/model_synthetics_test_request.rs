// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the Synthetic test request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRequest {
    /// Allows loading insecure content for an HTTP request in a multistep test step.
    #[serde(rename = "allow_insecure")]
    pub allow_insecure: Option<bool>,
    /// Object to handle basic authentication when performing the test.
    #[serde(rename = "basicAuth")]
    pub basic_auth: Option<crate::datadogV1::model::SyntheticsBasicAuth>,
    /// Body to include in the test.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Type of the request body.
    #[serde(rename = "bodyType")]
    pub body_type: Option<crate::datadogV1::model::SyntheticsTestRequestBodyType>,
    /// The type of gRPC call to perform.
    #[serde(rename = "callType")]
    pub call_type: Option<crate::datadogV1::model::SyntheticsTestCallType>,
    /// Client certificate to use when performing the test request.
    #[serde(rename = "certificate")]
    pub certificate: Option<crate::datadogV1::model::SyntheticsTestRequestCertificate>,
    /// By default, the client certificate is applied on the domain of the starting URL for browser tests. If you want your client certificate to be applied on other domains instead, add them in `certificateDomains`.
    #[serde(rename = "certificateDomains")]
    pub certificate_domains: Option<Vec<String>>,
    /// A protobuf JSON descriptor that needs to be gzipped first then base64 encoded.
    #[serde(rename = "compressedJsonDescriptor")]
    pub compressed_json_descriptor: Option<String>,
    /// A protobuf file that needs to be gzipped first then base64 encoded.
    #[serde(rename = "compressedProtoFile")]
    pub compressed_proto_file: Option<String>,
    /// DNS server to use for DNS tests.
    #[serde(rename = "dnsServer")]
    pub dns_server: Option<String>,
    /// DNS server port to use for DNS tests.
    #[serde(rename = "dnsServerPort")]
    pub dns_server_port: Option<i32>,
    /// Specifies whether or not the request follows redirects.
    #[serde(rename = "follow_redirects")]
    pub follow_redirects: Option<bool>,
    /// Headers to include when performing the test.
    #[serde(rename = "headers")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    /// Host name to perform the test with.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// HTTP version to use for a Synthetic test.
    #[serde(rename = "httpVersion")]
    pub http_version: Option<crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion>,
    /// Message to send for UDP or WebSocket tests.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Metadata to include when performing the gRPC test.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    /// Either the HTTP method/verb to use or a gRPC method available on the service set in the `service` field. Required if `subtype` is `HTTP` or if `subtype` is `grpc` and `callType` is `unary`.
    #[serde(rename = "method")]
    pub method: Option<String>,
    /// Determines whether or not to save the response body.
    #[serde(rename = "noSavingResponseBody")]
    pub no_saving_response_body: Option<bool>,
    /// Number of pings to use per test.
    #[serde(rename = "numberOfPackets")]
    pub number_of_packets: Option<i32>,
    /// Persist cookies across redirects.
    #[serde(rename = "persistCookies")]
    pub persist_cookies: Option<bool>,
    /// Port to use when performing the test.
    #[serde(rename = "port")]
    pub port: Option<i64>,
    /// The proxy to perform the test.
    #[serde(rename = "proxy")]
    pub proxy: Option<crate::datadogV1::model::SyntheticsTestRequestProxy>,
    /// Query to use for the test.
    #[serde(rename = "query")]
    pub query: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// For SSL tests, it specifies on which server you want to initiate the TLS handshake,
    /// allowing the server to present one of multiple possible certificates on
    /// the same IP address and TCP port number.
    #[serde(rename = "servername")]
    pub servername: Option<String>,
    /// The gRPC service on which you want to perform the gRPC call.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Turns on a traceroute probe to discover all gateways along the path to the host destination.
    #[serde(rename = "shouldTrackHops")]
    pub should_track_hops: Option<bool>,
    /// Timeout in seconds for the test.
    #[serde(rename = "timeout")]
    pub timeout: Option<f64>,
    /// URL to perform the test with.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRequest {
    pub fn new() -> SyntheticsTestRequest {
        SyntheticsTestRequest {
            allow_insecure: None,
            basic_auth: None,
            body: None,
            body_type: None,
            call_type: None,
            certificate: None,
            certificate_domains: None,
            compressed_json_descriptor: None,
            compressed_proto_file: None,
            dns_server: None,
            dns_server_port: None,
            follow_redirects: None,
            headers: None,
            host: None,
            http_version: None,
            message: None,
            metadata: None,
            method: None,
            no_saving_response_body: None,
            number_of_packets: None,
            persist_cookies: None,
            port: None,
            proxy: None,
            query: None,
            servername: None,
            service: None,
            should_track_hops: None,
            timeout: None,
            url: None,
            _unparsed: false,
        }
    }

    pub fn allow_insecure(mut self, value: bool) -> Self {
        self.allow_insecure = Some(value);
        self
    }

    pub fn basic_auth(mut self, value: crate::datadogV1::model::SyntheticsBasicAuth) -> Self {
        self.basic_auth = Some(value);
        self
    }

    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }

    pub fn body_type(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestBodyType,
    ) -> Self {
        self.body_type = Some(value);
        self
    }

    pub fn call_type(mut self, value: crate::datadogV1::model::SyntheticsTestCallType) -> Self {
        self.call_type = Some(value);
        self
    }

    pub fn certificate(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestRequestCertificate,
    ) -> Self {
        self.certificate = Some(value);
        self
    }

    pub fn certificate_domains(mut self, value: Vec<String>) -> Self {
        self.certificate_domains = Some(value);
        self
    }

    pub fn compressed_json_descriptor(mut self, value: String) -> Self {
        self.compressed_json_descriptor = Some(value);
        self
    }

    pub fn compressed_proto_file(mut self, value: String) -> Self {
        self.compressed_proto_file = Some(value);
        self
    }

    pub fn dns_server(mut self, value: String) -> Self {
        self.dns_server = Some(value);
        self
    }

    pub fn dns_server_port(mut self, value: i32) -> Self {
        self.dns_server_port = Some(value);
        self
    }

    pub fn follow_redirects(mut self, value: bool) -> Self {
        self.follow_redirects = Some(value);
        self
    }

    pub fn headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn http_version(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion,
    ) -> Self {
        self.http_version = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn metadata(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.metadata = Some(value);
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

    pub fn number_of_packets(mut self, value: i32) -> Self {
        self.number_of_packets = Some(value);
        self
    }

    pub fn persist_cookies(mut self, value: bool) -> Self {
        self.persist_cookies = Some(value);
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    pub fn proxy(mut self, value: crate::datadogV1::model::SyntheticsTestRequestProxy) -> Self {
        self.proxy = Some(value);
        self
    }

    pub fn query(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.query = Some(value);
        self
    }

    pub fn servername(mut self, value: String) -> Self {
        self.servername = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn should_track_hops(mut self, value: bool) -> Self {
        self.should_track_hops = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for SyntheticsTestRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRequestVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRequestVisitor {
            type Value = SyntheticsTestRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_insecure: Option<bool> = None;
                let mut basic_auth: Option<crate::datadogV1::model::SyntheticsBasicAuth> = None;
                let mut body: Option<String> = None;
                let mut body_type: Option<crate::datadogV1::model::SyntheticsTestRequestBodyType> =
                    None;
                let mut call_type: Option<crate::datadogV1::model::SyntheticsTestCallType> = None;
                let mut certificate: Option<
                    crate::datadogV1::model::SyntheticsTestRequestCertificate,
                > = None;
                let mut certificate_domains: Option<Vec<String>> = None;
                let mut compressed_json_descriptor: Option<String> = None;
                let mut compressed_proto_file: Option<String> = None;
                let mut dns_server: Option<String> = None;
                let mut dns_server_port: Option<i32> = None;
                let mut follow_redirects: Option<bool> = None;
                let mut headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut host: Option<String> = None;
                let mut http_version: Option<
                    crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion,
                > = None;
                let mut message: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut method: Option<String> = None;
                let mut no_saving_response_body: Option<bool> = None;
                let mut number_of_packets: Option<i32> = None;
                let mut persist_cookies: Option<bool> = None;
                let mut port: Option<i64> = None;
                let mut proxy: Option<crate::datadogV1::model::SyntheticsTestRequestProxy> = None;
                let mut query: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut servername: Option<String> = None;
                let mut service: Option<String> = None;
                let mut should_track_hops: Option<bool> = None;
                let mut timeout: Option<f64> = None;
                let mut url: Option<String> = None;
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
                        "basicAuth" => {
                            if v.is_null() {
                                continue;
                            }
                            basic_auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _basic_auth) = basic_auth {
                                match _basic_auth {
                                    crate::datadogV1::model::SyntheticsBasicAuth::UnparsedObject(_basic_auth) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bodyType" => {
                            if v.is_null() {
                                continue;
                            }
                            body_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _body_type) = body_type {
                                match _body_type {
                                    crate::datadogV1::model::SyntheticsTestRequestBodyType::UnparsedObject(_body_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "callType" => {
                            if v.is_null() {
                                continue;
                            }
                            call_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _call_type) = call_type {
                                match _call_type {
                                    crate::datadogV1::model::SyntheticsTestCallType::UnparsedObject(_call_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "certificate" => {
                            if v.is_null() {
                                continue;
                            }
                            certificate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "certificateDomains" => {
                            if v.is_null() {
                                continue;
                            }
                            certificate_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compressedJsonDescriptor" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_json_descriptor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compressedProtoFile" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_proto_file =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dnsServer" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_server = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dnsServerPort" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_server_port =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "follow_redirects" => {
                            if v.is_null() {
                                continue;
                            }
                            follow_redirects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "httpVersion" => {
                            if v.is_null() {
                                continue;
                            }
                            http_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _http_version) = http_version {
                                match _http_version {
                                    crate::datadogV1::model::SyntheticsTestOptionsHTTPVersion::UnparsedObject(_http_version) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "method" => {
                            if v.is_null() {
                                continue;
                            }
                            method = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "noSavingResponseBody" => {
                            if v.is_null() {
                                continue;
                            }
                            no_saving_response_body =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numberOfPackets" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_packets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "persistCookies" => {
                            if v.is_null() {
                                continue;
                            }
                            persist_cookies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "port" => {
                            if v.is_null() {
                                continue;
                            }
                            port = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "proxy" => {
                            if v.is_null() {
                                continue;
                            }
                            proxy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "servername" => {
                            if v.is_null() {
                                continue;
                            }
                            servername = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "shouldTrackHops" => {
                            if v.is_null() {
                                continue;
                            }
                            should_track_hops =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestRequest {
                    allow_insecure,
                    basic_auth,
                    body,
                    body_type,
                    call_type,
                    certificate,
                    certificate_domains,
                    compressed_json_descriptor,
                    compressed_proto_file,
                    dns_server,
                    dns_server_port,
                    follow_redirects,
                    headers,
                    host,
                    http_version,
                    message,
                    metadata,
                    method,
                    no_saving_response_body,
                    number_of_packets,
                    persist_cookies,
                    port,
                    proxy,
                    query,
                    servername,
                    service,
                    should_track_hops,
                    timeout,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRequestVisitor)
    }
}
