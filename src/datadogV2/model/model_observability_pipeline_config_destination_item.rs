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

        return Ok(ObservabilityPipelineConfigDestinationItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
