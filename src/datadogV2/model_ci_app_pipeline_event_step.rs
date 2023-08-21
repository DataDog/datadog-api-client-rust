// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEventStep {
    /// Time when the step run finished. The time format must be RFC3339.
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: String,
    /// Contains information of the CI error.
    #[serde(rename = "error", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub error: NullableCIAppCIError,
    /// If pipelines are triggered due to actions to a Git repository, then all payloads must contain this.
Note that either `tag` or `branch` has to be provided, but not both.
    #[serde(rename = "git")]
    pub git: NullableCIAppGitInfo,
    /// UUID for the step. It has to be unique within each pipeline execution.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The parent job UUID (if applicable).
    #[serde(rename = "job_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub job_id: Option<String>,
    /// The parent job name (if applicable).
    #[serde(rename = "job_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub job_name: Option<String>,
    /// Used to distinguish between pipelines, stages, jobs and steps.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: CIAppPipelineEventStepLevel,
    /// A list of user-defined metrics. The metrics must follow the `key:value` pattern and the value must be numeric.
    #[serde(rename = "metrics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub metrics: datadog.NullableList[String],
    /// The name for the step.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Contains information of the host running the pipeline, stage, job, or step.
    #[serde(rename = "node", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub node: NullableCIAppHostInfo,
    /// A map of key-value parameters or environment variables that were defined for the pipeline.
    #[serde(rename = "parameters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub parameters: map[string]String,
    /// The parent pipeline name.
    #[serde(rename = "pipeline_name", skip_serializing_if = "Option::is_none")]
    pub pipeline_name: String,
    /// The parent pipeline UUID.
    #[serde(rename = "pipeline_unique_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_unique_id: String,
    /// The parent stage UUID (if applicable).
    #[serde(rename = "stage_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub stage_id: Option<String>,
    /// The parent stage name (if applicable).
    #[serde(rename = "stage_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub stage_name: Option<String>,
    /// Time when the step run started. The time format must be RFC3339.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: String,
    /// The final status of the step.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: CIAppPipelineEventStepStatus,
    /// A list of user-defined tags. The tags must follow the `key:value` pattern.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub tags: datadog.NullableList[String],
    /// The URL to look at the step in the CI provider UI.
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
}

