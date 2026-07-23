// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Type-specific details for a content pack state. The set of fields present depends
/// on the content pack's `type`. When Cloud SIEM is inactive for the requesting organization, `onboarding` is returned instead of the content pack's usual type, such as `logs` or `vulnerability`.`
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringContentPackStateDetails {
    SecurityMonitoringContentPackLogsDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackLogsDetails>,
    ),
    SecurityMonitoringContentPackThreatIntelDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackThreatIntelDetails>,
    ),
    SecurityMonitoringContentPackEntityDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackEntityDetails>,
    ),
    SecurityMonitoringContentPackAuditDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackAuditDetails>,
    ),
    SecurityMonitoringContentPackAppSecDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackAppSecDetails>,
    ),
    SecurityMonitoringContentPackVulnerabilityDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackVulnerabilityDetails>,
    ),
    SecurityMonitoringContentPackOnboardingDetails(
        Box<crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetails>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringContentPackStateDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackLogsDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackLogsDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackThreatIntelDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackThreatIntelDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackEntityDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackEntityDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackAuditDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackAuditDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackAppSecDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackAppSecDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackVulnerabilityDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackVulnerabilityDetails(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringContentPackOnboardingDetails>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringContentPackStateDetails::SecurityMonitoringContentPackOnboardingDetails(_v));
            }
        }

        return Ok(SecurityMonitoringContentPackStateDetails::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
