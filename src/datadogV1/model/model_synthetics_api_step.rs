// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The steps used in a Synthetic multistep API test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPIStep {
    /// Determines whether or not to continue with test if this step fails.
    #[serde(rename = "allowFailure")]
    pub allow_failure: Option<bool>,
    /// Array of assertions used for the test.
    #[serde(rename = "assertions")]
    pub assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
    /// Array of values to parse and save as variables from the response.
    #[serde(rename = "extractedValues")]
    pub extracted_values: Option<Vec<crate::datadogV1::model::SyntheticsParsingOptions>>,
    /// Determines whether or not to consider the entire test as failed if this step fails.
    /// Can be used only if `allowFailure` is `true`.
    #[serde(rename = "isCritical")]
    pub is_critical: Option<bool>,
    /// The name of the step.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: crate::datadogV1::model::SyntheticsTestRequest,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// The subtype of the Synthetic multistep API test step, currently only supporting `http`.
    #[serde(rename = "subtype")]
    pub subtype: crate::datadogV1::model::SyntheticsAPIStepSubtype,
}

impl SyntheticsAPIStep {
    pub fn new(
        assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
        name: String,
        request: crate::datadogV1::model::SyntheticsTestRequest,
        subtype: crate::datadogV1::model::SyntheticsAPIStepSubtype,
    ) -> SyntheticsAPIStep {
        SyntheticsAPIStep {
            allow_failure: None,
            assertions,
            extracted_values: None,
            is_critical: None,
            name,
            request,
            retry: None,
            subtype,
        }
    }

    pub fn with_allow_failure(&mut self, value: bool) -> &mut Self {
        self.allow_failure = Some(value);
        self
    }

    pub fn with_extracted_values(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsParsingOptions>,
    ) -> &mut Self {
        self.extracted_values = Some(value);
        self
    }

    pub fn with_is_critical(&mut self, value: bool) -> &mut Self {
        self.is_critical = Some(value);
        self
    }

    pub fn with_retry(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsRetry,
    ) -> &mut Self {
        self.retry = Some(value);
        self
    }
}
