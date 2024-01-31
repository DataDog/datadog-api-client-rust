// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Details of the CI pipeline event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CIAppCreatePipelineEventRequestAttributesResource {
    CIAppPipelineEventPipeline(crate::datadogV2::model::CIAppPipelineEventPipeline),
    CIAppPipelineEventStage(crate::datadogV2::model::CIAppPipelineEventStage),
    CIAppPipelineEventJob(crate::datadogV2::model::CIAppPipelineEventJob),
    CIAppPipelineEventStep(crate::datadogV2::model::CIAppPipelineEventStep),
}
