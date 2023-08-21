// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthSigv4 {
    /// Access key for the `SIGV4` authentication.
    #[serde(rename = "accessKey", skip_serializing_if = "Option::is_none")]
    pub access_key: String,
    /// Region for the `SIGV4` authentication.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: String,
    /// Secret key for the `SIGV4` authentication.
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: String,
    /// Service name for the `SIGV4` authentication.
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: String,
    /// Session token for the `SIGV4` authentication.
    #[serde(rename = "sessionToken", skip_serializing_if = "Option::is_none")]
    pub session_token: String,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SyntheticsBasicAuthSigv4Type,
}

