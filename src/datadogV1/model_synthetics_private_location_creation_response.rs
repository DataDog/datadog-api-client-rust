// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationCreationResponse {
    /// Configuration skeleton for the private location. See installation instructions of the private location on how to use this configuration.
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: interface{},
    /// Object containing information about the private location to create.
    #[serde(rename = "private_location")]
    pub private_location: SyntheticsPrivateLocation,
    /// Public key for the result encryption.
    #[serde(rename = "result_encryption", skip_serializing_if = "Option::is_none")]
    pub result_encryption: SyntheticsPrivateLocationCreationResponseResultEncryption,
}

