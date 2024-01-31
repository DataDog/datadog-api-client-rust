// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing an array of Synthetic tests configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsListTestsResponse {
    /// Array of Synthetic tests configuration.
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV1::model::SyntheticsTestDetails>>,
}

impl SyntheticsListTestsResponse {
    pub fn new() -> SyntheticsListTestsResponse {
        SyntheticsListTestsResponse { tests: None }
    }

    pub fn with_tests(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsTestDetails>,
    ) -> &mut Self {
        self.tests = Some(value);
        self
    }
}
impl Default for SyntheticsListTestsResponse {
    fn default() -> Self {
        Self::new()
    }
}
