// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Configuration for buffer settings on destination components.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineBufferOptions {
    ObservabilityPipelineDiskBufferOptions(
        Box<crate::datadogV2::model::ObservabilityPipelineDiskBufferOptions>,
    ),
    ObservabilityPipelineMemoryBufferOptions(
        Box<crate::datadogV2::model::ObservabilityPipelineMemoryBufferOptions>,
    ),
    ObservabilityPipelineMemoryBufferSizeOptions(
        Box<crate::datadogV2::model::ObservabilityPipelineMemoryBufferSizeOptions>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineBufferOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineDiskBufferOptions>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineBufferOptions::ObservabilityPipelineDiskBufferOptions(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineMemoryBufferOptions>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineBufferOptions::ObservabilityPipelineMemoryBufferOptions(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineMemoryBufferSizeOptions>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineBufferOptions::ObservabilityPipelineMemoryBufferSizeOptions(_v));
            }
        }

        return Ok(ObservabilityPipelineBufferOptions::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
