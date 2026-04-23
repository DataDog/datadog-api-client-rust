// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the response received during the test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultResponseInfo {
    /// Body of the response.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Compressed representation of the response body.
    #[serde(rename = "body_compressed")]
    pub body_compressed: Option<String>,
    /// Hashes computed over the response body.
    #[serde(rename = "body_hashes")]
    pub body_hashes: Option<String>,
    /// Size of the response body in bytes.
    #[serde(rename = "body_size")]
    pub body_size: Option<i64>,
    /// Cache-related response headers.
    #[serde(rename = "cache_headers")]
    pub cache_headers: Option<std::collections::BTreeMap<String, String>>,
    /// CDN provider details inferred from response headers.
    #[serde(rename = "cdn")]
    pub cdn: Option<crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo>,
    /// WebSocket close frame information for WebSocket test responses.
    #[serde(rename = "close")]
    pub close: Option<crate::datadogV2::model::SyntheticsTestResultWebSocketClose>,
    /// Compressed representation of the response message.
    #[serde(rename = "compressed_message")]
    pub compressed_message: Option<String>,
    /// Response headers.
    #[serde(rename = "headers")]
    pub headers: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Health check information returned from a gRPC health check call.
    #[serde(rename = "healthcheck")]
    pub healthcheck: Option<crate::datadogV2::model::SyntheticsTestResultHealthCheck>,
    /// HTTP version of the response.
    #[serde(rename = "http_version")]
    pub http_version: Option<String>,
    /// Whether the response body was truncated.
    #[serde(rename = "is_body_truncated")]
    pub is_body_truncated: Option<bool>,
    /// Whether the response message was truncated.
    #[serde(rename = "is_message_truncated")]
    pub is_message_truncated: Option<bool>,
    /// Message received in the response (for WebSocket/TCP/UDP tests).
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Additional metadata returned with the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    /// DNS records returned in the response (DNS tests only).
    #[serde(rename = "records")]
    pub records: Option<Vec<crate::datadogV2::model::SyntheticsTestResultDnsRecord>>,
    /// Redirect hops encountered while performing the request.
    #[serde(rename = "redirects")]
    pub redirects: Option<Vec<crate::datadogV2::model::SyntheticsTestResultRedirect>>,
    /// HTTP status code of the response.
    #[serde(rename = "status_code")]
    pub status_code: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultResponseInfo {
    pub fn new() -> SyntheticsTestResultResponseInfo {
        SyntheticsTestResultResponseInfo {
            body: None,
            body_compressed: None,
            body_hashes: None,
            body_size: None,
            cache_headers: None,
            cdn: None,
            close: None,
            compressed_message: None,
            headers: None,
            healthcheck: None,
            http_version: None,
            is_body_truncated: None,
            is_message_truncated: None,
            message: None,
            metadata: None,
            records: None,
            redirects: None,
            status_code: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }

    pub fn body_compressed(mut self, value: String) -> Self {
        self.body_compressed = Some(value);
        self
    }

    pub fn body_hashes(mut self, value: String) -> Self {
        self.body_hashes = Some(value);
        self
    }

    pub fn body_size(mut self, value: i64) -> Self {
        self.body_size = Some(value);
        self
    }

    pub fn cache_headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.cache_headers = Some(value);
        self
    }

    pub fn cdn(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo,
    ) -> Self {
        self.cdn = Some(value);
        self
    }

    pub fn close(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultWebSocketClose,
    ) -> Self {
        self.close = Some(value);
        self
    }

    pub fn compressed_message(mut self, value: String) -> Self {
        self.compressed_message = Some(value);
        self
    }

    pub fn headers(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn healthcheck(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultHealthCheck,
    ) -> Self {
        self.healthcheck = Some(value);
        self
    }

    pub fn http_version(mut self, value: String) -> Self {
        self.http_version = Some(value);
        self
    }

    pub fn is_body_truncated(mut self, value: bool) -> Self {
        self.is_body_truncated = Some(value);
        self
    }

    pub fn is_message_truncated(mut self, value: bool) -> Self {
        self.is_message_truncated = Some(value);
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

    pub fn records(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultDnsRecord>,
    ) -> Self {
        self.records = Some(value);
        self
    }

    pub fn redirects(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultRedirect>,
    ) -> Self {
        self.redirects = Some(value);
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
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

impl Default for SyntheticsTestResultResponseInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultResponseInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultResponseInfoVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultResponseInfoVisitor {
            type Value = SyntheticsTestResultResponseInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut body: Option<String> = None;
                let mut body_compressed: Option<String> = None;
                let mut body_hashes: Option<String> = None;
                let mut body_size: Option<i64> = None;
                let mut cache_headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut cdn: Option<crate::datadogV2::model::SyntheticsTestResultCdnProviderInfo> =
                    None;
                let mut close: Option<crate::datadogV2::model::SyntheticsTestResultWebSocketClose> =
                    None;
                let mut compressed_message: Option<String> = None;
                let mut headers: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut healthcheck: Option<
                    crate::datadogV2::model::SyntheticsTestResultHealthCheck,
                > = None;
                let mut http_version: Option<String> = None;
                let mut is_body_truncated: Option<bool> = None;
                let mut is_message_truncated: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut metadata: Option<std::collections::BTreeMap<String, String>> = None;
                let mut records: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultDnsRecord>,
                > = None;
                let mut redirects: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultRedirect>,
                > = None;
                let mut status_code: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "body_compressed" => {
                            if v.is_null() {
                                continue;
                            }
                            body_compressed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "body_hashes" => {
                            if v.is_null() {
                                continue;
                            }
                            body_hashes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "body_size" => {
                            if v.is_null() {
                                continue;
                            }
                            body_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cache_headers" => {
                            if v.is_null() {
                                continue;
                            }
                            cache_headers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cdn" => {
                            if v.is_null() {
                                continue;
                            }
                            cdn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "close" => {
                            if v.is_null() {
                                continue;
                            }
                            close = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compressed_message" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "healthcheck" => {
                            if v.is_null() {
                                continue;
                            }
                            healthcheck =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "http_version" => {
                            if v.is_null() {
                                continue;
                            }
                            http_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_body_truncated" => {
                            if v.is_null() {
                                continue;
                            }
                            is_body_truncated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_message_truncated" => {
                            if v.is_null() {
                                continue;
                            }
                            is_message_truncated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "records" => {
                            if v.is_null() {
                                continue;
                            }
                            records = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "redirects" => {
                            if v.is_null() {
                                continue;
                            }
                            redirects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_code" => {
                            if v.is_null() {
                                continue;
                            }
                            status_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultResponseInfo {
                    body,
                    body_compressed,
                    body_hashes,
                    body_size,
                    cache_headers,
                    cdn,
                    close,
                    compressed_message,
                    headers,
                    healthcheck,
                    http_version,
                    is_body_truncated,
                    is_message_truncated,
                    message,
                    metadata,
                    records,
                    redirects,
                    status_code,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultResponseInfoVisitor)
    }
}
