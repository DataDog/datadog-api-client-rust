// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Encoding format for the destination.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineAmazonS3GenericEncoding {
    ObservabilityPipelineAmazonS3GenericEncodingJson(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingJson>,
    ),
    ObservabilityPipelineAmazonS3GenericEncodingParquet(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingParquet>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineAmazonS3GenericEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingJson>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineAmazonS3GenericEncoding::ObservabilityPipelineAmazonS3GenericEncodingJson(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3GenericEncodingParquet>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineAmazonS3GenericEncoding::ObservabilityPipelineAmazonS3GenericEncodingParquet(_v));
            }
        }

        return Ok(
            ObservabilityPipelineAmazonS3GenericEncoding::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
