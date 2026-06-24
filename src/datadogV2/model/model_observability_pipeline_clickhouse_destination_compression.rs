// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Compression setting for outbound HTTP requests to ClickHouse.
/// Can be specified as a shorthand string (`"gzip"` or `"none"`) or as an object
/// with an `algorithm` field and an optional `level` (gzip only, 1–9).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineClickhouseDestinationCompression {
    ObservabilityPipelineClickhouseDestinationCompressionAlgorithm(
        Box<
            crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm,
        >,
    ),
    ObservabilityPipelineClickhouseDestinationCompressionObject(
        Box<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionObject>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestinationCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm>>(value.clone()) {
			match *_v {
				crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm::UnparsedObject(_v) => {},
				_ => {
					return Ok(ObservabilityPipelineClickhouseDestinationCompression::ObservabilityPipelineClickhouseDestinationCompressionAlgorithm(_v))
				}
			}
		}
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationCompressionObject>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineClickhouseDestinationCompression::ObservabilityPipelineClickhouseDestinationCompressionObject(_v));
            }
        }

        return Ok(
            ObservabilityPipelineClickhouseDestinationCompression::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
