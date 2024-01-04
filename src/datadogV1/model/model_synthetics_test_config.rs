// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration object for a Synthetic test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestConfig {
    /// Array of assertions used for the test. Required for single API tests.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV1::model::SyntheticsAssertion>>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables")]
    pub config_variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: Option<Box<crate::datadogV1::model::SyntheticsTestRequest>>,
    /// Browser tests only - array of variables used for the test steps.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>>,
}

impl SyntheticsTestConfig {
    pub fn new() -> SyntheticsTestConfig {
        SyntheticsTestConfig {
            assertions: None,
            config_variables: None,
            request: None,
            variables: None,
        }
    }
}