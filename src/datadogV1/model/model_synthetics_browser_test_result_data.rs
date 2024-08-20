// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing results for your Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestResultData {
    /// Type of browser device used for the browser test.
    #[serde(rename = "browserType")]
    pub browser_type: Option<String>,
    /// Browser version used for the browser test.
    #[serde(rename = "browserVersion")]
    pub browser_version: Option<String>,
    /// Object describing the device used to perform the Synthetic test.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDevice>,
    /// Global duration in second of the browser test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Error returned for the browser test.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// The browser test failure details.
    #[serde(rename = "failure")]
    pub failure: Option<crate::datadogV1::model::SyntheticsBrowserTestResultFailure>,
    /// Whether or not the browser test was conducted.
    #[serde(rename = "passed")]
    pub passed: Option<bool>,
    /// The amount of email received during the browser test.
    #[serde(rename = "receivedEmailCount")]
    pub received_email_count: Option<i64>,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl")]
    pub start_url: Option<String>,
    /// Array containing the different browser test steps.
    #[serde(rename = "stepDetails")]
    pub step_details: Option<Vec<crate::datadogV1::model::SyntheticsStepDetail>>,
    /// Whether or not a thumbnail is associated with the browser test.
    #[serde(rename = "thumbnailsBucketKey")]
    pub thumbnails_bucket_key: Option<bool>,
    /// Time in second to wait before the browser test starts after
    /// reaching the start URL.
    #[serde(rename = "timeToInteractive")]
    pub time_to_interactive: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestResultData {
    pub fn new() -> SyntheticsBrowserTestResultData {
        SyntheticsBrowserTestResultData {
            browser_type: None,
            browser_version: None,
            device: None,
            duration: None,
            error: None,
            failure: None,
            passed: None,
            received_email_count: None,
            start_url: None,
            step_details: None,
            thumbnails_bucket_key: None,
            time_to_interactive: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn browser_type(mut self, value: String) -> Self {
        self.browser_type = Some(value);
        self
    }

    pub fn browser_version(mut self, value: String) -> Self {
        self.browser_version = Some(value);
        self
    }

    pub fn device(mut self, value: crate::datadogV1::model::SyntheticsDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn error(mut self, value: String) -> Self {
        self.error = Some(value);
        self
    }

    pub fn failure(
        mut self,
        value: crate::datadogV1::model::SyntheticsBrowserTestResultFailure,
    ) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn received_email_count(mut self, value: i64) -> Self {
        self.received_email_count = Some(value);
        self
    }

    pub fn start_url(mut self, value: String) -> Self {
        self.start_url = Some(value);
        self
    }

    pub fn step_details(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsStepDetail>,
    ) -> Self {
        self.step_details = Some(value);
        self
    }

    pub fn thumbnails_bucket_key(mut self, value: bool) -> Self {
        self.thumbnails_bucket_key = Some(value);
        self
    }

    pub fn time_to_interactive(mut self, value: f64) -> Self {
        self.time_to_interactive = Some(value);
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

impl Default for SyntheticsBrowserTestResultData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBrowserTestResultData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestResultDataVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestResultDataVisitor {
            type Value = SyntheticsBrowserTestResultData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut browser_type: Option<String> = None;
                let mut browser_version: Option<String> = None;
                let mut device: Option<crate::datadogV1::model::SyntheticsDevice> = None;
                let mut duration: Option<f64> = None;
                let mut error: Option<String> = None;
                let mut failure: Option<
                    crate::datadogV1::model::SyntheticsBrowserTestResultFailure,
                > = None;
                let mut passed: Option<bool> = None;
                let mut received_email_count: Option<i64> = None;
                let mut start_url: Option<String> = None;
                let mut step_details: Option<Vec<crate::datadogV1::model::SyntheticsStepDetail>> =
                    None;
                let mut thumbnails_bucket_key: Option<bool> = None;
                let mut time_to_interactive: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "browserType" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "browserVersion" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "failure" => {
                            if v.is_null() {
                                continue;
                            }
                            failure = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "passed" => {
                            if v.is_null() {
                                continue;
                            }
                            passed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "receivedEmailCount" => {
                            if v.is_null() {
                                continue;
                            }
                            received_email_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startUrl" => {
                            if v.is_null() {
                                continue;
                            }
                            start_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stepDetails" => {
                            if v.is_null() {
                                continue;
                            }
                            step_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thumbnailsBucketKey" => {
                            if v.is_null() {
                                continue;
                            }
                            thumbnails_bucket_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeToInteractive" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_interactive =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsBrowserTestResultData {
                    browser_type,
                    browser_version,
                    device,
                    duration,
                    error,
                    failure,
                    passed,
                    received_email_count,
                    start_url,
                    step_details,
                    thumbnails_bucket_key,
                    time_to_interactive,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestResultDataVisitor)
    }
}
