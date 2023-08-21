// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsGlobalVariableValue {
    /// Options for the Global Variable for MFA.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: SyntheticsGlobalVariableOptions,
    /// Determines if the value of the variable is hidden.
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: bool,
    /// Value of the global variable. When reading a global variable,
the value will not be present if the variable is hidden with the `secure` property.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: String,
}

