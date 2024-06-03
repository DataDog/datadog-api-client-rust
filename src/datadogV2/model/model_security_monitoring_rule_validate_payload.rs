// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Validate a rule.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringRuleValidatePayload {
    SecurityMonitoringStandardRulePayload(
        Box<crate::datadogV2::model::SecurityMonitoringStandardRulePayload>,
    ),
    SecurityMonitoringSignalRulePayload(
        Box<crate::datadogV2::model::SecurityMonitoringSignalRulePayload>,
    ),
    CloudConfigurationRulePayload(Box<crate::datadogV2::model::CloudConfigurationRulePayload>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleValidatePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringStandardRulePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleValidatePayload::SecurityMonitoringStandardRulePayload(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringSignalRulePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleValidatePayload::SecurityMonitoringSignalRulePayload(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CloudConfigurationRulePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleValidatePayload::CloudConfigurationRulePayload(_v),
                );
            }
        }

        return Ok(SecurityMonitoringRuleValidatePayload::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
