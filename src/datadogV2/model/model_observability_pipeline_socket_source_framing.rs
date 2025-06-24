// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Framing method configuration for the socket source.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineSocketSourceFraming {
    ObservabilityPipelineSocketSourceFramingNewlineDelimited(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingNewlineDelimited>,
    ),
    ObservabilityPipelineSocketSourceFramingBytes(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytes>,
    ),
    ObservabilityPipelineSocketSourceFramingCharacterDelimited(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingCharacterDelimited>,
    ),
    ObservabilityPipelineSocketSourceFramingOctetCounting(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingOctetCounting>,
    ),
    ObservabilityPipelineSocketSourceFramingChunkedGelf(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingChunkedGelf>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineSocketSourceFraming {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingNewlineDelimited>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketSourceFraming::ObservabilityPipelineSocketSourceFramingNewlineDelimited(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingBytes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketSourceFraming::ObservabilityPipelineSocketSourceFramingBytes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<
                crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingCharacterDelimited,
            >,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketSourceFraming::ObservabilityPipelineSocketSourceFramingCharacterDelimited(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingOctetCounting>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketSourceFraming::ObservabilityPipelineSocketSourceFramingOctetCounting(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketSourceFramingChunkedGelf>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineSocketSourceFraming::ObservabilityPipelineSocketSourceFramingChunkedGelf(_v));
            }
        }

        return Ok(ObservabilityPipelineSocketSourceFraming::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
