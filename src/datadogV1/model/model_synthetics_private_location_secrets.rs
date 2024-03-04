// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Secrets for the private location. Only present in the response when creating the private location.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationSecrets {
    /// Authentication part of the secrets.
    #[serde(rename = "authentication")]
    pub authentication:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationSecretsAuthentication>,
    /// Private key for the private location.
    #[serde(rename = "config_decryption")]
    pub config_decryption:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationSecretsConfigDecryption>,
}

impl SyntheticsPrivateLocationSecrets {
    pub fn new() -> SyntheticsPrivateLocationSecrets {
        SyntheticsPrivateLocationSecrets {
            authentication: None,
            config_decryption: None,
        }
    }

    pub fn authentication(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationSecretsAuthentication,
    ) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn config_decryption(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationSecretsConfigDecryption,
    ) -> Self {
        self.config_decryption = Some(value);
        self
    }
}

impl Default for SyntheticsPrivateLocationSecrets {
    fn default() -> Self {
        Self::new()
    }
}
