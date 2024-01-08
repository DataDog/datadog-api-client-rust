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
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
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
}
impl Default for SyntheticsStep {
    fn default() -> Self {
        Self::new()
    }
}
