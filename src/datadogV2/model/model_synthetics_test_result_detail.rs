// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Full result details for a Synthetic test execution.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultDetail {
    /// Assertion results produced by the test.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>>,
    /// Storage bucket keys for artifacts produced during a step or test.
    #[serde(rename = "bucket_keys")]
    pub bucket_keys: Option<crate::datadogV2::model::SyntheticsTestResultBucketKeys>,
    /// gRPC call type (for example, `unary`, `healthCheck`, or `reflection`).
    #[serde(rename = "call_type")]
    pub call_type: Option<String>,
    /// SSL/TLS certificate information returned from an SSL test.
    #[serde(rename = "cert")]
    pub cert: Option<crate::datadogV2::model::SyntheticsTestResultCertificate>,
    /// Compressed JSON descriptor for the test (internal format).
    #[serde(rename = "compressed_json_descriptor")]
    pub compressed_json_descriptor: Option<String>,
    /// Compressed representation of the test steps (internal format).
    #[serde(rename = "compressed_steps")]
    pub compressed_steps: Option<String>,
    /// Outcome of the connection attempt (for example, `established`, `refused`).
    #[serde(rename = "connection_outcome")]
    pub connection_outcome: Option<String>,
    /// DNS resolution details recorded during the test execution.
    #[serde(rename = "dns_resolution")]
    pub dns_resolution: Option<crate::datadogV2::model::SyntheticsTestResultDnsResolution>,
    /// Duration of the test execution (in milliseconds).
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Whether the test exited early because a step marked with `exitIfSucceed` passed.
    #[serde(rename = "exited_on_step_success")]
    pub exited_on_step_success: Option<bool>,
    /// Details about the failure of a Synthetic test.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV2::model::SyntheticsTestResultFailure>,
    /// Timestamp of when the test finished (in milliseconds).
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Handshake request and response for protocol-level tests.
    #[serde(rename = "handshake")]
    pub handshake: Option<crate::datadogV2::model::SyntheticsTestResultHandshake>,
    /// The unique identifier for this result.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The initial result ID before any retries.
    #[serde(rename = "initial_id")]
    pub initial_id: Option<String>,
    /// Whether this result is from a fast retry.
    #[serde(rename = "is_fast_retry")]
    pub is_fast_retry: Option<bool>,
    /// Whether this result is from the last retry.
    #[serde(rename = "is_last_retry")]
    pub is_last_retry: Option<bool>,
    /// Network Path test result capturing the path between source and destination.
    #[serde(rename = "netpath")]
    pub netpath: Option<crate::datadogV2::model::SyntheticsTestResultNetpath>,
    /// Aggregated network statistics from the test execution.
    #[serde(rename = "netstats")]
    pub netstats: Option<crate::datadogV2::model::SyntheticsTestResultNetstats>,
    /// OCSP response received while validating a certificate.
    #[serde(rename = "ocsp")]
    pub ocsp: Option<crate::datadogV2::model::SyntheticsTestResultOCSPResponse>,
    /// A network probe result, used for traceroute hops and ping summaries.
    #[serde(rename = "ping")]
    pub ping: Option<crate::datadogV2::model::SyntheticsTestResultTracerouteHop>,
    /// Number of emails received during the test (email tests).
    #[serde(rename = "received_email_count")]
    pub received_email_count: Option<i64>,
    /// Message received from the target (for WebSocket/TCP/UDP tests).
    #[serde(rename = "received_message")]
    pub received_message: Option<String>,
    /// Details of the outgoing request made during the test execution.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo>,
    /// IP address resolved for the target host.
    #[serde(rename = "resolved_ip")]
    pub resolved_ip: Option<String>,
    /// Details of the response received during the test execution.
    #[serde(rename = "response")]
    pub response: Option<crate::datadogV2::model::SyntheticsTestResultResponseInfo>,
    /// The type of run for a Synthetic test result.
    #[serde(rename = "run_type")]
    pub run_type: Option<crate::datadogV2::model::SyntheticsTestResultRunType>,
    /// Message sent to the target (for WebSocket/TCP/UDP tests).
    #[serde(rename = "sent_message")]
    pub sent_message: Option<String>,
    /// Start URL for the test (browser tests).
    #[serde(rename = "start_url")]
    pub start_url: Option<String>,
    /// Timestamp of when the test started (in milliseconds).
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    /// Status of a Synthetic test result.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SyntheticsTestResultStatus>,
    /// Step results (for browser, mobile, and multistep API tests).
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV2::model::SyntheticsTestResultStep>>,
    /// Time to interactive in milliseconds (browser tests).
    #[serde(rename = "time_to_interactive")]
    pub time_to_interactive: Option<i64>,
    /// Timing breakdown of the test request phases (for example, DNS, TCP, TLS, first byte).
    #[serde(rename = "timings")]
    pub timings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Trace identifiers associated with a Synthetic test result.
    #[serde(rename = "trace")]
    pub trace: Option<crate::datadogV2::model::SyntheticsTestResultTrace>,
    /// Traceroute hop results (for network tests).
    #[serde(rename = "traceroute")]
    pub traceroute: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTracerouteHop>>,
    /// Timestamp of when the test was triggered (in milliseconds).
    #[serde(rename = "triggered_at")]
    pub triggered_at: Option<i64>,
    /// Whether the test was executed through a tunnel.
    #[serde(rename = "tunnel")]
    pub tunnel: Option<bool>,
    /// Turns executed by a goal-based browser test.
    #[serde(rename = "turns")]
    pub turns: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTurn>>,
    /// Whether the test runner was unhealthy at the time of execution.
    #[serde(rename = "unhealthy")]
    pub unhealthy: Option<bool>,
    /// Variables captured during a test step.
    #[serde(rename = "variables")]
    pub variables: Option<crate::datadogV2::model::SyntheticsTestResultVariables>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultDetail {
    pub fn new() -> SyntheticsTestResultDetail {
        SyntheticsTestResultDetail {
            assertions: None,
            bucket_keys: None,
            call_type: None,
            cert: None,
            compressed_json_descriptor: None,
            compressed_steps: None,
            connection_outcome: None,
            dns_resolution: None,
            duration: None,
            exited_on_step_success: None,
            failure: None,
            finished_at: None,
            handshake: None,
            id: None,
            initial_id: None,
            is_fast_retry: None,
            is_last_retry: None,
            netpath: None,
            netstats: None,
            ocsp: None,
            ping: None,
            received_email_count: None,
            received_message: None,
            request: None,
            resolved_ip: None,
            response: None,
            run_type: None,
            sent_message: None,
            start_url: None,
            started_at: None,
            status: None,
            steps: None,
            time_to_interactive: None,
            timings: None,
            trace: None,
            traceroute: None,
            triggered_at: None,
            tunnel: None,
            turns: None,
            unhealthy: None,
            variables: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assertions(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
    ) -> Self {
        self.assertions = Some(value);
        self
    }

    pub fn bucket_keys(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultBucketKeys,
    ) -> Self {
        self.bucket_keys = Some(value);
        self
    }

    pub fn call_type(mut self, value: String) -> Self {
        self.call_type = Some(value);
        self
    }

    pub fn cert(mut self, value: crate::datadogV2::model::SyntheticsTestResultCertificate) -> Self {
        self.cert = Some(value);
        self
    }

    pub fn compressed_json_descriptor(mut self, value: String) -> Self {
        self.compressed_json_descriptor = Some(value);
        self
    }

    pub fn compressed_steps(mut self, value: String) -> Self {
        self.compressed_steps = Some(value);
        self
    }

    pub fn connection_outcome(mut self, value: String) -> Self {
        self.connection_outcome = Some(value);
        self
    }

    pub fn dns_resolution(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultDnsResolution,
    ) -> Self {
        self.dns_resolution = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn exited_on_step_success(mut self, value: bool) -> Self {
        self.exited_on_step_success = Some(value);
        self
    }

    pub fn failure(mut self, value: crate::datadogV2::model::SyntheticsTestResultFailure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn handshake(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultHandshake,
    ) -> Self {
        self.handshake = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn initial_id(mut self, value: String) -> Self {
        self.initial_id = Some(value);
        self
    }

    pub fn is_fast_retry(mut self, value: bool) -> Self {
        self.is_fast_retry = Some(value);
        self
    }

    pub fn is_last_retry(mut self, value: bool) -> Self {
        self.is_last_retry = Some(value);
        self
    }

    pub fn netpath(mut self, value: crate::datadogV2::model::SyntheticsTestResultNetpath) -> Self {
        self.netpath = Some(value);
        self
    }

    pub fn netstats(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultNetstats,
    ) -> Self {
        self.netstats = Some(value);
        self
    }

    pub fn ocsp(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultOCSPResponse,
    ) -> Self {
        self.ocsp = Some(value);
        self
    }

    pub fn ping(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultTracerouteHop,
    ) -> Self {
        self.ping = Some(value);
        self
    }

    pub fn received_email_count(mut self, value: i64) -> Self {
        self.received_email_count = Some(value);
        self
    }

    pub fn received_message(mut self, value: String) -> Self {
        self.received_message = Some(value);
        self
    }

    pub fn request(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultRequestInfo,
    ) -> Self {
        self.request = Some(value);
        self
    }

    pub fn resolved_ip(mut self, value: String) -> Self {
        self.resolved_ip = Some(value);
        self
    }

    pub fn response(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultResponseInfo,
    ) -> Self {
        self.response = Some(value);
        self
    }

    pub fn run_type(mut self, value: crate::datadogV2::model::SyntheticsTestResultRunType) -> Self {
        self.run_type = Some(value);
        self
    }

    pub fn sent_message(mut self, value: String) -> Self {
        self.sent_message = Some(value);
        self
    }

    pub fn start_url(mut self, value: String) -> Self {
        self.start_url = Some(value);
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::SyntheticsTestResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<crate::datadogV2::model::SyntheticsTestResultStep>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn time_to_interactive(mut self, value: i64) -> Self {
        self.time_to_interactive = Some(value);
        self
    }

    pub fn timings(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.timings = Some(value);
        self
    }

    pub fn trace(mut self, value: crate::datadogV2::model::SyntheticsTestResultTrace) -> Self {
        self.trace = Some(value);
        self
    }

    pub fn traceroute(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultTracerouteHop>,
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

    pub fn turns(mut self, value: Vec<crate::datadogV2::model::SyntheticsTestResultTurn>) -> Self {
        self.turns = Some(value);
        self
    }

    pub fn unhealthy(mut self, value: bool) -> Self {
        self.unhealthy = Some(value);
        self
    }

    pub fn variables(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultVariables,
    ) -> Self {
        self.variables = Some(value);
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

impl Default for SyntheticsTestResultDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultDetailVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultDetailVisitor {
            type Value = SyntheticsTestResultDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assertions: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
                > = None;
                let mut bucket_keys: Option<
                    crate::datadogV2::model::SyntheticsTestResultBucketKeys,
                > = None;
                let mut call_type: Option<String> = None;
                let mut cert: Option<crate::datadogV2::model::SyntheticsTestResultCertificate> =
                    None;
                let mut compressed_json_descriptor: Option<String> = None;
                let mut compressed_steps: Option<String> = None;
                let mut connection_outcome: Option<String> = None;
                let mut dns_resolution: Option<
                    crate::datadogV2::model::SyntheticsTestResultDnsResolution,
                > = None;
                let mut duration: Option<f64> = None;
                let mut exited_on_step_success: Option<bool> = None;
                let mut failure: Option<crate::datadogV2::model::SyntheticsTestResultFailure> =
                    None;
                let mut finished_at: Option<i64> = None;
                let mut handshake: Option<crate::datadogV2::model::SyntheticsTestResultHandshake> =
                    None;
                let mut id: Option<String> = None;
                let mut initial_id: Option<String> = None;
                let mut is_fast_retry: Option<bool> = None;
                let mut is_last_retry: Option<bool> = None;
                let mut netpath: Option<crate::datadogV2::model::SyntheticsTestResultNetpath> =
                    None;
                let mut netstats: Option<crate::datadogV2::model::SyntheticsTestResultNetstats> =
                    None;
                let mut ocsp: Option<crate::datadogV2::model::SyntheticsTestResultOCSPResponse> =
                    None;
                let mut ping: Option<crate::datadogV2::model::SyntheticsTestResultTracerouteHop> =
                    None;
                let mut received_email_count: Option<i64> = None;
                let mut received_message: Option<String> = None;
                let mut request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo> =
                    None;
                let mut resolved_ip: Option<String> = None;
                let mut response: Option<
                    crate::datadogV2::model::SyntheticsTestResultResponseInfo,
                > = None;
                let mut run_type: Option<crate::datadogV2::model::SyntheticsTestResultRunType> =
                    None;
                let mut sent_message: Option<String> = None;
                let mut start_url: Option<String> = None;
                let mut started_at: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::SyntheticsTestResultStatus> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::SyntheticsTestResultStep>> =
                    None;
                let mut time_to_interactive: Option<i64> = None;
                let mut timings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut trace: Option<crate::datadogV2::model::SyntheticsTestResultTrace> = None;
                let mut traceroute: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultTracerouteHop>,
                > = None;
                let mut triggered_at: Option<i64> = None;
                let mut tunnel: Option<bool> = None;
                let mut turns: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTurn>> =
                    None;
                let mut unhealthy: Option<bool> = None;
                let mut variables: Option<crate::datadogV2::model::SyntheticsTestResultVariables> =
                    None;
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
                        "bucket_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "compressed_json_descriptor" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_json_descriptor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compressed_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connection_outcome" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_outcome =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dns_resolution" => {
                            if v.is_null() {
                                continue;
                            }
                            dns_resolution =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exited_on_step_success" => {
                            if v.is_null() {
                                continue;
                            }
                            exited_on_step_success =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "handshake" => {
                            if v.is_null() {
                                continue;
                            }
                            handshake = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "initial_id" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fast_retry" => {
                            if v.is_null() {
                                continue;
                            }
                            is_fast_retry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_last_retry" => {
                            if v.is_null() {
                                continue;
                            }
                            is_last_retry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "netpath" => {
                            if v.is_null() {
                                continue;
                            }
                            netpath = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "netstats" => {
                            if v.is_null() {
                                continue;
                            }
                            netstats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ocsp" => {
                            if v.is_null() {
                                continue;
                            }
                            ocsp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ping" => {
                            if v.is_null() {
                                continue;
                            }
                            ping = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "received_email_count" => {
                            if v.is_null() {
                                continue;
                            }
                            received_email_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "received_message" => {
                            if v.is_null() {
                                continue;
                            }
                            received_message =
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
                            if let Some(ref _run_type) = run_type {
                                match _run_type {
                                    crate::datadogV2::model::SyntheticsTestResultRunType::UnparsedObject(_run_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sent_message" => {
                            if v.is_null() {
                                continue;
                            }
                            sent_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_url" => {
                            if v.is_null() {
                                continue;
                            }
                            start_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::SyntheticsTestResultStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_to_interactive" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_interactive =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace" => {
                            if v.is_null() {
                                continue;
                            }
                            trace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "turns" => {
                            if v.is_null() {
                                continue;
                            }
                            turns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unhealthy" => {
                            if v.is_null() {
                                continue;
                            }
                            unhealthy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultDetail {
                    assertions,
                    bucket_keys,
                    call_type,
                    cert,
                    compressed_json_descriptor,
                    compressed_steps,
                    connection_outcome,
                    dns_resolution,
                    duration,
                    exited_on_step_success,
                    failure,
                    finished_at,
                    handshake,
                    id,
                    initial_id,
                    is_fast_retry,
                    is_last_retry,
                    netpath,
                    netstats,
                    ocsp,
                    ping,
                    received_email_count,
                    received_message,
                    request,
                    resolved_ip,
                    response,
                    run_type,
                    sent_message,
                    start_url,
                    started_at,
                    status,
                    steps,
                    time_to_interactive,
                    timings,
                    trace,
                    traceroute,
                    triggered_at,
                    tunnel,
                    turns,
                    unhealthy,
                    variables,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultDetailVisitor)
    }
}
