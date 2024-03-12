// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// If the pipeline is triggered as child of another pipeline, this should contain the details of the parent pipeline.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventParentPipeline {
    /// UUID of a pipeline.
    #[serde(rename = "id")]
    pub id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl CIAppPipelineEventParentPipeline {
    pub fn new(id: String) -> CIAppPipelineEventParentPipeline {
        CIAppPipelineEventParentPipeline { id, url: None }
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}
