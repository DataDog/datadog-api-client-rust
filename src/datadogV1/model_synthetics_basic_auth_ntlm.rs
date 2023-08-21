// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthNTLM {
    /// Domain for the authentication to use when performing the test.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: String,
    /// Password for the authentication to use when performing the test.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: String,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsBasicAuthNTLMType,
    /// Username for the authentication to use when performing the test.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: String,
    /// Workstation for the authentication to use when performing the test.
    #[serde(rename = "workstation", skip_serializing_if = "Option::is_none")]
    pub workstation: String,
}

