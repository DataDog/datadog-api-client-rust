// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// CI/CD options for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestCiOptions {
    /// Execution rule for a Synthetic test.
    #[serde(rename = "executionRule")]
    pub execution_rule: Option<crate::datadogV1::model::SyntheticsTestExecutionRule>,
}

impl SyntheticsTestCiOptions {
    pub fn new() -> SyntheticsTestCiOptions {
        SyntheticsTestCiOptions {
            execution_rule: None,
        }
    }

    pub fn execution_rule(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestExecutionRule,
    ) -> Self {
        self.execution_rule = Some(value);
        self
    }
}

impl Default for SyntheticsTestCiOptions {
    fn default() -> Self {
        Self::new()
    }
}
