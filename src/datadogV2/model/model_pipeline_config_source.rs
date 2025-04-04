// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A data source for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PipelineConfigSource {
    PipelineKafkaSource(Box<crate::datadogV2::model::PipelineKafkaSource>),
    PipelineDatadogAgentSource(Box<crate::datadogV2::model::PipelineDatadogAgentSource>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for PipelineConfigSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::PipelineKafkaSource>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(PipelineConfigSource::PipelineKafkaSource(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::PipelineDatadogAgentSource>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PipelineConfigSource::PipelineDatadogAgentSource(_v));
            }
        }

        return Ok(PipelineConfigSource::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
