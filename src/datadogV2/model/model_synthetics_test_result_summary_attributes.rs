// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Synthetic test result summary.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultSummaryAttributes {
    /// Device information for the test result (browser and mobile tests).
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV2::model::SyntheticsTestResultDevice>,
    /// Execution details for a Synthetic test result.
    #[serde(rename = "execution_info")]
    pub execution_info: Option<crate::datadogV2::model::SyntheticsTestResultExecutionInfo>,
    /// Timestamp of when the test finished (in milliseconds).
    #[serde(rename = "finished_at")]
    pub finished_at: Option<i64>,
    /// Location information for a Synthetic test result.
    #[serde(rename = "location")]
    pub location: Option<crate::datadogV2::model::SyntheticsTestResultLocation>,
    /// The type of run for a Synthetic test result.
    #[serde(rename = "run_type")]
    pub run_type: Option<crate::datadogV2::model::SyntheticsTestResultRunType>,
    /// Timestamp of when the test started (in milliseconds).
    #[serde(rename = "started_at")]
    pub started_at: Option<i64>,
    /// Status of a Synthetic test result.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SyntheticsTestResultStatus>,
    /// Step execution summary for a Synthetic test result.
    #[serde(rename = "steps_info")]
    pub steps_info: Option<crate::datadogV2::model::SyntheticsTestResultStepsInfo>,
    /// The subtype of the test (for example, `http`, `multi`, `ssl`).
    #[serde(rename = "test_sub_type")]
    pub test_sub_type: Option<String>,
    /// The type of the test (for example, `api` or `browser`).
    #[serde(rename = "test_type")]
    pub test_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultSummaryAttributes {
    pub fn new() -> SyntheticsTestResultSummaryAttributes {
        SyntheticsTestResultSummaryAttributes {
            device: None,
            execution_info: None,
            finished_at: None,
            location: None,
            run_type: None,
            started_at: None,
            status: None,
            steps_info: None,
            test_sub_type: None,
            test_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn device(mut self, value: crate::datadogV2::model::SyntheticsTestResultDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn execution_info(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultExecutionInfo,
    ) -> Self {
        self.execution_info = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn run_type(mut self, value: crate::datadogV2::model::SyntheticsTestResultRunType) -> Self {
        self.run_type = Some(value);
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

    pub fn steps_info(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultStepsInfo,
    ) -> Self {
        self.steps_info = Some(value);
        self
    }

    pub fn test_sub_type(mut self, value: String) -> Self {
        self.test_sub_type = Some(value);
        self
    }

    pub fn test_type(mut self, value: String) -> Self {
        self.test_type = Some(value);
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

impl Default for SyntheticsTestResultSummaryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultSummaryAttributesVisitor {
            type Value = SyntheticsTestResultSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut device: Option<crate::datadogV2::model::SyntheticsTestResultDevice> = None;
                let mut execution_info: Option<
                    crate::datadogV2::model::SyntheticsTestResultExecutionInfo,
                > = None;
                let mut finished_at: Option<i64> = None;
                let mut location: Option<crate::datadogV2::model::SyntheticsTestResultLocation> =
                    None;
                let mut run_type: Option<crate::datadogV2::model::SyntheticsTestResultRunType> =
                    None;
                let mut started_at: Option<i64> = None;
                let mut status: Option<crate::datadogV2::model::SyntheticsTestResultStatus> = None;
                let mut steps_info: Option<crate::datadogV2::model::SyntheticsTestResultStepsInfo> =
                    None;
                let mut test_sub_type: Option<String> = None;
                let mut test_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_info" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_info =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finished_at" => {
                            if v.is_null() {
                                continue;
                            }
                            finished_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "steps_info" => {
                            if v.is_null() {
                                continue;
                            }
                            steps_info = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_sub_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_sub_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultSummaryAttributes {
                    device,
                    execution_info,
                    finished_at,
                    location,
                    run_type,
                    started_at,
                    status,
                    steps_info,
                    test_sub_type,
                    test_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultSummaryAttributesVisitor)
    }
}
