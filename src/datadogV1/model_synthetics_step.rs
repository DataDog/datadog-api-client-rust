// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsStep {
    /// A boolean set to allow this step to fail.
    #[serde(rename = "allowFailure", skip_serializing_if = "Option::is_none")]
    pub allow_failure: bool,
    /// A boolean to use in addition to `allowFailure` to determine if the test should be marked as failed when the step fails.
    #[serde(rename = "isCritical", skip_serializing_if = "Option::is_none")]
    pub is_critical: bool,
    /// The name of the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// A boolean set to not take a screenshot for the step.
    #[serde(rename = "noScreenshot", skip_serializing_if = "Option::is_none")]
    pub no_screenshot: bool,
    /// The parameters of the step.
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: interface{},
    /// The time before declaring a step failed.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: i64,
    /// Step type used in your Synthetic test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsStepType,
}

