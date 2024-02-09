// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Authentication part of the secrets.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationSecretsAuthentication {
    /// Access key for the private location.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Secret access key for the private location.
    #[serde(rename = "key")]
    pub key: Option<String>,
}

impl SyntheticsPrivateLocationSecretsAuthentication {
    pub fn new() -> SyntheticsPrivateLocationSecretsAuthentication {
        SyntheticsPrivateLocationSecretsAuthentication {
            id: None,
            key: None,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn key(&mut self, value: String) -> &mut Self {
        self.key = Some(value);
        self
    }
}

impl Default for SyntheticsPrivateLocationSecretsAuthentication {
    fn default() -> Self {
        Self::new()
    }
}
