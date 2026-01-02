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
    ObservabilityPipelineCloudPremDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineCloudPremDestination>,
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
    ObservabilityPipelineSumoLogicDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestination>,
    ),
    ObservabilityPipelineElasticsearchDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestination>,
    ),
    ObservabilityPipelineRsyslogDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineRsyslogDestination>,
    ),
    ObservabilityPipelineSyslogNgDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineSyslogNgDestination>,
    ),
    AzureStorageDestination(Box<crate::datadogV2::model::AzureStorageDestination>),
    MicrosoftSentinelDestination(Box<crate::datadogV2::model::MicrosoftSentinelDestination>),
    ObservabilityPipelineGoogleChronicleDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestination>,
    ),
    ObservabilityPipelineNewRelicDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineNewRelicDestination>,
    ),
    ObservabilityPipelineSentinelOneDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineSentinelOneDestination>,
    ),
    ObservabilityPipelineOpenSearchDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineOpenSearchDestination>,
    ),
    ObservabilityPipelineAmazonOpenSearchDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestination>,
    ),
    ObservabilityPipelineSocketDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineSocketDestination>,
    ),
    ObservabilityPipelineAmazonSecurityLakeDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestination>,
    ),
    ObservabilityPipelineCrowdStrikeNextGenSiemDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineCrowdStrikeNextGenSiemDestination>,
    ),
    ObservabilityPipelineGooglePubSubDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestination>,
    ),
    ObservabilityPipelineKafkaDestination(
        Box<crate::datadogV2::model::ObservabilityPipelineKafkaDestination>,
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
            Box<crate::datadogV2::model::ObservabilityPipelineCloudPremDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineCloudPremDestination(_v));
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
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSumoLogicDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineElasticsearchDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineElasticsearchDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineRsyslogDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineRsyslogDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSyslogNgDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSyslogNgDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AzureStorageDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::AzureStorageDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::MicrosoftSentinelDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigDestinationItem::MicrosoftSentinelDestination(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineGoogleChronicleDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineNewRelicDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineNewRelicDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSentinelOneDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSentinelOneDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineOpenSearchDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineOpenSearchDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonOpenSearchDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineAmazonOpenSearchDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineSocketDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineSocketDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineAmazonSecurityLakeDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineAmazonSecurityLakeDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineCrowdStrikeNextGenSiemDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineCrowdStrikeNextGenSiemDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineGooglePubSubDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineGooglePubSubDestination(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineKafkaDestination>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ObservabilityPipelineConfigDestinationItem::ObservabilityPipelineKafkaDestination(_v));
            }
        }

        return Ok(ObservabilityPipelineConfigDestinationItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
