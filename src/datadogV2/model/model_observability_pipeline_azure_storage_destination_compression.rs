// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Compression configuration for archived logs. When omitted, logs are compressed with gzip
/// for backward compatibility.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineAzureStorageDestinationCompression {
    ObservabilityPipelineAzureStorageDestinationCompressionZstd(
        Box<crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionZstd>,
    ),
    ObservabilityPipelineAzureStorageDestinationCompressionGzip(
        Box<crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionGzip>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAzureStorageDestinationCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionZstd>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureStorageDestinationCompression::ObservabilityPipelineAzureStorageDestinationCompressionZstd(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ObservabilityPipelineAzureStorageDestinationCompressionGzip>>(value.clone()) {
			if !_v._unparsed {
                return Ok(ObservabilityPipelineAzureStorageDestinationCompression::ObservabilityPipelineAzureStorageDestinationCompressionGzip(_v));
            }
        }

        return Ok(
            ObservabilityPipelineAzureStorageDestinationCompression::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
