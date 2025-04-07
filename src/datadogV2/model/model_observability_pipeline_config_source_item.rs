// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A data source for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObservabilityPipelineConfigSourceItem {
    ObservabilityPipelineKafkaSource(
        Box<crate::datadogV2::model::ObservabilityPipelineKafkaSource>,
    ),
    ObservabilityPipelineDatadogAgentSource(
        Box<crate::datadogV2::model::ObservabilityPipelineDatadogAgentSource>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ObservabilityPipelineConfigSourceItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineKafkaSource>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigSourceItem::ObservabilityPipelineKafkaSource(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ObservabilityPipelineDatadogAgentSource>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ObservabilityPipelineConfigSourceItem::ObservabilityPipelineDatadogAgentSource(
                        _v,
                    ),
                );
            }
        }

        return Ok(ObservabilityPipelineConfigSourceItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
