// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The minimal threshold to apply to the matching partition.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold {
    ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes(Box<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes>),
	ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep(Box<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdBytes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold::ObservabilityPipelineSpikeProtectionProcessorMinimalThresholdAlwaysKeep(_v));
            }
        }

        return Ok(
            ObservabilityPipelineSpikeProtectionProcessorMinimalThreshold::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
