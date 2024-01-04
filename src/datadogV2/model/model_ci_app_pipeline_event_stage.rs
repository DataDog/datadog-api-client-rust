// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of a CI stage.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventStage {
    /// A list of stage IDs that this stage depends on.
    #[serde(
        rename = "dependencies",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dependencies: Option<Option<Vec<String>>>,
    /// Time when the stage run finished. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<Box<crate::datadogV2::model::CIAppCIError>>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<Box<crate::datadogV2::model::CIAppGitInfo>>>,
    /// UUID for the stage. It has to be unique at least in the pipeline scope.
    #[serde(rename = "id")]
    pub id: String,
    /// Used to distinguish between pipelines, stages, jobs and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventStageLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// The name for the stage.
    #[serde(rename = "name")]
    pub name: String,
    /// Contains information of the host running the pipeline, stage, job, or step.
    #[serde(rename = "node", default, with = "::serde_with::rust::double_option")]
    pub node: Option<Option<Box<crate::datadogV2::model::CIAppHostInfo>>>,
    /// A map of key-value parameters or environment variables that were defined for the pipeline.
    #[serde(
        rename = "parameters",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parameters: Option<Option<std::collections::HashMap<String, String>>>,
    /// The parent pipeline name.
    #[serde(rename = "pipeline_name")]
    pub pipeline_name: String,
    /// The parent pipeline UUID.
    #[serde(rename = "pipeline_unique_id")]
    pub pipeline_unique_id: String,
    /// The queue time in milliseconds, if applicable.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub queue_time: Option<Option<i64>>,
    /// Time when the stage run started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: String,
    /// The final status of the stage.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventStageStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
}

impl CIAppPipelineEventStage {
    pub fn new(
        end: String,
        id: String,
        level: crate::datadogV2::model::CIAppPipelineEventStageLevel,
        name: String,
        pipeline_name: String,
        pipeline_unique_id: String,
        start: String,
        status: crate::datadogV2::model::CIAppPipelineEventStageStatus,
    ) -> CIAppPipelineEventStage {
        CIAppPipelineEventStage {
            dependencies: None,
            end,
            error: None,
            git: None,
            id,
            level,
            metrics: None,
            name,
            node: None,
            parameters: None,
            pipeline_name,
            pipeline_unique_id,
            queue_time: None,
            start,
            status,
            tags: None,
        }
    }
}