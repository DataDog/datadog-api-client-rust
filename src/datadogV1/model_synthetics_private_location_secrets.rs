// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationSecrets {
    /// Authentication part of the secrets.
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: SyntheticsPrivateLocationSecretsAuthentication,
    /// Private key for the private location.
    #[serde(rename = "config_decryption", skip_serializing_if = "Option::is_none")]
    pub config_decryption: SyntheticsPrivateLocationSecretsConfigDecryption,
}

