// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAssertionTarget {
    /// Assertion operator to apply.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: SyntheticsAssertionOperator,
    /// The associated assertion property.
    #[serde(rename = "property", skip_serializing_if = "Option::is_none")]
    pub property: String,
    /// Value used by the operator.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: interface{},
    /// Type of the assertion.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsAssertionType,
}

