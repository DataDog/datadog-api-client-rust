// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A destination for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineConfigDestinationItem {
    ObservabilityPipelineDatadogLogsDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestination>,
    ),
    ObservabilityPipelineAmazonS3Destination(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3Destination>,
    ),
    ObservabilityPipelineGoogleCloudStorageDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestination>,
    ),
    ObservabilityPipelineSplunkHecDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineSplunkHecDestination>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineConfigDestinationItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineDatadogLogsDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineDatadogLogsDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonS3Destination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineAmazonS3Destination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineGoogleCloudStorageDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineGoogleCloudStorageDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSplunkHecDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSplunkHecDestination(_v));
            }
        }

        return Ok(ObservabilityPipelineConfigDestinationItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
