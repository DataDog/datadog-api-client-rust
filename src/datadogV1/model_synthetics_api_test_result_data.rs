// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultData {
    /// Object describing the SSL certificate used for a Synthetic test.
    #[serde(rename = "cert", skip_serializing_if = "Option::is_none")]
    pub cert: SyntheticsSSLCertificate,
    /// Status of a Synthetic test.
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: SyntheticsTestProcessStatus,
    /// The API test failure details.
    #[serde(rename = "failure", skip_serializing_if = "Option::is_none")]
    pub failure: SyntheticsApiTestResultFailure,
    /// The API test HTTP status code.
    #[serde(rename = "httpStatusCode", skip_serializing_if = "Option::is_none")]
    pub http_status_code: i64,
    /// Request header object used for the API test.
    #[serde(rename = "requestHeaders", skip_serializing_if = "Option::is_none")]
    pub request_headers: map[string]interface{},
    /// Response body returned for the API test.
    #[serde(rename = "responseBody", skip_serializing_if = "Option::is_none")]
    pub response_body: String,
    /// Response headers returned for the API test.
    #[serde(rename = "responseHeaders", skip_serializing_if = "Option::is_none")]
    pub response_headers: map[string]interface{},
    /// Global size in byte of the API test response.
    #[serde(rename = "responseSize", skip_serializing_if = "Option::is_none")]
    pub response_size: i64,
    /// Object containing all metrics and their values collected for a Synthetic API test.
See the [Synthetic Monitoring Metrics documentation](https://docs.datadoghq.com/synthetics/metrics/).
    #[serde(rename = "timings", skip_serializing_if = "Option::is_none")]
    pub timings: SyntheticsTiming,
}

