// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Details of the top level pipeline, build, or workflow of your CI.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CIAppPipelineEventPipeline {
    CIAppPipelineEventFinishedPipeline(
        Box<crate::datadogV2::model::CIAppPipelineEventFinishedPipeline>,
    ),
    CIAppPipelineEventInProgressPipeline(
        Box<crate::datadogV2::model::CIAppPipelineEventInProgressPipeline>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CIAppPipelineEventPipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CIAppPipelineEventFinishedPipeline>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CIAppPipelineEventPipeline::CIAppPipelineEventFinishedPipeline(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CIAppPipelineEventInProgressPipeline>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CIAppPipelineEventPipeline::CIAppPipelineEventInProgressPipeline(_v));
            }
        }

        return Ok(CIAppPipelineEventPipeline::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
