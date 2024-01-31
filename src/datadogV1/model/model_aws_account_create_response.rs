// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Response returned by the AWS Create Account call.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSAccountCreateResponse {
    /// AWS external_id.
    #[serde(rename = "external_id")]
    pub external_id: Option<String>,
}

impl AWSAccountCreateResponse {
    pub fn new() -> AWSAccountCreateResponse {
        AWSAccountCreateResponse { external_id: None }
    }

    pub fn with_external_id(&mut self, value: String) -> &mut Self {
        self.external_id = Some(value);
        self
    }
}
impl Default for AWSAccountCreateResponse {
    fn default() -> Self {
        Self::new()
    }
}
