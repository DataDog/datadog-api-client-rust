// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Composed target for `validatesJSONPath` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAssertionJSONPathTargetTarget {
    /// The JSON path to assert.
    #[serde(rename = "jsonPath")]
    pub json_path: Option<String>,
    /// The specific operator to use on the path.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// The path target value to compare to.
    #[serde(rename = "targetValue")]
    pub target_value: Option<serde_json::Value>,
}

impl SyntheticsAssertionJSONPathTargetTarget {
    pub fn new() -> SyntheticsAssertionJSONPathTargetTarget {
        SyntheticsAssertionJSONPathTargetTarget {
            json_path: None,
            operator: None,
            target_value: None,
        }
    }

    pub fn json_path(&mut self, value: String) -> &mut Self {
        self.json_path = Some(value);
        self
    }

    pub fn operator(&mut self, value: String) -> &mut Self {
        self.operator = Some(value);
        self
    }

    pub fn target_value(&mut self, value: serde_json::Value) -> &mut Self {
        self.target_value = Some(value);
        self
    }
}

impl Default for SyntheticsAssertionJSONPathTargetTarget {
    fn default() -> Self {
        Self::new()
    }
}
