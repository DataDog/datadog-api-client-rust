// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication for creating the GitHub Costs integration account. Exactly one method is set.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GitHubCostsIntegrationAccountAuthenticationRequest {
    GitHubCostsIntegrationAccountBearerTokenAuthRequest(
        Box<crate::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthRequest>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GitHubCostsIntegrationAccountAuthenticationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(GitHubCostsIntegrationAccountAuthenticationRequest::GitHubCostsIntegrationAccountBearerTokenAuthRequest(_v));
            }
        }

        return Ok(
            GitHubCostsIntegrationAccountAuthenticationRequest::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
