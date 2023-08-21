// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestConfig {
    /// Array of assertions used for the test. Required for single API tests.
    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Vec<SyntheticsAssertion>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables", skip_serializing_if = "Option::is_none")]
    pub config_variables: Vec<SyntheticsConfigVariable>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: SyntheticsTestRequest,
    /// When the test subtype is `multi`, the steps of the test.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Vec<SyntheticsAPIStep>,
}

