// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication for creating the Databricks integration account. Exactly one method is set. Choosing `private_action_runner` leaves the `databricks-model-serving-metrics` dataflow unable to collect data. `bearer_token` is listed but always rejected on creation: Databricks accepts it only on accounts that already use it.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DatabricksIntegrationAccountAuthenticationRequest {
    DatabricksIntegrationAccountOAuthAuthRequest(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthRequest>,
    ),
    DatabricksIntegrationAccountPrivateActionRunnerAuthRequest(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthRequest>,
    ),
    DatabricksIntegrationAccountBearerTokenAuthRequest(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthRequest>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountAuthenticationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationRequest::DatabricksIntegrationAccountOAuthAuthRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::DatabricksIntegrationAccountPrivateActionRunnerAuthRequest,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationRequest::DatabricksIntegrationAccountPrivateActionRunnerAuthRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountBearerTokenAuthRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationRequest::DatabricksIntegrationAccountBearerTokenAuthRequest(_v));
            }
        }

        return Ok(
            DatabricksIntegrationAccountAuthenticationRequest::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
