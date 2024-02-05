// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details of the top level pipeline, build, or workflow of your CI.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventPipeline {
    /// Time when the pipeline run finished. It cannot be older than 18 hours in the past from the current time. The time format must be RFC3339.
    #[serde(rename = "end")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option")]
    pub error: Option<Option<crate::datadogV2::model::CIAppCIError>>,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
    /// Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git", default, with = "::serde_with::rust::double_option")]
    pub git: Option<Option<crate::datadogV2::model::CIAppGitInfo>>,
    /// Whether or not the pipeline was triggered manually by the user.
    #[serde(
        rename = "is_manual",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_manual: Option<Option<bool>>,
    /// Whether or not the pipeline was resumed after being blocked.
    #[serde(
        rename = "is_resumed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub is_resumed: Option<Option<bool>>,
    /// Used to distinguish between pipelines, stages, jobs, and steps.
    #[serde(rename = "level")]
    pub level: crate::datadogV2::model::CIAppPipelineEventPipelineLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// Name of the pipeline. All pipeline runs for the builds should have the same name.
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
    /// If the pipeline is triggered as child of another pipeline, this should contain the details of the parent pipeline.
    #[serde(
        rename = "parent_pipeline",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub parent_pipeline: Option<Option<crate::datadogV2::model::CIAppPipelineEventParentPipeline>>,
    /// Whether or not the pipeline was a partial retry of a previous attempt. A partial retry is one
    /// which only runs a subset of the original jobs.
    #[serde(rename = "partial_retry")]
    pub partial_retry: bool,
    /// Any ID used in the provider to identify the pipeline run even if it is not unique across retries.
    /// If the `pipeline_id` is unique, then both `unique_id` and `pipeline_id` can be set to the same value.
    #[serde(rename = "pipeline_id")]
    pub pipeline_id: Option<String>,
    /// If the pipeline is a retry, this should contain the details of the previous attempt.
    #[serde(
        rename = "previous_attempt",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub previous_attempt:
        Option<Option<crate::datadogV2::model::CIAppPipelineEventPreviousPipeline>>,
    /// The queue time in milliseconds, if applicable.
    #[serde(
        rename = "queue_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub queue_time: Option<Option<i64>>,
    /// Time when the pipeline run started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start")]
    pub start: String,
    /// The final status of the pipeline.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CIAppPipelineEventPipelineStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// UUID of the pipeline run. The ID has to be unique across retries and pipelines,
    /// including partial retries.
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url")]
    pub url: String,
}

impl CIAppPipelineEventPipeline {
    pub fn new(
        end: String,
        level: crate::datadogV2::model::CIAppPipelineEventPipelineLevel,
        name: String,
        partial_retry: bool,
        start: String,
        status: crate::datadogV2::model::CIAppPipelineEventPipelineStatus,
        unique_id: String,
        url: String,
    ) -> CIAppPipelineEventPipeline {
        CIAppPipelineEventPipeline {
            end,
            error: None,
            git: None,
            is_manual: None,
            is_resumed: None,
            level,
            metrics: None,
            name,
            node: None,
            parameters: None,
            parent_pipeline: None,
            partial_retry,
            pipeline_id: None,
            previous_attempt: None,
            queue_time: None,
            start,
            status,
            tags: None,
            unique_id,
            url,
        }
    }

    pub fn error(&mut self, value: Option<crate::datadogV2::model::CIAppCIError>) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn git(&mut self, value: Option<crate::datadogV2::model::CIAppGitInfo>) -> &mut Self {
        self.git = Some(value);
        self
    }

    pub fn is_manual(&mut self, value: Option<bool>) -> &mut Self {
        self.is_manual = Some(value);
        self
    }

    pub fn is_resumed(&mut self, value: Option<bool>) -> &mut Self {
        self.is_resumed = Some(value);
        self
    }

    pub fn metrics(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.metrics = Some(value);
        self
    }

    pub fn node(&mut self, value: Option<crate::datadogV2::model::CIAppHostInfo>) -> &mut Self {
        self.node = Some(value);
        self
    }

    pub fn parameters(
        &mut self,
        value: Option<std::collections::BTreeMap<String, String>>,
    ) -> &mut Self {
        self.parameters = Some(value);
        self
    }

    pub fn parent_pipeline(
        &mut self,
        value: Option<crate::datadogV2::model::CIAppPipelineEventParentPipeline>,
    ) -> &mut Self {
        self.parent_pipeline = Some(value);
        self
    }

    pub fn pipeline_id(&mut self, value: String) -> &mut Self {
        self.pipeline_id = Some(value);
        self
    }

    pub fn previous_attempt(
        &mut self,
        value: Option<crate::datadogV2::model::CIAppPipelineEventPreviousPipeline>,
    ) -> &mut Self {
        self.previous_attempt = Some(value);
        self
    }

    pub fn queue_time(&mut self, value: Option<i64>) -> &mut Self {
        self.queue_time = Some(value);
        self
    }

    pub fn tags(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
