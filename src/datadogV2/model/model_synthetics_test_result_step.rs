// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A step result from a browser, mobile, or multistep API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultStep {
    /// Whether the test continues when this step fails.
    #[serde(rename = "allow_failure")]
    pub allow_failure: Option<bool>,
    /// Inner API test definition for browser `runApiTest` steps.
    #[serde(rename = "api_test")]
    pub api_test: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Assertion result for a browser or mobile step.
    #[serde(rename = "assertion_result")]
    pub assertion_result: Option<crate::datadogV2::model::SyntheticsTestResultStepAssertionResult>,
    /// Assertion results produced by the step.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>>,
    /// URLs of requests blocked during the step.
    #[serde(rename = "blocked_requests_urls")]
    pub blocked_requests_urls: Option<Vec<String>>,
    /// Bounding box of an element on the page.
    #[serde(rename = "bounds")]
    pub bounds: Option<crate::datadogV2::model::SyntheticsTestResultBounds>,
    /// Browser errors captured during the step.
    #[serde(rename = "browser_errors")]
    pub browser_errors: Option<Vec<crate::datadogV2::model::SyntheticsTestResultBrowserError>>,
    /// Storage bucket keys for artifacts produced during a step or test.
    #[serde(rename = "bucket_keys")]
    pub bucket_keys: Option<crate::datadogV2::model::SyntheticsTestResultBucketKeys>,
    /// CDN resources encountered during the step.
    #[serde(rename = "cdn_resources")]
    pub cdn_resources: Option<Vec<crate::datadogV2::model::SyntheticsTestResultCdnResource>>,
    /// Click type performed in a browser step.
    #[serde(rename = "click_type")]
    pub click_type: Option<String>,
    /// Compressed JSON descriptor for the step (internal format).
    #[serde(rename = "compressed_json_descriptor")]
    pub compressed_json_descriptor: Option<String>,
    /// Request configuration executed by this step (API test steps).
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Human-readable description of the step.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Duration of the step in milliseconds.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Description of the element interacted with by the step.
    #[serde(rename = "element_description")]
    pub element_description: Option<String>,
    /// Element locator updates produced during a step.
    #[serde(rename = "element_updates")]
    pub element_updates: Option<crate::datadogV2::model::SyntheticsTestResultStepElementUpdates>,
    /// A variable used or extracted during a test.
    #[serde(rename = "extracted_value")]
    pub extracted_value: Option<crate::datadogV2::model::SyntheticsTestResultVariable>,
    /// Details about the failure of a Synthetic test.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV2::model::SyntheticsTestResultFailure>,
    /// HTTP results produced by an MCP step.
    #[serde(rename = "http_results")]
    pub http_results: Option<Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>>,
    /// Identifier of the step.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Whether this step is critical for the test outcome.
    #[serde(rename = "is_critical")]
    pub is_critical: Option<bool>,
    /// Whether the step uses a custom JavaScript assertion.
    #[serde(rename = "javascript_custom_assertion_code")]
    pub javascript_custom_assertion_code: Option<bool>,
    /// Time taken to locate the element in milliseconds.
    #[serde(rename = "locate_element_duration")]
    pub locate_element_duration: Option<f64>,
    /// Name of the step.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Details of the outgoing request made during the test execution.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo>,
    /// Details of the response received during the test execution.
    #[serde(rename = "response")]
    pub response: Option<crate::datadogV2::model::SyntheticsTestResultResponseInfo>,
    /// Retry results for the step.
    #[serde(rename = "retries")]
    pub retries: Option<Vec<crate::datadogV2::model::SyntheticsTestResultStep>>,
    /// Number of times this step was retried.
    #[serde(rename = "retry_count")]
    pub retry_count: Option<i64>,
    /// RUM application context associated with a step or sub-test.
    #[serde(rename = "rum_context")]
    pub rum_context: Option<crate::datadogV2::model::SyntheticsTestResultRumContext>,
    /// Unix timestamp (ms) of when the step started.
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    /// Status of the step (for example, `passed`, `failed`).
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// Information about a sub-step in a nested test execution.
    #[serde(rename = "sub_step")]
    pub sub_step: Option<crate::datadogV2::model::SyntheticsTestResultSubStep>,
    /// Information about a sub-test played from a parent browser test.
    #[serde(rename = "sub_test")]
    pub sub_test: Option<crate::datadogV2::model::SyntheticsTestResultSubTest>,
    /// Subtype of the step.
    #[serde(rename = "subtype")]
    pub subtype: Option<String>,
    /// Browser tabs involved in the step.
    #[serde(rename = "tabs")]
    pub tabs: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTab>>,
    /// Timing breakdown of the step execution.
    #[serde(rename = "timings")]
    pub timings: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Whether the step was executed through a Synthetics tunnel.
    #[serde(rename = "tunnel")]
    pub tunnel: Option<bool>,
    /// Type of the step (for example, `click`, `assertElementContent`, `runApiTest`).
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL associated with the step (for navigation steps).
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Step value. Its type depends on the step type.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    /// Variables captured during a test step.
    #[serde(rename = "variables")]
    pub variables: Option<crate::datadogV2::model::SyntheticsTestResultVariables>,
    /// Web vitals metrics captured during the step.
    #[serde(rename = "vitals_metrics")]
    pub vitals_metrics: Option<Vec<crate::datadogV2::model::SyntheticsTestResultVitalsMetrics>>,
    /// Warnings emitted during the step.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::SyntheticsTestResultWarning>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultStep {
    pub fn new() -> SyntheticsTestResultStep {
        SyntheticsTestResultStep {
            allow_failure: None,
            api_test: None,
            assertion_result: None,
            assertions: None,
            blocked_requests_urls: None,
            bounds: None,
            browser_errors: None,
            bucket_keys: None,
            cdn_resources: None,
            click_type: None,
            compressed_json_descriptor: None,
            config: None,
            description: None,
            duration: None,
            element_description: None,
            element_updates: None,
            extracted_value: None,
            failure: None,
            http_results: None,
            id: None,
            is_critical: None,
            javascript_custom_assertion_code: None,
            locate_element_duration: None,
            name: None,
            request: None,
            response: None,
            retries: None,
            retry_count: None,
            rum_context: None,
            started_at: None,
            status: None,
            sub_step: None,
            sub_test: None,
            subtype: None,
            tabs: None,
            timings: None,
            tunnel: None,
            type_: None,
            url: None,
            value: None,
            variables: None,
            vitals_metrics: None,
            warnings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_failure(mut self, value: bool) -> Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn api_test(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.api_test = Some(value);
        self
    }

    pub fn assertion_result(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultStepAssertionResult,
    ) -> Self {
        self.assertion_result = Some(value);
        self
    }

    pub fn assertions(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
    ) -> Self {
        self.assertions = Some(value);
        self
    }

    pub fn blocked_requests_urls(mut self, value: Vec<String>) -> Self {
        self.blocked_requests_urls = Some(value);
        self
    }

    pub fn bounds(mut self, value: crate::datadogV2::model::SyntheticsTestResultBounds) -> Self {
        self.bounds = Some(value);
        self
    }

    pub fn browser_errors(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultBrowserError>,
    ) -> Self {
        self.browser_errors = Some(value);
        self
    }

    pub fn bucket_keys(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultBucketKeys,
    ) -> Self {
        self.bucket_keys = Some(value);
        self
    }

    pub fn cdn_resources(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultCdnResource>,
    ) -> Self {
        self.cdn_resources = Some(value);
        self
    }

    pub fn click_type(mut self, value: String) -> Self {
        self.click_type = Some(value);
        self
    }

    pub fn compressed_json_descriptor(mut self, value: String) -> Self {
        self.compressed_json_descriptor = Some(value);
        self
    }

    pub fn config(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn element_description(mut self, value: String) -> Self {
        self.element_description = Some(value);
        self
    }

    pub fn element_updates(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultStepElementUpdates,
    ) -> Self {
        self.element_updates = Some(value);
        self
    }

    pub fn extracted_value(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultVariable,
    ) -> Self {
        self.extracted_value = Some(value);
        self
    }

    pub fn failure(mut self, value: crate::datadogV2::model::SyntheticsTestResultFailure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn http_results(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
    ) -> Self {
        self.http_results = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_critical(mut self, value: bool) -> Self {
        self.is_critical = Some(value);
        self
    }

    pub fn javascript_custom_assertion_code(mut self, value: bool) -> Self {
        self.javascript_custom_assertion_code = Some(value);
        self
    }

    pub fn locate_element_duration(mut self, value: f64) -> Self {
        self.locate_element_duration = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn request(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultRequestInfo,
    ) -> Self {
        self.request = Some(value);
        self
    }

    pub fn response(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultResponseInfo,
    ) -> Self {
        self.response = Some(value);
        self
    }

    pub fn retries(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultStep>,
    ) -> Self {
        self.retries = Some(value);
        self
    }

    pub fn retry_count(mut self, value: i64) -> Self {
        self.retry_count = Some(value);
        self
    }

    pub fn rum_context(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultRumContext,
    ) -> Self {
        self.rum_context = Some(value);
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

    pub fn sub_step(mut self, value: crate::datadogV2::model::SyntheticsTestResultSubStep) -> Self {
        self.sub_step = Some(value);
        self
    }

    pub fn sub_test(mut self, value: crate::datadogV2::model::SyntheticsTestResultSubTest) -> Self {
        self.sub_test = Some(value);
        self
    }

    pub fn subtype(mut self, value: String) -> Self {
        self.subtype = Some(value);
        self
    }

    pub fn tabs(mut self, value: Vec<crate::datadogV2::model::SyntheticsTestResultTab>) -> Self {
        self.tabs = Some(value);
        self
    }

    pub fn timings(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.timings = Some(value);
        self
    }

    pub fn tunnel(mut self, value: bool) -> Self {
        self.tunnel = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn variables(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultVariables,
    ) -> Self {
        self.variables = Some(value);
        self
    }

    pub fn vitals_metrics(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultVitalsMetrics>,
    ) -> Self {
        self.vitals_metrics = Some(value);
        self
    }

    pub fn warnings(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultWarning>,
    ) -> Self {
        self.warnings = Some(value);
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

impl Default for SyntheticsTestResultStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultStepVisitor {
            type Value = SyntheticsTestResultStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_failure: Option<bool> = None;
                let mut api_test: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut assertion_result: Option<
                    crate::datadogV2::model::SyntheticsTestResultStepAssertionResult,
                > = None;
                let mut assertions: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
                > = None;
                let mut blocked_requests_urls: Option<Vec<String>> = None;
                let mut bounds: Option<crate::datadogV2::model::SyntheticsTestResultBounds> = None;
                let mut browser_errors: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultBrowserError>,
                > = None;
                let mut bucket_keys: Option<
                    crate::datadogV2::model::SyntheticsTestResultBucketKeys,
                > = None;
                let mut cdn_resources: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultCdnResource>,
                > = None;
                let mut click_type: Option<String> = None;
                let mut compressed_json_descriptor: Option<String> = None;
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut description: Option<String> = None;
                let mut duration: Option<f64> = None;
                let mut element_description: Option<String> = None;
                let mut element_updates: Option<
                    crate::datadogV2::model::SyntheticsTestResultStepElementUpdates,
                > = None;
                let mut extracted_value: Option<
                    crate::datadogV2::model::SyntheticsTestResultVariable,
                > = None;
                let mut failure: Option<crate::datadogV2::model::SyntheticsTestResultFailure> =
                    None;
                let mut http_results: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultAssertionResult>,
                > = None;
                let mut id: Option<String> = None;
                let mut is_critical: Option<bool> = None;
                let mut javascript_custom_assertion_code: Option<bool> = None;
                let mut locate_element_duration: Option<f64> = None;
                let mut name: Option<String> = None;
                let mut request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo> =
                    None;
                let mut response: Option<
                    crate::datadogV2::model::SyntheticsTestResultResponseInfo,
                > = None;
                let mut retries: Option<Vec<crate::datadogV2::model::SyntheticsTestResultStep>> =
                    None;
                let mut retry_count: Option<i64> = None;
                let mut rum_context: Option<
                    crate::datadogV2::model::SyntheticsTestResultRumContext,
                > = None;
                let mut started_at: Option<i64> = None;
                let mut status: Option<String> = None;
                let mut sub_step: Option<crate::datadogV2::model::SyntheticsTestResultSubStep> =
                    None;
                let mut sub_test: Option<crate::datadogV2::model::SyntheticsTestResultSubTest> =
                    None;
                let mut subtype: Option<String> = None;
                let mut tabs: Option<Vec<crate::datadogV2::model::SyntheticsTestResultTab>> = None;
                let mut timings: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut tunnel: Option<bool> = None;
                let mut type_: Option<String> = None;
                let mut url: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut variables: Option<crate::datadogV2::model::SyntheticsTestResultVariables> =
                    None;
                let mut vitals_metrics: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultVitalsMetrics>,
                > = None;
                let mut warnings: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultWarning>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_failure" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "api_test" => {
                            if v.is_null() {
                                continue;
                            }
                            api_test = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assertion_result" => {
                            if v.is_null() {
                                continue;
                            }
                            assertion_result =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assertions" => {
                            if v.is_null() {
                                continue;
                            }
                            assertions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "blocked_requests_urls" => {
                            if v.is_null() {
                                continue;
                            }
                            blocked_requests_urls =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bounds" => {
                            if v.is_null() {
                                continue;
                            }
                            bounds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browser_errors" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_errors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cdn_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            cdn_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "click_type" => {
                            if v.is_null() {
                                continue;
                            }
                            click_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "compressed_json_descriptor" => {
                            if v.is_null() {
                                continue;
                            }
                            compressed_json_descriptor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "element_description" => {
                            if v.is_null() {
                                continue;
                            }
                            element_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "element_updates" => {
                            if v.is_null() {
                                continue;
                            }
                            element_updates =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extracted_value" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure" => {
                            if v.is_null() {
                                continue;
                            }
                            failure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "http_results" => {
                            if v.is_null() {
                                continue;
                            }
                            http_results =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_critical" => {
                            if v.is_null() {
                                continue;
                            }
                            is_critical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "javascript_custom_assertion_code" => {
                            if v.is_null() {
                                continue;
                            }
                            javascript_custom_assertion_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locate_element_duration" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            locate_element_duration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request" => {
                            if v.is_null() {
                                continue;
                            }
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response" => {
                            if v.is_null() {
                                continue;
                            }
                            response = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retries" => {
                            if v.is_null() {
                                continue;
                            }
                            retries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retry_count" => {
                            if v.is_null() {
                                continue;
                            }
                            retry_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_context" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_context =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "sub_step" => {
                            if v.is_null() {
                                continue;
                            }
                            sub_step = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_test" => {
                            if v.is_null() {
                                continue;
                            }
                            sub_test = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subtype" => {
                            if v.is_null() {
                                continue;
                            }
                            subtype = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tabs" => {
                            if v.is_null() {
                                continue;
                            }
                            tabs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timings" => {
                            if v.is_null() {
                                continue;
                            }
                            timings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tunnel" => {
                            if v.is_null() {
                                continue;
                            }
                            tunnel = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vitals_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            vitals_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultStep {
                    allow_failure,
                    api_test,
                    assertion_result,
                    assertions,
                    blocked_requests_urls,
                    bounds,
                    browser_errors,
                    bucket_keys,
                    cdn_resources,
                    click_type,
                    compressed_json_descriptor,
                    config,
                    description,
                    duration,
                    element_description,
                    element_updates,
                    extracted_value,
                    failure,
                    http_results,
                    id,
                    is_critical,
                    javascript_custom_assertion_code,
                    locate_element_duration,
                    name,
                    request,
                    response,
                    retries,
                    retry_count,
                    rum_context,
                    started_at,
                    status,
                    sub_step,
                    sub_test,
                    subtype,
                    tabs,
                    timings,
                    tunnel,
                    type_,
                    url,
                    value,
                    variables,
                    vitals_metrics,
                    warnings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultStepVisitor)
    }
}
