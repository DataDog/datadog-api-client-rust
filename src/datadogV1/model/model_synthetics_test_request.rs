// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the Synthetic test request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequest {
    /// Allows loading insecure content for an HTTP request in a multistep test step.
    #[serde(rename = "allow_insecure")]
    pub allow_insecure: Option<bool>,
    /// Object to handle basic authentication when performing the test.
    #[serde(rename = "basicAuth")]
    pub basic_auth: Option<Box<crate::datadogV1::model::SyntheticsBasicAuth>>,
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
    pub certificate: Option<Box<crate::datadogV1::model::SyntheticsTestRequestCertificate>>,
    /// By default, the client certificate is applied on the domain of the starting URL for browser tests. If you want your client certificate to be applied on other domains instead, add them in `certificateDomains`.
    #[serde(rename = "certificateDomains")]
    pub certificate_domains: Option<Vec<String>>,
    /// A protobuf JSON descriptor that needs to be gzipped first then base64 encoded.
    #[serde(rename = "compressedJsonDescriptor")]
    pub compressed_json_descriptor: Option<String>,
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
    pub proxy: Option<Box<crate::datadogV1::model::SyntheticsTestRequestProxy>>,
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
            dns_server: None,
            dns_server_port: None,
            follow_redirects: None,
            headers: None,
            host: None,
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
        }
    }
}
impl Default for SyntheticsTestRequest {
    fn default() -> Self {
        Self::new()
    }
}
