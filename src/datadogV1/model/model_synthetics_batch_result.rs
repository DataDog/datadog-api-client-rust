// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object with the results of a Synthetic batch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBatchResult {
    /// The device ID.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV1::model::SyntheticsDeviceID>,
    /// Total duration in millisecond of the test.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Execution rule for a Synthetic test.
    #[serde(rename = "execution_rule")]
    pub execution_rule: Option<crate::datadogV1::model::SyntheticsTestExecutionRule>,
    /// Name of the location.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The ID of the result to get.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// Number of times this result has been retried.
    #[serde(rename = "retries")]
    pub retries: Option<f64>,
    /// Determines whether the batch has passed, failed, or is in progress.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsBatchStatus>,
    /// Name of the test.
    #[serde(rename = "test_name")]
    pub test_name: Option<String>,
    /// The public ID of the Synthetic test.
    #[serde(rename = "test_public_id")]
    pub test_public_id: Option<String>,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "test_type")]
    pub test_type: Option<crate::datadogV1::model::SyntheticsTestDetailsType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBatchResult {
    pub fn new() -> SyntheticsBatchResult {
        SyntheticsBatchResult {
            device: None,
            duration: None,
            execution_rule: None,
            location: None,
            result_id: None,
            retries: None,
            status: None,
            test_name: None,
            test_public_id: None,
            test_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn device(mut self, value: crate::datadogV1::model::SyntheticsDeviceID) -> Self {
        self.device = Some(value);
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn execution_rule(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestExecutionRule,
    ) -> Self {
        self.execution_rule = Some(value);
        self
    }

    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }

    pub fn result_id(mut self, value: String) -> Self {
        self.result_id = Some(value);
        self
    }

    pub fn retries(mut self, value: f64) -> Self {
        self.retries = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::SyntheticsBatchStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn test_name(mut self, value: String) -> Self {
        self.test_name = Some(value);
        self
    }

    pub fn test_public_id(mut self, value: String) -> Self {
        self.test_public_id = Some(value);
        self
    }

    pub fn test_type(mut self, value: crate::datadogV1::model::SyntheticsTestDetailsType) -> Self {
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

impl Default for SyntheticsBatchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBatchResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBatchResultVisitor;
        impl<'a> Visitor<'a> for SyntheticsBatchResultVisitor {
            type Value = SyntheticsBatchResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut device: Option<crate::datadogV1::model::SyntheticsDeviceID> = None;
                let mut duration: Option<f64> = None;
                let mut execution_rule: Option<
                    crate::datadogV1::model::SyntheticsTestExecutionRule,
                > = None;
                let mut location: Option<String> = None;
                let mut result_id: Option<String> = None;
                let mut retries: Option<f64> = None;
                let mut status: Option<crate::datadogV1::model::SyntheticsBatchStatus> = None;
                let mut test_name: Option<String> = None;
                let mut test_public_id: Option<String> = None;
                let mut test_type: Option<crate::datadogV1::model::SyntheticsTestDetailsType> =
                    None;
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
                            if let Some(ref _device) = device {
                                match _device {
                                    crate::datadogV1::model::SyntheticsDeviceID::UnparsedObject(
                                        _device,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "execution_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            execution_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _execution_rule) = execution_rule {
                                match _execution_rule {
                                    crate::datadogV1::model::SyntheticsTestExecutionRule::UnparsedObject(_execution_rule) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_id" => {
                            if v.is_null() {
                                continue;
                            }
                            result_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retries" => {
                            if v.is_null() {
                                continue;
                            }
                            retries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::SyntheticsBatchStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "test_name" => {
                            if v.is_null() {
                                continue;
                            }
                            test_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            test_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _test_type) = test_type {
                                match _test_type {
                                    crate::datadogV1::model::SyntheticsTestDetailsType::UnparsedObject(_test_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsBatchResult {
                    device,
                    duration,
                    execution_rule,
                    location,
                    result_id,
                    retries,
                    status,
                    test_name,
                    test_public_id,
                    test_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBatchResultVisitor)
    }
}
