// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCreatePipelineEventRequest {
    /// Data of the pipeline event to create.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::CIAppCreatePipelineEventRequestData>,
}

impl CIAppCreatePipelineEventRequest {
    pub fn new() -> CIAppCreatePipelineEventRequest {
        CIAppCreatePipelineEventRequest { data: None }
    }

    pub fn data(
        &mut self,
        value: crate::datadogV2::model::CIAppCreatePipelineEventRequestData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for CIAppCreatePipelineEventRequest {
    fn default() -> Self {
        Self::new()
    }
}
