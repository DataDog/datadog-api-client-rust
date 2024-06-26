// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Convert a rule from JSON to Terraform.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SecurityMonitoringRuleConvertPayload {
    SecurityMonitoringStandardRulePayload(
        Box<crate::datadogV2::model::SecurityMonitoringStandardRulePayload>,
    ),
    SecurityMonitoringSignalRulePayload(
        Box<crate::datadogV2::model::SecurityMonitoringSignalRulePayload>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleConvertPayload {
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
                    SecurityMonitoringRuleConvertPayload::SecurityMonitoringStandardRulePayload(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SecurityMonitoringSignalRulePayload>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    SecurityMonitoringRuleConvertPayload::SecurityMonitoringSignalRulePayload(_v),
                );
            }
        }

        return Ok(SecurityMonitoringRuleConvertPayload::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
