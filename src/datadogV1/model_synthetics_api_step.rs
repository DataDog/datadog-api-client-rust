// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPIStep {
    /// Determines whether or not to continue with test if this step fails.
    #[serde(rename = "allowFailure", skip_serializing_if = "Option::is_none")]
    pub allow_failure: bool,
    /// Array of assertions used for the test.
    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Vec<SyntheticsAssertion>,
    /// Array of values to parse and save as variables from the response.
    #[serde(rename = "extractedValues", skip_serializing_if = "Option::is_none")]
    pub extracted_values: Vec<SyntheticsParsingOptions>,
    /// Determines whether or not to consider the entire test as failed if this step fails.
Can be used only if `allowFailure` is `true`.
    #[serde(rename = "isCritical", skip_serializing_if = "Option::is_none")]
    pub is_critical: bool,
    /// The name of the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: SyntheticsTestRequest,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: SyntheticsTestOptionsRetry,
    /// The subtype of the Synthetic multistep API test step, currently only supporting `http`.
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: SyntheticsAPIStepSubtype,
}

