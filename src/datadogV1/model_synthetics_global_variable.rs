// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariable {
    /// Attributes of the global variable.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: SyntheticsGlobalVariableAttributes,
    /// Description of the global variable.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: String,
    /// Unique identifier of the global variable.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Name of the global variable. Unique across Synthetic global variables.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Parser options to use for retrieving a Synthetic global variable from a Synthetic test. Used in conjunction with `parse_test_public_id`.
    #[serde(rename = "parse_test_options")]
    pub parse_test_options: SyntheticsGlobalVariableParseTestOptions,
    /// A Synthetic test ID to use as a test to generate the variable value.
    #[serde(rename = "parse_test_public_id", skip_serializing_if = "Option::is_none")]
    pub parse_test_public_id: String,
    /// Tags of the global variable.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// Value of the global variable.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: SyntheticsGlobalVariableValue,
}

