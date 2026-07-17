// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The credentials to validate against the external entity source.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringIntegrationCredentialsValidateAttributes {
    SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes(Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes>),
	SecurityMonitoringOktaIntegrationCredentialsValidateAttributes(Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationCredentialsValidateAttributes>),
	SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes(Box<crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes>),
	SecurityMonitoringCrowdStrikeIntegrationCredentialsValidateAttributes(Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationCredentialsValidateAttributes>),
	SecurityMonitoringSentinelOneIntegrationCredentialsValidateAttributes(Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationCredentialsValidateAttributes>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationCredentialsValidateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationCredentialsValidateAttributes::SecurityMonitoringGoogleWorkspaceIntegrationCredentialsValidateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationCredentialsValidateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationCredentialsValidateAttributes::SecurityMonitoringOktaIntegrationCredentialsValidateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationCredentialsValidateAttributes::SecurityMonitoringEntraIdIntegrationCredentialsValidateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationCredentialsValidateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationCredentialsValidateAttributes::SecurityMonitoringCrowdStrikeIntegrationCredentialsValidateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationCredentialsValidateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationCredentialsValidateAttributes::SecurityMonitoringSentinelOneIntegrationCredentialsValidateAttributes(_v));
            }
        }

        return Ok(
            SecurityMonitoringIntegrationCredentialsValidateAttributes::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
