// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthWeb {
    /// Password to use for the basic authentication.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: String,
    /// The type of basic authentication to use when performing the test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsBasicAuthWebType,
    /// Username to use for the basic authentication.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: String,
}

