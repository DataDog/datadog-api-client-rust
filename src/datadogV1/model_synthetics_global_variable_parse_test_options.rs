// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableParseTestOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: String,
    /// When type is `local_variable`, name of the local variable to use to extract the value.
    #[serde(rename = "localVariableName", skip_serializing_if = "Option::is_none")]
    pub local_variable_name: String,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: SyntheticsVariableParser,
    /// Property of the Synthetic Test Response to use for a Synthetic global variable.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsGlobalVariableParseTestOptionsType,
}

