// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A processor for the pipeline.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PipelineConfigProcessor {
    PipelineFilterProcessor(Box<crate::datadogV2::model::PipelineFilterProcessor>),
    ParseJSONProcessor(Box<crate::datadogV2::model::ParseJSONProcessor>),
    PipelineQuotaProcessor(Box<crate::datadogV2::model::PipelineQuotaProcessor>),
    PipelineAddFieldsProcessor(Box<crate::datadogV2::model::PipelineAddFieldsProcessor>),
    PipelineRemoveFieldsProcessor(Box<crate::datadogV2::model::PipelineRemoveFieldsProcessor>),
    PipelineRenameFieldsProcessor(Box<crate::datadogV2::model::PipelineRenameFieldsProcessor>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for PipelineConfigProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::PipelineFilterProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::PipelineFilterProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ParseJSONProcessor>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::ParseJSONProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::PipelineQuotaProcessor>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::PipelineQuotaProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::PipelineAddFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::PipelineAddFieldsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::PipelineRemoveFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::PipelineRemoveFieldsProcessor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::PipelineRenameFieldsProcessor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(PipelineConfigProcessor::PipelineRenameFieldsProcessor(_v));
            }
        }

        return Ok(PipelineConfigProcessor::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
