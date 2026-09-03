// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication for updating the Databricks integration account. Exactly one method is set. Choosing `private-action-runner` leaves the `databricks-model-serving-metrics` dataflow unable to collect data. `pat` is accepted only on accounts that already use it, so it cannot move an account onto personal access token authentication.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DatabricksIntegrationAccountAuthenticationUpdate {
    DatabricksIntegrationAccountOAuthAuthUpdate(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthUpdate>,
    ),
    IntegrationAccountPrivateActionRunnerAuthRequest(
        Box<crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthRequest>,
    ),
    DatabricksIntegrationAccountPatAuthUpdate(
        Box<crate::datadogV2::model::DatabricksIntegrationAccountPatAuthUpdate>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DatabricksIntegrationAccountAuthenticationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountOAuthAuthUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationUpdate::DatabricksIntegrationAccountOAuthAuthUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IntegrationAccountPrivateActionRunnerAuthRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationUpdate::IntegrationAccountPrivateActionRunnerAuthRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatabricksIntegrationAccountPatAuthUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DatabricksIntegrationAccountAuthenticationUpdate::DatabricksIntegrationAccountPatAuthUpdate(_v));
            }
        }

        return Ok(
            DatabricksIntegrationAccountAuthenticationUpdate::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
