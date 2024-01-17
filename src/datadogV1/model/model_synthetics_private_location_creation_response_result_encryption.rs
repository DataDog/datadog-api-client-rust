// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Public key for the result encryption.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationCreationResponseResultEncryption {
    /// Fingerprint for the encryption key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Public key for result encryption.
    #[serde(rename = "key")]
    pub key: Option<String>,
}

impl SyntheticsPrivateLocationCreationResponseResultEncryption {
    pub fn new() -> SyntheticsPrivateLocationCreationResponseResultEncryption {
        SyntheticsPrivateLocationCreationResponseResultEncryption {
            id: None,
            key: None,
        }
    }
}
impl Default for SyntheticsPrivateLocationCreationResponseResultEncryption {
    fn default() -> Self {
        Self::new()
    }
}
