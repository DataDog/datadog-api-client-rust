// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration object for a Synthetic browser test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestConfig {
    /// Array of assertions used for the test.
    #[serde(rename = "assertions")]
    pub assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables")]
    pub config_variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: Box<crate::datadogV1::model::SyntheticsTestRequest>,
    /// Cookies to be used for the request, using the [Set-Cookie](<https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie>) syntax.
    #[serde(rename = "setCookie")]
    pub set_cookie: Option<String>,
    /// Array of variables used for the test steps.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>>,
}

impl SyntheticsBrowserTestConfig {
    pub fn new(
        assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
        request: Box<crate::datadogV1::model::SyntheticsTestRequest>,
    ) -> SyntheticsBrowserTestConfig {
        SyntheticsBrowserTestConfig {
            assertions,
            config_variables: None,
            request,
            set_cookie: None,
            variables: None,
        }
    }
}
