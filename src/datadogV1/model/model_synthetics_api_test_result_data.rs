// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing results for your Synthetic API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
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

impl<'de> Deserialize<'de> for SyntheticsAPITestResultData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestResultDataVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestResultDataVisitor {
            type Value = SyntheticsAPITestResultData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cert: Option<crate::datadogV1::model::SyntheticsSSLCertificate> = None;
                let mut event_type: Option<crate::datadogV1::model::SyntheticsTestProcessStatus> =
                    None;
                let mut failure: Option<crate::datadogV1::model::SyntheticsApiTestResultFailure> =
                    None;
                let mut http_status_code: Option<i64> = None;
                let mut request_headers: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, serde_json::Value>,
                    >,
                > = None;
                let mut response_body: Option<String> = None;
                let mut response_headers: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut response_size: Option<i64> = None;
                let mut timings: Option<crate::datadogV1::model::SyntheticsTiming> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cert" => {
                            if v.is_null() {
                                continue;
                            }
                            cert = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "eventType" => {
                            if v.is_null() {
                                continue;
                            }
                            event_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _event_type) = event_type {
                                match _event_type {
                                    crate::datadogV1::model::SyntheticsTestProcessStatus::UnparsedObject(_event_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "failure" => {
                            if v.is_null() {
                                continue;
                            }
                            failure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "httpStatusCode" => {
                            if v.is_null() {
                                continue;
                            }
                            http_status_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requestHeaders" => {
                            if v.is_null() {
                                continue;
                            }
                            request_headers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responseBody" => {
                            if v.is_null() {
                                continue;
                            }
                            response_body =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responseHeaders" => {
                            if v.is_null() {
                                continue;
                            }
                            response_headers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "responseSize" => {
                            if v.is_null() {
                                continue;
                            }
                            response_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsAPITestResultData {
                    cert,
                    event_type,
                    failure,
                    http_status_code,
                    request_headers,
                    response_body,
                    response_headers,
                    response_size,
                    timings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestResultDataVisitor)
    }
}
