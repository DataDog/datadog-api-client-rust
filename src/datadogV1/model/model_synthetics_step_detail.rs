// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a step for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsStepDetail {
    /// Array of errors collected for a browser test.
    #[serde(rename = "browserErrors")]
    pub browser_errors: Option<Vec<crate::datadogV1::model::SyntheticsBrowserError>>,
    /// Type of assertion to apply in an API test.
    #[serde(rename = "checkType")]
    pub check_type: Option<crate::datadogV1::model::SyntheticsCheckType>,
    /// Description of the test.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Error returned by the test.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Navigate between different tabs for your browser test.
    #[serde(rename = "playingTab")]
    pub playing_tab: Option<crate::datadogV1::model::SyntheticsPlayingTab>,
    /// Whether or not screenshots where collected by the test.
    #[serde(rename = "screenshotBucketKey")]
    pub screenshot_bucket_key: Option<bool>,
    /// Whether or not to skip this step.
    #[serde(rename = "skipped")]
    pub skipped: Option<bool>,
    /// Whether or not snapshots where collected by the test.
    #[serde(rename = "snapshotBucketKey")]
    pub snapshot_bucket_key: Option<bool>,
    /// The step ID.
    #[serde(rename = "stepId")]
    pub step_id: Option<i64>,
    /// If this step includes a sub-test.
    /// [Subtests documentation](<https://docs.datadoghq.com/synthetics/browser_tests/advanced_options/#subtests>).
    #[serde(rename = "subTestStepDetails")]
    pub sub_test_step_details: Option<Vec<crate::datadogV1::model::SyntheticsStepDetail>>,
    /// Time before starting the step.
    #[serde(rename = "timeToInteractive")]
    pub time_to_interactive: Option<f64>,
    /// Step type used in your Synthetic test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsStepType>,
    /// URL to perform the step against.
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Value for the step.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    /// Array of Core Web Vitals metrics for the step.
    #[serde(rename = "vitalsMetrics")]
    pub vitals_metrics: Option<Vec<crate::datadogV1::model::SyntheticsCoreWebVitals>>,
    /// Warning collected that didn't failed the step.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV1::model::SyntheticsStepDetailWarning>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsStepDetail {
    pub fn new() -> SyntheticsStepDetail {
        SyntheticsStepDetail {
            browser_errors: None,
            check_type: None,
            description: None,
            duration: None,
            error: None,
            playing_tab: None,
            screenshot_bucket_key: None,
            skipped: None,
            snapshot_bucket_key: None,
            step_id: None,
            sub_test_step_details: None,
            time_to_interactive: None,
            type_: None,
            url: None,
            value: None,
            vitals_metrics: None,
            warnings: None,
            _unparsed: false,
        }
    }

    pub fn browser_errors(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsBrowserError>,
    ) -> &mut Self {
        self.browser_errors = Some(value);
        self
    }

    pub fn check_type(&mut self, value: crate::datadogV1::model::SyntheticsCheckType) -> &mut Self {
        self.check_type = Some(value);
        self
    }

    pub fn description(&mut self, value: String) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn duration(&mut self, value: f64) -> &mut Self {
        self.duration = Some(value);
        self
    }

    pub fn error(&mut self, value: String) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn playing_tab(
        &mut self,
        value: crate::datadogV1::model::SyntheticsPlayingTab,
    ) -> &mut Self {
        self.playing_tab = Some(value);
        self
    }

    pub fn screenshot_bucket_key(&mut self, value: bool) -> &mut Self {
        self.screenshot_bucket_key = Some(value);
        self
    }

    pub fn skipped(&mut self, value: bool) -> &mut Self {
        self.skipped = Some(value);
        self
    }

    pub fn snapshot_bucket_key(&mut self, value: bool) -> &mut Self {
        self.snapshot_bucket_key = Some(value);
        self
    }

    pub fn step_id(&mut self, value: i64) -> &mut Self {
        self.step_id = Some(value);
        self
    }

    pub fn sub_test_step_details(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsStepDetail>,
    ) -> &mut Self {
        self.sub_test_step_details = Some(value);
        self
    }

    pub fn time_to_interactive(&mut self, value: f64) -> &mut Self {
        self.time_to_interactive = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV1::model::SyntheticsStepType) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn url(&mut self, value: String) -> &mut Self {
        self.url = Some(value);
        self
    }

    pub fn value(&mut self, value: serde_json::Value) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn vitals_metrics(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsCoreWebVitals>,
    ) -> &mut Self {
        self.vitals_metrics = Some(value);
        self
    }

    pub fn warnings(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsStepDetailWarning>,
    ) -> &mut Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for SyntheticsStepDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsStepDetail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsStepDetailVisitor;
        impl<'a> Visitor<'a> for SyntheticsStepDetailVisitor {
            type Value = SyntheticsStepDetail;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut browser_errors: Option<
                    Vec<crate::datadogV1::model::SyntheticsBrowserError>,
                > = None;
                let mut check_type: Option<crate::datadogV1::model::SyntheticsCheckType> = None;
                let mut description: Option<String> = None;
                let mut duration: Option<f64> = None;
                let mut error: Option<String> = None;
                let mut playing_tab: Option<crate::datadogV1::model::SyntheticsPlayingTab> = None;
                let mut screenshot_bucket_key: Option<bool> = None;
                let mut skipped: Option<bool> = None;
                let mut snapshot_bucket_key: Option<bool> = None;
                let mut step_id: Option<i64> = None;
                let mut sub_test_step_details: Option<
                    Vec<crate::datadogV1::model::SyntheticsStepDetail>,
                > = None;
                let mut time_to_interactive: Option<f64> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsStepType> = None;
                let mut url: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut vitals_metrics: Option<
                    Vec<crate::datadogV1::model::SyntheticsCoreWebVitals>,
                > = None;
                let mut warnings: Option<
                    Vec<crate::datadogV1::model::SyntheticsStepDetailWarning>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "browserErrors" => {
                            if v.is_null() {
                                continue;
                            }
                            browser_errors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "checkType" => {
                            if v.is_null() {
                                continue;
                            }
                            check_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _check_type) = check_type {
                                match _check_type {
                                    crate::datadogV1::model::SyntheticsCheckType::UnparsedObject(_check_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "playingTab" => {
                            if v.is_null() {
                                continue;
                            }
                            playing_tab =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _playing_tab) = playing_tab {
                                match _playing_tab {
                                    crate::datadogV1::model::SyntheticsPlayingTab::UnparsedObject(_playing_tab) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "screenshotBucketKey" => {
                            if v.is_null() {
                                continue;
                            }
                            screenshot_bucket_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "skipped" => {
                            if v.is_null() {
                                continue;
                            }
                            skipped = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshotBucketKey" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot_bucket_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stepId" => {
                            if v.is_null() {
                                continue;
                            }
                            step_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subTestStepDetails" => {
                            if v.is_null() {
                                continue;
                            }
                            sub_test_step_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeToInteractive" => {
                            if v.is_null() {
                                continue;
                            }
                            time_to_interactive =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsStepType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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
                        "vitalsMetrics" => {
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
                        &_ => {}
                    }
                }

                let content = SyntheticsStepDetail {
                    browser_errors,
                    check_type,
                    description,
                    duration,
                    error,
                    playing_tab,
                    screenshot_bucket_key,
                    skipped,
                    snapshot_bucket_key,
                    step_id,
                    sub_test_step_details,
                    time_to_interactive,
                    type_,
                    url,
                    value,
                    vitals_metrics,
                    warnings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsStepDetailVisitor)
    }
}
