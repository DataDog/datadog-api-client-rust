// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Framing method configuration.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSocketDestinationFraming {
    ObservabilityPipelineSocketDestinationFramingNewlineDelimited(Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimited>),
	ObservabilityPipelineSocketDestinationFramingBytes(Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingBytes>),
	ObservabilityPipelineSocketDestinationFramingCharacterDelimited(Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimited>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketDestinationFraming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingNewlineDelimited>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketDestinationFraming::ObservabilityPipelineSocketDestinationFramingNewlineDelimited(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingBytes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketDestinationFraming::ObservabilityPipelineSocketDestinationFramingBytes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineSocketDestinationFramingCharacterDelimited>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketDestinationFraming::ObservabilityPipelineSocketDestinationFramingCharacterDelimited(_v));
            }
        }

        return Ok(
            ObservabilityPipelineSocketDestinationFraming::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
