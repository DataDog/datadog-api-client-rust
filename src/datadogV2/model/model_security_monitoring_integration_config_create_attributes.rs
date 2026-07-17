// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The attributes of the entity context sync configuration to create.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringIntegrationConfigCreateAttributes {
    SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes(Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes>),
	SecurityMonitoringOktaIntegrationConfigCreateAttributes(Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationConfigCreateAttributes>),
	SecurityMonitoringEntraIdIntegrationConfigCreateAttributes(Box<crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationConfigCreateAttributes>),
	SecurityMonitoringCrowdStrikeIntegrationConfigCreateAttributes(Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationConfigCreateAttributes>),
	SecurityMonitoringSentinelOneIntegrationConfigCreateAttributes(Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationConfigCreateAttributes>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigCreateAttributes::SecurityMonitoringGoogleWorkspaceIntegrationConfigCreateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationConfigCreateAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigCreateAttributes::SecurityMonitoringOktaIntegrationConfigCreateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationConfigCreateAttributes,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigCreateAttributes::SecurityMonitoringEntraIdIntegrationConfigCreateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationConfigCreateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigCreateAttributes::SecurityMonitoringCrowdStrikeIntegrationConfigCreateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationConfigCreateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigCreateAttributes::SecurityMonitoringSentinelOneIntegrationConfigCreateAttributes(_v));
            }
        }

        return Ok(
            SecurityMonitoringIntegrationConfigCreateAttributes::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
