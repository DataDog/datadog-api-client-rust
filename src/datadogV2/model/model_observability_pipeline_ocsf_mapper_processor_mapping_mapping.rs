// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Defines a single mapping rule for transforming logs into the OCSF schema.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineOcsfMapperProcessorMappingMapping {
    ObservabilityPipelineOcsfMappingLibrary(
        Box<crate::datadogV2::model::ObservabilityPipelineOcsfMappingLibrary>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMapperProcessorMappingMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineOcsfMappingLibrary>,
        >(value.clone())
        {
            match *_v {
				crate::datadogV2::model::ObservabilityPipelineOcsfMappingLibrary::UnparsedObject(_v) => {},
				_ => {
					return Ok(ObservabilityPipelineOcsfMapperProcessorMappingMapping::ObservabilityPipelineOcsfMappingLibrary(_v))
				}
			}
        }

        return Ok(
            ObservabilityPipelineOcsfMapperProcessorMappingMapping::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
