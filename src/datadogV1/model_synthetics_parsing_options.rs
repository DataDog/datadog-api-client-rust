// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsParsingOptions {
    /// When type is `http_header`, name of the header to use to extract the value.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: String,
    /// Name of the variable to extract.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Details of the parser to use for the global variable.
    #[serde(rename = "parser")]
    pub parser: SyntheticsVariableParser,
    /// Determines whether or not the extracted value will be obfuscated.
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: bool,
    /// Property of the Synthetic Test Response to use for a Synthetic global variable.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsGlobalVariableParseTestOptionsType,
}

