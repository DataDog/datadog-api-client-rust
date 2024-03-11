// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Details of the CI pipeline event.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CIAppCreatePipelineEventRequestAttributesResource {
    CIAppPipelineEventPipeline(Box<crate::datadogV2::model::CIAppPipelineEventPipeline>),
    CIAppPipelineEventStage(Box<crate::datadogV2::model::CIAppPipelineEventStage>),
    CIAppPipelineEventJob(Box<crate::datadogV2::model::CIAppPipelineEventJob>),
    CIAppPipelineEventStep(Box<crate::datadogV2::model::CIAppPipelineEventStep>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for CIAppCreatePipelineEventRequestAttributesResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CIAppPipelineEventPipeline>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventPipeline(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CIAppPipelineEventStage>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventStage(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CIAppPipelineEventJob>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(
                    CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventJob(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CIAppPipelineEventStep>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(
                    CIAppCreatePipelineEventRequestAttributesResource::CIAppPipelineEventStep(_v),
                );
            }
        }

        return Ok(
            CIAppCreatePipelineEventRequestAttributesResource::UnparsedObject(
                crate::datadog::UnparsedObejct { value },
            ),
        );
    }
}
