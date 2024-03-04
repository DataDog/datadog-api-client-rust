// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing results for your Synthetic API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultData {
    /// Object describing the SSL certificate used for a Synthetic test.
    #[serde(rename = "cert")]
    pub cert: Option<crate::datadogV1::model::SyntheticsSSLCertificate>,
    /// Status of a Synthetic test.
    #[serde(rename = "eventType")]
    pub event_type: Option<crate::datadogV1::model::SyntheticsTestProcessStatus>,
    /// The API test failure details.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV1::model::SyntheticsApiTestResultFailure>,
    /// The API test HTTP status code.
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: Option<i64>,
    /// Request header object used for the API test.
    #[serde(rename = "requestHeaders")]
    pub request_headers: Option<
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, serde_json::Value>>,
    >,
    /// Response body returned for the API test.
    #[serde(rename = "responseBody")]
    pub response_body: Option<String>,
    /// Response headers returned for the API test.
    #[serde(rename = "responseHeaders")]
    pub response_headers: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Global size in byte of the API test response.
    #[serde(rename = "responseSize")]
    pub response_size: Option<i64>,
    /// Object containing all metrics and their values collected for a Synthetic API test.
    /// See the [Synthetic Monitoring Metrics documentation](<https://docs.datadoghq.com/synthetics/metrics/>).
    #[serde(rename = "timings")]
    pub timings: Option<crate::datadogV1::model::SyntheticsTiming>,
}

impl SyntheticsAPITestResultData {
    pub fn new() -> SyntheticsAPITestResultData {
        SyntheticsAPITestResultData {
            cert: None,
            event_type: None,
            failure: None,
            http_status_code: None,
            request_headers: None,
            response_body: None,
            response_headers: None,
            response_size: None,
            timings: None,
        }
    }

    pub fn cert(mut self, value: crate::datadogV1::model::SyntheticsSSLCertificate) -> Self {
        self.cert = Some(value);
        self
    }

    pub fn event_type(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestProcessStatus,
    ) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn failure(
        mut self,
        value: crate::datadogV1::model::SyntheticsApiTestResultFailure,
    ) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn http_status_code(mut self, value: i64) -> Self {
        self.http_status_code = Some(value);
        self
    }

    pub fn request_headers(
        mut self,
        value: std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, serde_json::Value>,
        >,
    ) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn response_body(mut self, value: String) -> Self {
        self.response_body = Some(value);
        self
    }

    pub fn response_headers(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.response_headers = Some(value);
        self
    }

    pub fn response_size(mut self, value: i64) -> Self {
        self.response_size = Some(value);
        self
    }

    pub fn timings(mut self, value: crate::datadogV1::model::SyntheticsTiming) -> Self {
        self.timings = Some(value);
        self
    }
}

impl Default for SyntheticsAPITestResultData {
    fn default() -> Self {
        Self::new()
    }
}
