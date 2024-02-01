// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The steps used in a Synthetic browser test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStep {
    /// A boolean set to allow this step to fail.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// A boolean to use in addition to `allowFailure` to determine if the test should be marked as failed when the step fails.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A boolean set to not take a screenshot for the step.
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
}

impl SyntheticsStep {
    pub fn new() -> SyntheticsStep {
        SyntheticsStep {
            allow_failure: None,
            is_critical: None,
            name: None,
            no_screenshot: None,
            params: None,
            timeout: None,
            type_: None,
        }
    }

    pub fn allow_failure(&mut self, value: bool) -> &mut Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn is_critical(&mut self, value: bool) -> &mut Self {
        self.is_critical = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn no_screenshot(&mut self, value: bool) -> &mut Self {
        self.no_screenshot = Some(value);
        self
    }

    pub fn params(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.params = Some(value);
        self
    }

    pub fn timeout(&mut self, value: i64) -> &mut Self {
        self.timeout = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV1::model::SyntheticsStepType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SyntheticsStep {
    fn default() -> Self {
        Self::new()
    }
}
