// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The steps used in a Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsStep {
    /// A boolean set to allow this step to fail.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// A boolean set to always execute this step even if the previous step failed or was skipped.
    #[serde(rename = "alwaysExecute")]
    pub always_execute: Option<bool>,
    /// A boolean set to exit the test if the step succeeds.
    #[serde(rename = "exitIfSucceed")]
    pub exit_if_succeed: Option<bool>,
    /// A boolean to use in addition to `allowFailure` to determine if the test should be marked as failed when the step fails.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A boolean set to skip taking a screenshot for the step.
    #[serde(rename = "noScreenshot")]
    pub no_screenshot: Option<bool>,
    /// The parameters of the step.
    #[serde(rename = "params")]
    pub params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The time before declaring a step failed.
    #[serde(rename = "timeout")]
    pub timeout: Option<i64>,
    /// Step type used in your Synthetic test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsStepType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsStep {
    pub fn new() -> SyntheticsStep {
        SyntheticsStep {
            allow_failure: None,
            always_execute: None,
            exit_if_succeed: None,
            is_critical: None,
            name: None,
            no_screenshot: None,
            params: None,
            timeout: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_failure(mut self, value: bool) -> Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn always_execute(mut self, value: bool) -> Self {
        self.always_execute = Some(value);
        self
    }

    pub fn exit_if_succeed(mut self, value: bool) -> Self {
        self.exit_if_succeed = Some(value);
        self
    }

    pub fn is_critical(mut self, value: bool) -> Self {
        self.is_critical = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn no_screenshot(mut self, value: bool) -> Self {
        self.no_screenshot = Some(value);
        self
    }

    pub fn params(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn timeout(mut self, value: i64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::SyntheticsStepType) -> Self {
        self.type_ = Some(value);
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

impl Default for SyntheticsStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsStepVisitor;
        impl<'a> Visitor<'a> for SyntheticsStepVisitor {
            type Value = SyntheticsStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_failure: Option<bool> = None;
                let mut always_execute: Option<bool> = None;
                let mut exit_if_succeed: Option<bool> = None;
                let mut is_critical: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut no_screenshot: Option<bool> = None;
                let mut params: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut timeout: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsStepType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowFailure" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_failure =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "alwaysExecute" => {
                            if v.is_null() {
                                continue;
                            }
                            always_execute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exitIfSucceed" => {
                            if v.is_null() {
                                continue;
                            }
                            exit_if_succeed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isCritical" => {
                            if v.is_null() {
                                continue;
                            }
                            is_critical =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "noScreenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            no_screenshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "params" => {
                            if v.is_null() {
                                continue;
                            }
                            params = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsStep {
                    allow_failure,
                    always_execute,
                    exit_if_succeed,
                    is_critical,
                    name,
                    no_screenshot,
                    params,
                    timeout,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsStepVisitor)
    }
}
