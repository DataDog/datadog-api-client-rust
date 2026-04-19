// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Storage bucket keys for artifacts produced during a step or test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultBucketKeys {
    /// Key for the screenshot captured after the step (goal-based tests).
    #[serde(rename = "after_step_screenshot")]
    pub after_step_screenshot: Option<String>,
    /// Key for the screenshot captured after the turn (goal-based tests).
    #[serde(rename = "after_turn_screenshot")]
    pub after_turn_screenshot: Option<String>,
    /// Key for miscellaneous artifacts.
    #[serde(rename = "artifacts")]
    pub artifacts: Option<String>,
    /// Key for the screenshot captured before the step (goal-based tests).
    #[serde(rename = "before_step_screenshot")]
    pub before_step_screenshot: Option<String>,
    /// Key for the screenshot captured before the turn (goal-based tests).
    #[serde(rename = "before_turn_screenshot")]
    pub before_turn_screenshot: Option<String>,
    /// Key for a captured crash report.
    #[serde(rename = "crash_report")]
    pub crash_report: Option<String>,
    /// Key for captured device logs.
    #[serde(rename = "device_logs")]
    pub device_logs: Option<String>,
    /// Keys for email message payloads captured by the step.
    #[serde(rename = "email_messages")]
    pub email_messages: Option<Vec<String>>,
    /// Key for the captured screenshot.
    #[serde(rename = "screenshot")]
    pub screenshot: Option<String>,
    /// Key for the captured DOM snapshot.
    #[serde(rename = "snapshot")]
    pub snapshot: Option<String>,
    /// Key for the page source or element source.
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultBucketKeys {
    pub fn new() -> SyntheticsTestResultBucketKeys {
        SyntheticsTestResultBucketKeys {
            after_step_screenshot: None,
            after_turn_screenshot: None,
            artifacts: None,
            before_step_screenshot: None,
            before_turn_screenshot: None,
            crash_report: None,
            device_logs: None,
            email_messages: None,
            screenshot: None,
            snapshot: None,
            source: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn after_step_screenshot(mut self, value: String) -> Self {
        self.after_step_screenshot = Some(value);
        self
    }

    pub fn after_turn_screenshot(mut self, value: String) -> Self {
        self.after_turn_screenshot = Some(value);
        self
    }

    pub fn artifacts(mut self, value: String) -> Self {
        self.artifacts = Some(value);
        self
    }

    pub fn before_step_screenshot(mut self, value: String) -> Self {
        self.before_step_screenshot = Some(value);
        self
    }

    pub fn before_turn_screenshot(mut self, value: String) -> Self {
        self.before_turn_screenshot = Some(value);
        self
    }

    pub fn crash_report(mut self, value: String) -> Self {
        self.crash_report = Some(value);
        self
    }

    pub fn device_logs(mut self, value: String) -> Self {
        self.device_logs = Some(value);
        self
    }

    pub fn email_messages(mut self, value: Vec<String>) -> Self {
        self.email_messages = Some(value);
        self
    }

    pub fn screenshot(mut self, value: String) -> Self {
        self.screenshot = Some(value);
        self
    }

    pub fn snapshot(mut self, value: String) -> Self {
        self.snapshot = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
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

impl Default for SyntheticsTestResultBucketKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultBucketKeys {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultBucketKeysVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultBucketKeysVisitor {
            type Value = SyntheticsTestResultBucketKeys;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut after_step_screenshot: Option<String> = None;
                let mut after_turn_screenshot: Option<String> = None;
                let mut artifacts: Option<String> = None;
                let mut before_step_screenshot: Option<String> = None;
                let mut before_turn_screenshot: Option<String> = None;
                let mut crash_report: Option<String> = None;
                let mut device_logs: Option<String> = None;
                let mut email_messages: Option<Vec<String>> = None;
                let mut screenshot: Option<String> = None;
                let mut snapshot: Option<String> = None;
                let mut source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "after_step_screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            after_step_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "after_turn_screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            after_turn_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "artifacts" => {
                            if v.is_null() {
                                continue;
                            }
                            artifacts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "before_step_screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            before_step_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "before_turn_screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            before_turn_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "crash_report" => {
                            if v.is_null() {
                                continue;
                            }
                            crash_report =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_logs" => {
                            if v.is_null() {
                                continue;
                            }
                            device_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email_messages" => {
                            if v.is_null() {
                                continue;
                            }
                            email_messages =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            screenshot = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultBucketKeys {
                    after_step_screenshot,
                    after_turn_screenshot,
                    artifacts,
                    before_step_screenshot,
                    before_turn_screenshot,
                    crash_report,
                    device_logs,
                    email_messages,
                    screenshot,
                    snapshot,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultBucketKeysVisitor)
    }
}
