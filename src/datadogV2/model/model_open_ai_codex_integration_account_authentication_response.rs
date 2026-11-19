// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication configured on the OpenAI Codex integration account.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OpenAICodexIntegrationAccountAuthenticationResponse {
    OpenAICodexIntegrationAccountBearerTokenAuthResponse(
        Box<crate::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthResponse>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for OpenAICodexIntegrationAccountAuthenticationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::OpenAICodexIntegrationAccountBearerTokenAuthResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(OpenAICodexIntegrationAccountAuthenticationResponse::OpenAICodexIntegrationAccountBearerTokenAuthResponse(_v));
            }
        }

        return Ok(
            OpenAICodexIntegrationAccountAuthenticationResponse::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
