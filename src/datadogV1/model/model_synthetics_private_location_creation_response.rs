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
    pub private_location: Option<crate::datadogV1::model::SyntheticsPrivateLocation>,
    /// Public key for the result encryption.
    #[serde(rename = "result_encryption")]
    pub result_encryption:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption>,
}

impl SyntheticsPrivateLocationCreationResponse {
    pub fn new() -> SyntheticsPrivateLocationCreationResponse {
        SyntheticsPrivateLocationCreationResponse {
            config: None,
            private_location: None,
            result_encryption: None,
        }
    }

    pub fn with_config(
        &mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> &mut Self {
        self.config = Some(value);
        self
    }

    pub fn with_private_location(
        &mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> &mut Self {
        self.private_location = Some(value);
        self
    }

    pub fn with_result_encryption(
        &mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption,
    ) -> &mut Self {
        self.result_encryption = Some(value);
        self
    }
}
impl Default for SyntheticsPrivateLocationCreationResponse {
    fn default() -> Self {
        Self::new()
    }
}
