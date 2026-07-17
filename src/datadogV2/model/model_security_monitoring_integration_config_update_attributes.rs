// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Fields to update on the entity context sync configuration. All fields other than the integration type are optional.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringIntegrationConfigUpdateAttributes {
    SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes(Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes>),
	SecurityMonitoringOktaIntegrationConfigUpdateAttributes(Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationConfigUpdateAttributes>),
	SecurityMonitoringEntraIdIntegrationConfigUpdateAttributes(Box<crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationConfigUpdateAttributes>),
	SecurityMonitoringCrowdStrikeIntegrationConfigUpdateAttributes(Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationConfigUpdateAttributes>),
	SecurityMonitoringSentinelOneIntegrationConfigUpdateAttributes(Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationConfigUpdateAttributes>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringIntegrationConfigUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigUpdateAttributes::SecurityMonitoringGoogleWorkspaceIntegrationConfigUpdateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringOktaIntegrationConfigUpdateAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigUpdateAttributes::SecurityMonitoringOktaIntegrationConfigUpdateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::SecurityMonitoringEntraIdIntegrationConfigUpdateAttributes,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigUpdateAttributes::SecurityMonitoringEntraIdIntegrationConfigUpdateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringCrowdStrikeIntegrationConfigUpdateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigUpdateAttributes::SecurityMonitoringCrowdStrikeIntegrationConfigUpdateAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SecurityMonitoringSentinelOneIntegrationConfigUpdateAttributes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(SecurityMonitoringIntegrationConfigUpdateAttributes::SecurityMonitoringSentinelOneIntegrationConfigUpdateAttributes(_v));
            }
        }

        return Ok(
            SecurityMonitoringIntegrationConfigUpdateAttributes::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
