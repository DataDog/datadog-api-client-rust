// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Detailed result data for the fast test run. The exact shape of nested fields
/// (`request`, `response`, `assertions`, etc.) depends on the test subtype.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsFastTestResultDetail {
    /// Results of each assertion evaluated during the test.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// gRPC call type (for example, `unary`, `healthCheck`, or `reflection`).
    #[serde(rename = "call_type")]
    pub call_type: Option<String>,
    /// TLS certificate details, present for SSL tests.
    #[serde(rename = "cert")]
    pub cert: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Total duration of the test in milliseconds.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Failure details if the fast test did not pass.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV2::model::SyntheticsFastTestResultFailure>,
    /// Unix timestamp (ms) of when the test finished.
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// The result ID. Set to the fast test UUID because no persistent result ID exists for fast tests.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether this result is from an automatic fast retry.
    #[serde(rename = "is_fast_retry")]
    pub is_fast_retry: Option<bool>,
    /// Details of the outgoing request made during the test.
    #[serde(rename = "request")]
    pub request: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// IP address resolved for the target host.
    #[serde(rename = "resolved_ip")]
    pub resolved_ip: Option<String>,
    /// Details of the response received during the test.
    #[serde(rename = "response")]
    pub response: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Run type indicating how this test was triggered (for example, `fast`).
    #[serde(rename = "run_type")]
    pub run_type: Option<String>,
    /// Unix timestamp (ms) of when the test started.
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    /// Status of the test result (`passed` or `failed`).
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Step results for multistep API tests.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Timing breakdown of the test request phases (for example, DNS, TCP, TLS, first byte).
    #[serde(rename = "timings")]
    pub timings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Traceroute hop results, present for ICMP and TCP tests.
    #[serde(rename = "traceroute")]
    pub traceroute: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Unix timestamp (ms) of when the test was triggered.
    #[serde(rename = "triggered_at")]
    pub triggered_at: Option<i64>,
    /// Whether the test was run through a Synthetics tunnel.
    #[serde(rename = "tunnel")]
    pub tunnel: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsFastTestResultDetail {
    pub fn new() -> SyntheticsFastTestResultDetail {
        SyntheticsFastTestResultDetail {
            assertions: None,
            call_type: None,
            cert: None,
            duration: None,
            failure: None,
            finished_at: None,
            id: None,
            is_fast_retry: None,
            request: None,
            resolved_ip: None,
            response: None,
            run_type: None,
            started_at: None,
            status: None,
            steps: None,
            timings: None,
            traceroute: None,
            triggered_at: None,
            tunnel: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assertions(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.assertions = Some(value);
        self
    }

    pub fn call_type(mut self, value: String) -> Self {
        self.call_type = Some(value);
        self
    }

    pub fn cert(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.cert = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn failure(
        mut self,
        value: crate::datadogV2::model::SyntheticsFastTestResultFailure,
    ) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_fast_retry(mut self, value: bool) -> Self {
        self.is_fast_retry = Some(value);
        self
    }

    pub fn request(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.request = Some(value);
        self
    }

    pub fn resolved_ip(mut self, value: String) -> Self {
        self.resolved_ip = Some(value);
        self
    }

    pub fn response(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.response = Some(value);
        self
    }

    pub fn run_type(mut self, value: String) -> Self {
        self.run_type = Some(value);
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    pub fn steps(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn timings(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.timings = Some(value);
        self
    }

    pub fn traceroute(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.traceroute = Some(value);
        self
    }

    pub fn triggered_at(mut self, value: i64) -> Self {
        self.triggered_at = Some(value);
        self
    }

    pub fn tunnel(mut self, value: bool) -> Self {
        self.tunnel = Some(value);
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

impl Default for SyntheticsFastTestResultDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsFastTestResultDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsFastTestResultDetailVisitor;
        impl<'a> Visitor<'a> for SyntheticsFastTestResultDetailVisitor {
            type Value = SyntheticsFastTestResultDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assertions: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut call_type: Option<String> = None;
                let mut cert: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut duration: Option<f64> = None;
                let mut failure: Option<crate::datadogV2::model::SyntheticsFastTestResultFailure> =
                    None;
                let mut finished_at: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut is_fast_retry: Option<bool> = None;
                let mut request: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut resolved_ip: Option<String> = None;
                let mut response: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut run_type: Option<String> = None;
                let mut started_at: Option<i64> = None;
                let mut status: Option<String> = None;
                let mut steps: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>> =
                    None;
                let mut timings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut traceroute: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut triggered_at: Option<i64> = None;
                let mut tunnel: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assertions" => {
                            if v.is_null() {
                                continue;
                            }
                            assertions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "call_type" => {
                            if v.is_null() {
                                continue;
                            }
                            call_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cert" => {
                            if v.is_null() {
                                continue;
                            }
                            cert = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure" => {
                            if v.is_null() {
                                continue;
                            }
                            failure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fast_retry" => {
                            if v.is_null() {
                                continue;
                            }
                            is_fast_retry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request" => {
                            if v.is_null() {
                                continue;
                            }
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolved_ip" => {
                            if v.is_null() {
                                continue;
                            }
                            resolved_ip =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response" => {
                            if v.is_null() {
                                continue;
                            }
                            response = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_type" => {
                            if v.is_null() {
                                continue;
                            }
                            run_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "traceroute" => {
                            if v.is_null() {
                                continue;
                            }
                            traceroute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triggered_at" => {
                            if v.is_null() {
                                continue;
                            }
                            triggered_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tunnel" => {
                            if v.is_null() {
                                continue;
                            }
                            tunnel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsFastTestResultDetail {
                    assertions,
                    call_type,
                    cert,
                    duration,
                    failure,
                    finished_at,
                    id,
                    is_fast_retry,
                    request,
                    resolved_ip,
                    response,
                    run_type,
                    started_at,
                    status,
                    steps,
                    timings,
                    traceroute,
                    triggered_at,
                    tunnel,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsFastTestResultDetailVisitor)
    }
}
