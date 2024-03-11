// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Create a new rule.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringRuleCreatePayload {
    SecurityMonitoringStandardRuleCreatePayload(
        Box<crate::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload>,
    ),
    SecurityMonitoringSignalRuleCreatePayload(
        Box<crate::datadogV2::model::SecurityMonitoringSignalRuleCreatePayload>,
    ),
    CloudConfigurationRuleCreatePayload(
        Box<crate::datadogV2::model::CloudConfigurationRuleCreatePayload>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleCreatePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringStandardRuleCreatePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SecurityMonitoringRuleCreatePayload::SecurityMonitoringStandardRuleCreatePayload(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringSignalRuleCreatePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleCreatePayload::SecurityMonitoringSignalRuleCreatePayload(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CloudConfigurationRuleCreatePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleCreatePayload::CloudConfigurationRuleCreatePayload(_v),
                );
            }
        }

        return Ok(SecurityMonitoringRuleCreatePayload::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
