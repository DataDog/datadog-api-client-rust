// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of a CI job.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventJob {
    /// A list of job IDs that this job depends on.
    #[serde(
        rename = "dependencies",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dependencies: Option<Option<Vec<String>>>,
    /// Time when the job run finished. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<crate::datadogV2::model::CIAppCIError>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<crate::datadogV2::model::CIAppGitInfo>>,
    /// The UUID for the job. It has to be unique within each pipeline execution.
    #[serde(rename = "id")]
    pub id: String,
    /// Used to distinguish between pipelines, stages, jobs, and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventJobLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// The name for the job.
    #[serde(rename = "name")]
    pub name: String,
    /// Contains information of the host running the pipeline, stage, job, or step.
    #[serde(rename = "node", default, with = "::serde_with::rust::double_option")]
    pub node: Option<Option<crate::datadogV2::model::CIAppHostInfo>>,
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
    /// The queue time in milliseconds, if applicable.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub queue_time: Option<Option<i64>>,
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
    /// Time when the job run instance started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: String,
    /// The final status of the job.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventJobStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// The URL to look at the job in the CI provider UI.
    #[serde(rename = "url")]
    pub url: String,
}

impl CIAppPipelineEventJob {
    pub fn new(
        end: String,
        id: String,
        level: crate::datadogV2::model::CIAppPipelineEventJobLevel,
        name: String,
        pipeline_name: String,
        pipeline_unique_id: String,
        start: String,
        status: crate::datadogV2::model::CIAppPipelineEventJobStatus,
        url: String,
    ) -> CIAppPipelineEventJob {
        CIAppPipelineEventJob {
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
            stage_id: None,
            stage_name: None,
            start,
            status,
            tags: None,
            url,
        }
    }

    pub fn with_dependencies(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.dependencies = Some(value);
        self
    }

    pub fn with_error(
        &mut self,
        value: Option<crate::datadogV2::model::CIAppCIError>,
    ) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn with_git(&mut self, value: Option<crate::datadogV2::model::CIAppGitInfo>) -> &mut Self {
        self.git = Some(value);
        self
    }

    pub fn with_metrics(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.metrics = Some(value);
        self
    }

    pub fn with_node(
        &mut self,
        value: Option<crate::datadogV2::model::CIAppHostInfo>,
    ) -> &mut Self {
        self.node = Some(value);
        self
    }

    pub fn with_parameters(
        &mut self,
        value: Option<std::collections::BTreeMap<String, String>>,
    ) -> &mut Self {
        self.parameters = Some(value);
        self
    }

    pub fn with_queue_time(&mut self, value: Option<i64>) -> &mut Self {
        self.queue_time = Some(value);
        self
    }

    pub fn with_stage_id(&mut self, value: Option<String>) -> &mut Self {
        self.stage_id = Some(value);
        self
    }

    pub fn with_stage_name(&mut self, value: Option<String>) -> &mut Self {
        self.stage_name = Some(value);
        self
    }

    pub fn with_tags(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
