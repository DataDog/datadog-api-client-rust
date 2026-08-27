// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Compression configuration for archived logs. When omitted, logs are compressed with gzip
/// for backward compatibility.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineAmazonS3DestinationCompression {
    ObservabilityPipelineAmazonS3DestinationCompressionZstd(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationCompressionZstd>,
    ),
    ObservabilityPipelineAmazonS3DestinationCompressionGzip(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationCompressionGzip>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3DestinationCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationCompressionZstd>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineAmazonS3DestinationCompression::ObservabilityPipelineAmazonS3DestinationCompressionZstd(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3DestinationCompressionGzip>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineAmazonS3DestinationCompression::ObservabilityPipelineAmazonS3DestinationCompressionGzip(_v));
            }
        }

        return Ok(
            ObservabilityPipelineAmazonS3DestinationCompression::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
