// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Composed target for `validatesXPath` operator.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAssertionXPathTargetTarget {
    /// The specific operator to use on the path.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// The path target value to compare to.
    #[serde(rename = "targetValue")]
    pub target_value: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The X path to assert.
    #[serde(rename = "xPath")]
    pub x_path: Option<String>,
}

impl SyntheticsAssertionXPathTargetTarget {
    pub fn new() -> SyntheticsAssertionXPathTargetTarget {
        SyntheticsAssertionXPathTargetTarget {
            operator: None,
            target_value: None,
            x_path: None,
        }
    }
}
