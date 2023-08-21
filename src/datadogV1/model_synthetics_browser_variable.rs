// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserVariable {
    /// Example for the variable.
    #[serde(rename = "example", skip_serializing_if = "Option::is_none")]
    pub example: String,
    /// ID for the variable. Global variables require an ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Name of the variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Pattern of the variable.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: String,
    /// Determines whether or not the browser test variable is obfuscated. Can only be used with browser variables of type `text`.
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: bool,
    /// Type of browser test variable.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsBrowserVariableType,
}

