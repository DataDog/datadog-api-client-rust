// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Details of the CI pipeline event.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CIAppCreatePipelineEventRequestAttributesResource {
    CIAppPipelineEventPipeline(Box<crate::datadogV2::model::CIAppPipelineEventPipeline>),
    CIAppPipelineEventStage(Box<crate::datadogV2::model::CIAppPipelineEventStage>),
    CIAppPipelineEventJob(Box<crate::datadogV2::model::CIAppPipelineEventJob>),
    CIAppPipelineEventStep(Box<crate::datadogV2::model::CIAppPipelineEventStep>),
}
