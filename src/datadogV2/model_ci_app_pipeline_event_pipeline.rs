// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventPipeline {
    /// Time when the pipeline run finished. It cannot be older than 18 hours in the past from the current time. The time format must be RFC3339.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub error: NullableCIAppCIError,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git")]
    pub git: NullableCIAppGitInfo,
    /// Whether or not the pipeline was triggered manually by the user.
    #[serde(rename = "is_manual", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub is_manual: Option<Bool>,
    /// Whether or not the pipeline was resumed after being blocked.
    #[serde(rename = "is_resumed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub is_resumed: Option<Bool>,
    /// Used to distinguish between pipelines, stages, jobs, and steps.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: CIAppPipelineEventPipelineLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(rename = "metrics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub metrics: datadog.NullableList[String],
    /// Name of the pipeline. All pipeline runs for the builds should have the same name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Contains information of the host running the pipeline, stage, job, or step.
    #[serde(rename = "node", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub node: NullableCIAppHostInfo,
    /// A map of key-value parameters or environment variables that were defined for the pipeline.
    #[serde(rename = "parameters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub parameters: map[string]String,
    /// If the pipeline is triggered as child of another pipeline, this should contain the details of the parent pipeline.
    #[serde(rename = "parent_pipeline")]
    pub parent_pipeline: NullableCIAppPipelineEventParentPipeline,
    /// Whether or not the pipeline was a partial retry of a previous attempt. A partial retry is one
which only runs a subset of the original jobs.
    #[serde(rename = "partial_retry", skip_serializing_if = "Option::is_none")]
    pub partial_retry: bool,
    /// Any ID used in the provider to identify the pipeline run even if it is not unique across retries.
If the `pipeline_id` is unique, then both `unique_id` and `pipeline_id` can be set to the same value.
    #[serde(rename = "pipeline_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_id: String,
    /// If the pipeline is a retry, this should contain the details of the previous attempt.
    #[serde(rename = "previous_attempt")]
    pub previous_attempt: NullableCIAppPipelineEventPreviousPipeline,
    /// The queue time in milliseconds, if applicable.
    #[serde(rename = "queue_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub queue_time: Option<Int64>,
    /// Time when the pipeline run started (it should not include any queue time). The time format must be RFC3339.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: String,
    /// The final status of the pipeline.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: CIAppPipelineEventPipelineStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tags: datadog.NullableList[String],
    /// UUID of the pipeline run. The ID has to be unique across retries and pipelines,
including partial retries.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

