// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication configured on the Databricks integration account.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DatabricksIntegrationAccountAuthenticationResponse {
    DatabricksIntegrationAccountOAuthAuthResponse(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthResponse>,
    ),
    IntegrationAccountPrivateActionRunnerAuthResponse(
        Box<crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthResponse>,
    ),
    DatabricksIntegrationAccountPatAuthResponse(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountPatAuthResponse>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountAuthenticationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationResponse::DatabricksIntegrationAccountOAuthAuthResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationResponse::IntegrationAccountPrivateActionRunnerAuthResponse(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountPatAuthResponse>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationResponse::DatabricksIntegrationAccountPatAuthResponse(_v));
            }
        }

        return Ok(
            DatabricksIntegrationAccountAuthenticationResponse::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
