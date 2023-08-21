// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestRequest {
    /// Allows loading insecure content for an HTTP request in a multistep test step.
    #[serde(rename = "allow_insecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: bool,
    /// Object to handle basic authentication when performing the test.
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: SyntheticsBasicAuth,
    /// Body to include in the test.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: String,
    /// Type of the request body.
    #[serde(rename = "bodyType", skip_serializing_if = "Option::is_none")]
    pub body_type: SyntheticsTestRequestBodyType,
    /// The type of gRPC call to perform.
    #[serde(rename = "callType", skip_serializing_if = "Option::is_none")]
    pub call_type: SyntheticsTestCallType,
    /// Client certificate to use when performing the test request.
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: SyntheticsTestRequestCertificate,
    /// By default, the client certificate is applied on the domain of the starting URL for browser tests. If you want your client certificate to be applied on other domains instead, add them in `certificateDomains`.
    #[serde(rename = "certificateDomains", skip_serializing_if = "Option::is_none")]
    pub certificate_domains: Vec<String>,
    /// A protobuf JSON descriptor that needs to be gzipped first then base64 encoded.
    #[serde(rename = "compressedJsonDescriptor", skip_serializing_if = "Option::is_none")]
    pub compressed_json_descriptor: String,
    /// DNS server to use for DNS tests.
    #[serde(rename = "dnsServer", skip_serializing_if = "Option::is_none")]
    pub dns_server: String,
    /// DNS server port to use for DNS tests.
    #[serde(rename = "dnsServerPort", skip_serializing_if = "Option::is_none")]
    pub dns_server_port: i32,
    /// Specifies whether or not the request follows redirects.
    #[serde(rename = "follow_redirects", skip_serializing_if = "Option::is_none")]
    pub follow_redirects: bool,
    /// Headers to include when performing the test.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: map[string]String,
    /// Host name to perform the test with.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// Message to send for UDP or WebSocket tests.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// Metadata to include when performing the gRPC test.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: map[string]String,
    /// Either the HTTP method/verb to use or a gRPC method available on the service set in the `service` field. Required if `subtype` is `HTTP` or if `subtype` is `grpc` and `callType` is `unary`.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: String,
    /// Determines whether or not to save the response body.
    #[serde(rename = "noSavingResponseBody", skip_serializing_if = "Option::is_none")]
    pub no_saving_response_body: bool,
    /// Number of pings to use per test.
    #[serde(rename = "numberOfPackets", skip_serializing_if = "Option::is_none")]
    pub number_of_packets: i32,
    /// Port to use when performing the test.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: i64,
    /// The proxy to perform the test.
    #[serde(rename = "proxy")]
    pub proxy: SyntheticsTestRequestProxy,
    /// Query to use for the test.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: interface{},
    /// For SSL tests, it specifies on which server you want to initiate the TLS handshake,
allowing the server to present one of multiple possible certificates on
the same IP address and TCP port number.
    #[serde(rename = "servername", skip_serializing_if = "Option::is_none")]
    pub servername: String,
    /// The gRPC service on which you want to perform the gRPC call.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// Turns on a traceroute probe to discover all gateways along the path to the host destination.
    #[serde(rename = "shouldTrackHops", skip_serializing_if = "Option::is_none")]
    pub should_track_hops: bool,
    /// Timeout in seconds for the test.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: f64,
    /// URL to perform the test with.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

