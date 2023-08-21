// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventPreviousPipeline {
    /// UUID of a pipeline.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

