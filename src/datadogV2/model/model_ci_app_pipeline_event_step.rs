// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of a CI step.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventStep {
    /// Time when the step run finished. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<Box<crate::datadogV2::model::CIAppCIError>>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<Box<crate::datadogV2::model::CIAppGitInfo>>>,
    /// UUID for the step. It has to be unique within each pipeline execution.
    #[serde(rename = "id")]
    pub id: String,
    /// The parent job UUID (if applicable).
    #[serde(rename = "job_id", default, with = "::serde_with::rust::double_option")]
    pub job_id: Option<Option<String>>,
    /// The parent job name (if applicable).
    #[serde(
        rename = "job_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub job_name: Option<Option<String>>,
    /// Used to distinguish between pipelines, stages, jobs and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventStepLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// The name for the step.
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
    pub parameters: Option<Option<std::collections::BTreeMap<String, String>>>,
    /// The parent pipeline name.
    #[serde(rename = "pipeline_name")]
    pub pipeline_name: String,
    /// The parent pipeline UUID.
    #[serde(rename = "pipeline_unique_id")]
    pub pipeline_unique_id: String,
    /// The parent stage UUID (if applicable).
    #[serde(
        rename = "stage_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stage_id: Option<Option<String>>,
    /// The parent stage name (if applicable).
    #[serde(
        rename = "stage_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub stage_name: Option<Option<String>>,
    /// Time when the step run started. The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: String,
    /// The final status of the step.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventStepStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// The URL to look at the step in the CI provider UI.
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option")]
    pub url: Option<Option<String>>,
}

impl CIAppPipelineEventStep {
    pub fn new(
        end: String,
        id: String,
        level: crate::datadogV2::model::CIAppPipelineEventStepLevel,
        name: String,
        pipeline_name: String,
        pipeline_unique_id: String,
        start: String,
        status: crate::datadogV2::model::CIAppPipelineEventStepStatus,
    ) -> CIAppPipelineEventStep {
        CIAppPipelineEventStep {
            end,
            error: None,
            git: None,
            id,
            job_id: None,
            job_name: None,
            level,
            metrics: None,
            name,
            node: None,
            parameters: None,
            pipeline_name,
            pipeline_unique_id,
            stage_id: None,
            stage_name: None,
            start,
            status,
            tags: None,
            url: None,
        }
    }
}
