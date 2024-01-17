// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object that contains the new private location, the public key for result encryption, and the configuration skeleton.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationCreationResponse {
    /// Configuration skeleton for the private location. See installation instructions of the private location on how to use this configuration.
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Object containing information about the private location to create.
    #[serde(rename = "private_location")]
    pub private_location: Option<Box<crate::datadogV1::model::SyntheticsPrivateLocation>>,
    /// Public key for the result encryption.
    #[serde(rename = "result_encryption")]
    pub result_encryption: Option<
        Box<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption>,
    >,
}

impl SyntheticsPrivateLocationCreationResponse {
    pub fn new() -> SyntheticsPrivateLocationCreationResponse {
        SyntheticsPrivateLocationCreationResponse {
            config: None,
            private_location: None,
            result_encryption: None,
        }
    }
}
impl Default for SyntheticsPrivateLocationCreationResponse {
    fn default() -> Self {
        Self::new()
    }
}
