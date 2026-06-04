// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ExportLLMObsDatasetOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::export_llm_obs_dataset`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ExportLLMObsDatasetOptionalParams {
    /// Export format for the dataset contents. Only `csv` is currently supported.
    pub format: Option<crate::datadogV2::model::LLMObsDatasetExportFormat>,
    /// Version of the dataset to export. If omitted, the current version is used. Must be between 0 and the current version of the dataset, inclusive.
    pub version: Option<i64>,
}

impl ExportLLMObsDatasetOptionalParams {
    /// Export format for the dataset contents. Only `csv` is currently supported.
    pub fn format(mut self, value: crate::datadogV2::model::LLMObsDatasetExportFormat) -> Self {
        self.format = Some(value);
        self
    }
    /// Version of the dataset to export. If omitted, the current version is used. Must be between 0 and the current version of the dataset, inclusive.
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }
}

/// GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::get_llm_obs_annotated_interactions_by_trace_i_ds`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams {
    /// Pagination offset. Must be >= 0. Defaults to 0.
    pub offset: Option<i32>,
    /// Maximum number of results to return. Must be > 0. Defaults to 100.
    pub limit: Option<i32>,
}

impl GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams {
    /// Pagination offset. Must be >= 0. Defaults to 0.
    pub fn offset(mut self, value: i32) -> Self {
        self.offset = Some(value);
        self
    }
    /// Maximum number of results to return. Must be > 0. Defaults to 100.
    pub fn limit(mut self, value: i32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// ListLLMObsAnnotationQueuesOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_annotation_queues`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsAnnotationQueuesOptionalParams {
    /// Filter annotation queues by project ID. Cannot be used together with `queueIds`.
    pub project_id: Option<String>,
    /// Filter annotation queues by queue IDs (comma-separated). Cannot be used together with `projectId`.
    pub queue_ids: Option<Vec<String>>,
}

impl ListLLMObsAnnotationQueuesOptionalParams {
    /// Filter annotation queues by project ID. Cannot be used together with `queueIds`.
    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }
    /// Filter annotation queues by queue IDs (comma-separated). Cannot be used together with `projectId`.
    pub fn queue_ids(mut self, value: Vec<String>) -> Self {
        self.queue_ids = Some(value);
        self
    }
}

/// ListLLMObsDatasetRecordsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_dataset_records`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsDatasetRecordsOptionalParams {
    /// Retrieve records from a specific dataset version. Defaults to the current version.
    pub filter_version: Option<i64>,
    /// Use the Pagination cursor to retrieve the next page of results.
    pub page_cursor: Option<String>,
    /// Maximum number of results to return per page.
    pub page_limit: Option<i64>,
}

impl ListLLMObsDatasetRecordsOptionalParams {
    /// Retrieve records from a specific dataset version. Defaults to the current version.
    pub fn filter_version(mut self, value: i64) -> Self {
        self.filter_version = Some(value);
        self
    }
    /// Use the Pagination cursor to retrieve the next page of results.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of results to return per page.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListLLMObsDatasetsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_datasets`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsDatasetsOptionalParams {
    /// Filter datasets by name.
    pub filter_name: Option<String>,
    /// Filter datasets by dataset ID.
    pub filter_id: Option<String>,
    /// Use the Pagination cursor to retrieve the next page of results.
    pub page_cursor: Option<String>,
    /// Maximum number of results to return per page.
    pub page_limit: Option<i64>,
}

impl ListLLMObsDatasetsOptionalParams {
    /// Filter datasets by name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter datasets by dataset ID.
    pub fn filter_id(mut self, value: String) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Use the Pagination cursor to retrieve the next page of results.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of results to return per page.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListLLMObsExperimentEventsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_experiment_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsExperimentEventsOptionalParams {
    /// Maximum number of spans to return per page. Defaults to 5000.
    pub page_limit: Option<i64>,
    /// Opaque cursor from a previous response to fetch the next page of results.
    pub page_cursor: Option<String>,
}

impl ListLLMObsExperimentEventsOptionalParams {
    /// Maximum number of spans to return per page. Defaults to 5000.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Opaque cursor from a previous response to fetch the next page of results.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
}

/// ListLLMObsExperimentsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_experiments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsExperimentsOptionalParams {
    /// Filter experiments by project ID. Required if `filter[dataset_id]` is not provided.
    pub filter_project_id: Option<String>,
    /// Filter experiments by dataset ID.
    pub filter_dataset_id: Option<String>,
    /// Filter experiments by experiment ID. Can be specified multiple times.
    pub filter_id: Option<String>,
    /// Filter experiments by their exact run name.
    pub filter_name: Option<String>,
    /// Filter by logical experiment name. This is the `name` field set when creating an experiment through `POST /experiments`. Returns all experiment runs that share the same name, enabling cross-commit and cross-branch comparisons.
    pub filter_experiment: Option<String>,
    /// Filter by JSONB metadata containment. Provide a JSON object string where
    /// experiments whose metadata contains all specified key-value pairs are returned.
    /// For example: `{"commit":"abc123","branch":"main"}`.
    pub filter_metadata: Option<String>,
    /// Filter experiments by the ID of their parent (baseline) experiment. Returns all experiments that were run against the given baseline. Can be specified multiple times.
    pub filter_parent_experiment_id: Option<String>,
    /// When `true`, return only soft-deleted experiments. Defaults to `false`.
    pub filter_is_deleted: Option<bool>,
    /// When `true`, enrich each experiment with its author's user data in the `author` field.
    pub include_user_data: Option<bool>,
    /// When `true`, enrich each experiment with its dataset name in the `dataset_name` field.
    pub include_dataset_names: Option<bool>,
    /// Use the pagination cursor returned in `meta.after` to retrieve the next page of results.
    pub page_cursor: Option<String>,
    /// Maximum number of results to return per page. Values above 5000 are clamped
    /// to 5000. Defaults to 5000.
    pub page_limit: Option<i64>,
}

impl ListLLMObsExperimentsOptionalParams {
    /// Filter experiments by project ID. Required if `filter[dataset_id]` is not provided.
    pub fn filter_project_id(mut self, value: String) -> Self {
        self.filter_project_id = Some(value);
        self
    }
    /// Filter experiments by dataset ID.
    pub fn filter_dataset_id(mut self, value: String) -> Self {
        self.filter_dataset_id = Some(value);
        self
    }
    /// Filter experiments by experiment ID. Can be specified multiple times.
    pub fn filter_id(mut self, value: String) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter experiments by their exact run name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter by logical experiment name. This is the `name` field set when creating an experiment through `POST /experiments`. Returns all experiment runs that share the same name, enabling cross-commit and cross-branch comparisons.
    pub fn filter_experiment(mut self, value: String) -> Self {
        self.filter_experiment = Some(value);
        self
    }
    /// Filter by JSONB metadata containment. Provide a JSON object string where
    /// experiments whose metadata contains all specified key-value pairs are returned.
    /// For example: `{"commit":"abc123","branch":"main"}`.
    pub fn filter_metadata(mut self, value: String) -> Self {
        self.filter_metadata = Some(value);
        self
    }
    /// Filter experiments by the ID of their parent (baseline) experiment. Returns all experiments that were run against the given baseline. Can be specified multiple times.
    pub fn filter_parent_experiment_id(mut self, value: String) -> Self {
        self.filter_parent_experiment_id = Some(value);
        self
    }
    /// When `true`, return only soft-deleted experiments. Defaults to `false`.
    pub fn filter_is_deleted(mut self, value: bool) -> Self {
        self.filter_is_deleted = Some(value);
        self
    }
    /// When `true`, enrich each experiment with its author's user data in the `author` field.
    pub fn include_user_data(mut self, value: bool) -> Self {
        self.include_user_data = Some(value);
        self
    }
    /// When `true`, enrich each experiment with its dataset name in the `dataset_name` field.
    pub fn include_dataset_names(mut self, value: bool) -> Self {
        self.include_dataset_names = Some(value);
        self
    }
    /// Use the pagination cursor returned in `meta.after` to retrieve the next page of results.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of results to return per page. Values above 5000 are clamped
    /// to 5000. Defaults to 5000.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListLLMObsProjectsOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_projects`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsProjectsOptionalParams {
    /// Filter projects by project ID.
    pub filter_id: Option<String>,
    /// Filter projects by name.
    pub filter_name: Option<String>,
    /// Use the Pagination cursor to retrieve the next page of results.
    pub page_cursor: Option<String>,
    /// Maximum number of results to return per page.
    pub page_limit: Option<i64>,
}

impl ListLLMObsProjectsOptionalParams {
    /// Filter projects by project ID.
    pub fn filter_id(mut self, value: String) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter projects by name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Use the Pagination cursor to retrieve the next page of results.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Maximum number of results to return per page.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
}

/// ListLLMObsSpansOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::list_llm_obs_spans`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListLLMObsSpansOptionalParams {
    /// Start of the time range. Accepts ISO 8601 or relative format (e.g., `now-15m`). Defaults to `now-15m`.
    pub filter_from: Option<String>,
    /// End of the time range. Accepts ISO 8601 or relative format. Defaults to `now`.
    pub filter_to: Option<String>,
    /// Search query using LLM Observability query syntax. Supports attribute filters using the field:value syntax (e.g. session_id, trace_id, ml_app, meta.span.kind). When provided, structured field filters (`filter[span_id]`, `filter[trace_id]`, etc.) are ignored.
    pub filter_query: Option<String>,
    /// Filter by exact span ID.
    pub filter_span_id: Option<String>,
    /// Filter by exact trace ID.
    pub filter_trace_id: Option<String>,
    /// Filter by span kind (e.g., llm, agent, tool, task, workflow).
    pub filter_span_kind: Option<String>,
    /// Filter by span name.
    pub filter_span_name: Option<String>,
    /// Filter by ML application name.
    pub filter_ml_app: Option<String>,
    /// Maximum number of spans to return. Defaults to `10`.
    pub page_limit: Option<i64>,
    /// Cursor from the previous response to retrieve the next page.
    pub page_cursor: Option<String>,
    /// Sort order for the results.
    pub sort: Option<String>,
    /// Whether to include attachment data in the response. Defaults to `true`.
    pub include_attachments: Option<bool>,
}

impl ListLLMObsSpansOptionalParams {
    /// Start of the time range. Accepts ISO 8601 or relative format (e.g., `now-15m`). Defaults to `now-15m`.
    pub fn filter_from(mut self, value: String) -> Self {
        self.filter_from = Some(value);
        self
    }
    /// End of the time range. Accepts ISO 8601 or relative format. Defaults to `now`.
    pub fn filter_to(mut self, value: String) -> Self {
        self.filter_to = Some(value);
        self
    }
    /// Search query using LLM Observability query syntax. Supports attribute filters using the field:value syntax (e.g. session_id, trace_id, ml_app, meta.span.kind). When provided, structured field filters (`filter[span_id]`, `filter[trace_id]`, etc.) are ignored.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Filter by exact span ID.
    pub fn filter_span_id(mut self, value: String) -> Self {
        self.filter_span_id = Some(value);
        self
    }
    /// Filter by exact trace ID.
    pub fn filter_trace_id(mut self, value: String) -> Self {
        self.filter_trace_id = Some(value);
        self
    }
    /// Filter by span kind (e.g., llm, agent, tool, task, workflow).
    pub fn filter_span_kind(mut self, value: String) -> Self {
        self.filter_span_kind = Some(value);
        self
    }
    /// Filter by span name.
    pub fn filter_span_name(mut self, value: String) -> Self {
        self.filter_span_name = Some(value);
        self
    }
    /// Filter by ML application name.
    pub fn filter_ml_app(mut self, value: String) -> Self {
        self.filter_ml_app = Some(value);
        self
    }
    /// Maximum number of spans to return. Defaults to `10`.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Cursor from the previous response to retrieve the next page.
    pub fn page_cursor(mut self, value: String) -> Self {
        self.page_cursor = Some(value);
        self
    }
    /// Sort order for the results.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Whether to include attachment data in the response. Defaults to `true`.
    pub fn include_attachments(mut self, value: bool) -> Self {
        self.include_attachments = Some(value);
        self
    }
}

/// UploadLLMObsDatasetRecordsFileOptionalParams is a struct for passing parameters to the method [`LLMObservabilityAPI::upload_llm_obs_dataset_records_file`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UploadLLMObsDatasetRecordsFileOptionalParams {
    /// The records file to upload. Currently only CSV is supported. The file must include an `input` column. Optional columns include `id`, `expected_output`, `metadata`, and `tags`.
    pub file: Option<Vec<u8>>,
    /// Whether to skip records whose `input` already exists in the dataset. Defaults to `false`.
    pub deduplicate: Option<bool>,
    /// Whether to overwrite existing records that share the same user-provided `id`. Defaults to `true`.
    pub overwrite: Option<bool>,
    /// Tags to apply to every uploaded record, in addition to any tags defined on individual rows. Can be repeated, e.g. `tags=env:prod&tags=team:ai`.
    pub tags: Option<Vec<String>>,
    /// Whether to enrich the response with user metadata.
    pub include_user_data: Option<bool>,
}

impl UploadLLMObsDatasetRecordsFileOptionalParams {
    /// The records file to upload. Currently only CSV is supported. The file must include an `input` column. Optional columns include `id`, `expected_output`, `metadata`, and `tags`.
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }
    /// Whether to skip records whose `input` already exists in the dataset. Defaults to `false`.
    pub fn deduplicate(mut self, value: bool) -> Self {
        self.deduplicate = Some(value);
        self
    }
    /// Whether to overwrite existing records that share the same user-provided `id`. Defaults to `true`.
    pub fn overwrite(mut self, value: bool) -> Self {
        self.overwrite = Some(value);
        self
    }
    /// Tags to apply to every uploaded record, in addition to any tags defined on individual rows. Can be repeated, e.g. `tags=env:prod&tags=team:ai`.
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
    /// Whether to enrich the response with user metadata.
    pub fn include_user_data(mut self, value: bool) -> Self {
        self.include_user_data = Some(value);
        self
    }
}

/// AggregateLLMObsExperimentationError is a struct for typed errors of method [`LLMObservabilityAPI::aggregate_llm_obs_experimentation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregateLLMObsExperimentationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// BatchUpdateLLMObsDatasetError is a struct for typed errors of method [`LLMObservabilityAPI::batch_update_llm_obs_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchUpdateLLMObsDatasetError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CloneLLMObsDatasetError is a struct for typed errors of method [`LLMObservabilityAPI::clone_llm_obs_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneLLMObsDatasetError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsAnnotationQueueError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_annotation_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsAnnotationQueueError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsAnnotationQueueInteractionsError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_annotation_queue_interactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsAnnotationQueueInteractionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsDatasetError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsDatasetError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsDatasetRecordsError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_dataset_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsDatasetRecordsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsExperimentError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsExperimentEventsError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_experiment_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsExperimentEventsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsIntegrationInferenceError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_integration_inference`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsIntegrationInferenceError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLLMObsProjectError is a struct for typed errors of method [`LLMObservabilityAPI::create_llm_obs_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLLMObsProjectError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsAnnotationQueueError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_annotation_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsAnnotationQueueError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsAnnotationQueueInteractionsError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_annotation_queue_interactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsAnnotationQueueInteractionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsCustomEvalConfigError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_custom_eval_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsCustomEvalConfigError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsDataError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsDataError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsDatasetRecordsError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_dataset_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsDatasetRecordsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsDatasetsError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_datasets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsDatasetsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsExperimentsError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_experiments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsExperimentsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLLMObsProjectsError is a struct for typed errors of method [`LLMObservabilityAPI::delete_llm_obs_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLLMObsProjectsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ExportLLMObsDatasetError is a struct for typed errors of method [`LLMObservabilityAPI::export_llm_obs_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportLLMObsDatasetError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLLMObsAnnotatedInteractionsError is a struct for typed errors of method [`LLMObservabilityAPI::get_llm_obs_annotated_interactions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLLMObsAnnotatedInteractionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLLMObsAnnotatedInteractionsByTraceIDsError is a struct for typed errors of method [`LLMObservabilityAPI::get_llm_obs_annotated_interactions_by_trace_i_ds`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLLMObsAnnotatedInteractionsByTraceIDsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLLMObsAnnotationQueueLabelSchemaError is a struct for typed errors of method [`LLMObservabilityAPI::get_llm_obs_annotation_queue_label_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLLMObsAnnotationQueueLabelSchemaError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLLMObsCustomEvalConfigError is a struct for typed errors of method [`LLMObservabilityAPI::get_llm_obs_custom_eval_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLLMObsCustomEvalConfigError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLLMObsDatasetDraftStateError is a struct for typed errors of method [`LLMObservabilityAPI::get_llm_obs_dataset_draft_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLLMObsDatasetDraftStateError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsAnnotationQueuesError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_annotation_queues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsAnnotationQueuesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsDatasetRecordsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_dataset_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsDatasetRecordsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsDatasetVersionsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_dataset_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsDatasetVersionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsDatasetsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_datasets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsDatasetsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsExperimentEventsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_experiment_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsExperimentEventsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsExperimentEventsV1Error is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_experiment_events_v1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsExperimentEventsV1Error {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsExperimentEventsV2Error is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_experiment_events_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsExperimentEventsV2Error {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsExperimentsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_experiments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsExperimentsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsIntegrationAccountsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_integration_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsIntegrationAccountsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsIntegrationModelsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_integration_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsIntegrationModelsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsProjectsError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_projects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsProjectsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLLMObsSpansError is a struct for typed errors of method [`LLMObservabilityAPI::list_llm_obs_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLLMObsSpansError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// LockLLMObsDatasetDraftStateError is a struct for typed errors of method [`LLMObservabilityAPI::lock_llm_obs_dataset_draft_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LockLLMObsDatasetDraftStateError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RestoreLLMObsDatasetVersionError is a struct for typed errors of method [`LLMObservabilityAPI::restore_llm_obs_dataset_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreLLMObsDatasetVersionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchLLMObsExperimentationError is a struct for typed errors of method [`LLMObservabilityAPI::search_llm_obs_experimentation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchLLMObsExperimentationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchLLMObsSpansError is a struct for typed errors of method [`LLMObservabilityAPI::search_llm_obs_spans`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchLLMObsSpansError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SimpleSearchLLMObsExperimentationError is a struct for typed errors of method [`LLMObservabilityAPI::simple_search_llm_obs_experimentation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleSearchLLMObsExperimentationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnlockLLMObsDatasetDraftStateError is a struct for typed errors of method [`LLMObservabilityAPI::unlock_llm_obs_dataset_draft_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnlockLLMObsDatasetDraftStateError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsAnnotationQueueError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_annotation_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsAnnotationQueueError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsAnnotationQueueLabelSchemaError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_annotation_queue_label_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsAnnotationQueueLabelSchemaError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsCustomEvalConfigError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_custom_eval_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsCustomEvalConfigError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsDatasetError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsDatasetError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsDatasetRecordsError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_dataset_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsDatasetRecordsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsExperimentError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLLMObsProjectError is a struct for typed errors of method [`LLMObservabilityAPI::update_llm_obs_project`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLLMObsProjectError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UploadLLMObsDatasetRecordsFileError is a struct for typed errors of method [`LLMObservabilityAPI::upload_llm_obs_dataset_records_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadLLMObsDatasetRecordsFileError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage LLM Observability spans, data, projects, datasets, dataset records, experiments, and annotations.
#[derive(Debug, Clone)]
pub struct LLMObservabilityAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for LLMObservabilityAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl LLMObservabilityAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = reqwest::Client::builder();
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
                struct RetryableStatus;
                impl reqwest_retry::RetryableStrategy for RetryableStatus {
                    fn handle(
                        &self,
                        res: &Result<reqwest::Response, reqwest_middleware::Error>,
                    ) -> Option<reqwest_retry::Retryable> {
                        match res {
                            Ok(success) => reqwest_retry::default_on_request_success(success),
                            Err(_) => None,
                        }
                    }
                }
                let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                    .build_with_max_retries(config.max_retries);

                let retry_middleware =
                    reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                        backoff_policy,
                        RetryableStatus,
                    );

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Execute an analytics aggregation over LLM Observability experimentation data.
    /// Use this endpoint to compute metrics (for example average eval scores) grouped by fields such as `span_id` or `experiment_id`.
    ///
    /// At least one `compute` definition and one `index` must be provided.
    pub async fn aggregate_llm_obs_experimentation(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationAnalyticsRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentationAnalyticsResponse,
        datadog::Error<AggregateLLMObsExperimentationError>,
    > {
        match self
            .aggregate_llm_obs_experimentation_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Execute an analytics aggregation over LLM Observability experimentation data.
    /// Use this endpoint to compute metrics (for example average eval scores) grouped by fields such as `span_id` or `experiment_id`.
    ///
    /// At least one `compute` definition and one `index` must be provided.
    pub async fn aggregate_llm_obs_experimentation_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationAnalyticsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentationAnalyticsResponse>,
        datadog::Error<AggregateLLMObsExperimentationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.aggregate_llm_obs_experimentation";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.aggregate_llm_obs_experimentation' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experimentation/analytics",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsExperimentationAnalyticsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<AggregateLLMObsExperimentationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Insert, update, and delete records in a single dataset operation. By default, a new dataset version is created when the batch is applied.
    pub async fn batch_update_llm_obs_dataset(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetBatchUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
        datadog::Error<BatchUpdateLLMObsDatasetError>,
    > {
        match self
            .batch_update_llm_obs_dataset_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Insert, update, and delete records in a single dataset operation. By default, a new dataset version is created when the batch is applied.
    pub async fn batch_update_llm_obs_dataset_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetBatchUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse>,
        datadog::Error<BatchUpdateLLMObsDatasetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.batch_update_llm_obs_dataset";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.batch_update_llm_obs_dataset' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/batch_update",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<BatchUpdateLLMObsDatasetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Clone a dataset, copying its current records into a new dataset within the same project.
    pub async fn clone_llm_obs_dataset(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetCloneRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetResponse,
        datadog::Error<CloneLLMObsDatasetError>,
    > {
        match self
            .clone_llm_obs_dataset_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Clone a dataset, copying its current records into a new dataset within the same project.
    pub async fn clone_llm_obs_dataset_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetCloneRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetResponse>,
        datadog::Error<CloneLLMObsDatasetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.clone_llm_obs_dataset";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.clone_llm_obs_dataset' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/clone",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CloneLLMObsDatasetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create an annotation queue. The `name` and `project_id` fields are required.
    /// An optional `annotation_schema` can be provided to define the labels for the queue.
    /// Fields such as `created_by`, `owned_by`, `created_at`, `modified_by`,
    /// and `modified_at` are inferred by the backend.
    pub async fn create_llm_obs_annotation_queue(
        &self,
        body: crate::datadogV2::model::LLMObsAnnotationQueueRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueueResponse,
        datadog::Error<CreateLLMObsAnnotationQueueError>,
    > {
        match self
            .create_llm_obs_annotation_queue_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create an annotation queue. The `name` and `project_id` fields are required.
    /// An optional `annotation_schema` can be provided to define the labels for the queue.
    /// Fields such as `created_by`, `owned_by`, `created_at`, `modified_by`,
    /// and `modified_at` are inferred by the backend.
    pub async fn create_llm_obs_annotation_queue_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsAnnotationQueueRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotationQueueResponse>,
        datadog::Error<CreateLLMObsAnnotationQueueError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_annotation_queue";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_annotation_queue' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsAnnotationQueueResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsAnnotationQueueError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Add one or more interactions to an annotation queue. At least one
    /// interaction must be provided. Each interaction has a `type`:
    ///
    /// - `trace`, `experiment_trace`, `session`: `content_id` references the
    ///   upstream entity; the server fetches the actual content.
    /// - `display_block`: omit `content_id` and provide the rendered content
    ///   in `display_block`. The server generates `content_id` as a
    ///   deterministic hash of the block list.
    ///
    /// Items of different types can be mixed in a single request.
    pub async fn create_llm_obs_annotation_queue_interactions(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueInteractionsRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueueInteractionsResponse,
        datadog::Error<CreateLLMObsAnnotationQueueInteractionsError>,
    > {
        match self
            .create_llm_obs_annotation_queue_interactions_with_http_info(queue_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Add one or more interactions to an annotation queue. At least one
    /// interaction must be provided. Each interaction has a `type`:
    ///
    /// - `trace`, `experiment_trace`, `session`: `content_id` references the
    ///   upstream entity; the server fetches the actual content.
    /// - `display_block`: omit `content_id` and provide the rendered content
    ///   in `display_block`. The server generates `content_id` as a
    ///   deterministic hash of the block list.
    ///
    /// Items of different types can be mixed in a single request.
    pub async fn create_llm_obs_annotation_queue_interactions_with_http_info(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueInteractionsRequest,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::LLMObsAnnotationQueueInteractionsResponse,
        >,
        datadog::Error<CreateLLMObsAnnotationQueueInteractionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_annotation_queue_interactions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_annotation_queue_interactions' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}/interactions",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsAnnotationQueueInteractionsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsAnnotationQueueInteractionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new LLM Observability dataset within the specified project.
    pub async fn create_llm_obs_dataset(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetResponse,
        datadog::Error<CreateLLMObsDatasetError>,
    > {
        match self
            .create_llm_obs_dataset_with_http_info(project_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new LLM Observability dataset within the specified project.
    pub async fn create_llm_obs_dataset_with_http_info(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetResponse>,
        datadog::Error<CreateLLMObsDatasetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_dataset";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_dataset' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsDatasetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Append one or more records to an LLM Observability dataset.
    pub async fn create_llm_obs_dataset_records(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRecordsRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
        datadog::Error<CreateLLMObsDatasetRecordsError>,
    > {
        match self
            .create_llm_obs_dataset_records_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Append one or more records to an LLM Observability dataset.
    pub async fn create_llm_obs_dataset_records_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRecordsRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse>,
        datadog::Error<CreateLLMObsDatasetRecordsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_dataset_records";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_dataset_records' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/records",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsDatasetRecordsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new LLM Observability experiment.
    pub async fn create_llm_obs_experiment(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentResponse,
        datadog::Error<CreateLLMObsExperimentError>,
    > {
        match self.create_llm_obs_experiment_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new LLM Observability experiment.
    pub async fn create_llm_obs_experiment_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentResponse>,
        datadog::Error<CreateLLMObsExperimentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_experiment";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_experiment' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Push spans and metrics for an LLM Observability experiment.
    pub async fn create_llm_obs_experiment_events(
        &self,
        experiment_id: String,
        body: crate::datadogV2::model::LLMObsExperimentEventsRequest,
    ) -> Result<(), datadog::Error<CreateLLMObsExperimentEventsError>> {
        match self
            .create_llm_obs_experiment_events_with_http_info(experiment_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Push spans and metrics for an LLM Observability experiment.
    pub async fn create_llm_obs_experiment_events_with_http_info(
        &self,
        experiment_id: String,
        body: crate::datadogV2::model::LLMObsExperimentEventsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<CreateLLMObsExperimentEventsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_experiment_events";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_experiment_events' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments/{experiment_id}/events",
            local_configuration.get_operation_host(operation_id),
            experiment_id = datadog::urlencode(experiment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<CreateLLMObsExperimentEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Run an LLM inference request through the specified integration and account, returning the model response and token usage.
    pub async fn create_llm_obs_integration_inference(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
        account_id: String,
        body: crate::datadogV2::model::LLMObsIntegrationInferenceRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsIntegrationInferenceResponse,
        datadog::Error<CreateLLMObsIntegrationInferenceError>,
    > {
        match self
            .create_llm_obs_integration_inference_with_http_info(integration, account_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Run an LLM inference request through the specified integration and account, returning the model response and token usage.
    pub async fn create_llm_obs_integration_inference_with_http_info(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
        account_id: String,
        body: crate::datadogV2::model::LLMObsIntegrationInferenceRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsIntegrationInferenceResponse>,
        datadog::Error<CreateLLMObsIntegrationInferenceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_integration_inference";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_integration_inference' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/integrations/{integration}/{account_id}/inference",
            local_configuration.get_operation_host(operation_id),
            integration = datadog::urlencode(integration.to_string()),
            account_id = datadog::urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsIntegrationInferenceResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsIntegrationInferenceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a new LLM Observability project. Returns the existing project if a name conflict occurs.
    pub async fn create_llm_obs_project(
        &self,
        body: crate::datadogV2::model::LLMObsProjectRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsProjectResponse,
        datadog::Error<CreateLLMObsProjectError>,
    > {
        match self.create_llm_obs_project_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new LLM Observability project. Returns the existing project if a name conflict occurs.
    pub async fn create_llm_obs_project_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsProjectRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsProjectResponse>,
        datadog::Error<CreateLLMObsProjectError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_llm_obs_project";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_llm_obs_project' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/projects",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsProjectResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateLLMObsProjectError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an annotation queue by its ID.
    pub async fn delete_llm_obs_annotation_queue(
        &self,
        queue_id: String,
    ) -> Result<(), datadog::Error<DeleteLLMObsAnnotationQueueError>> {
        match self
            .delete_llm_obs_annotation_queue_with_http_info(queue_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an annotation queue by its ID.
    pub async fn delete_llm_obs_annotation_queue_with_http_info(
        &self,
        queue_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsAnnotationQueueError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_annotation_queue";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_annotation_queue' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsAnnotationQueueError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete one or more interactions from an annotation queue.
    pub async fn delete_llm_obs_annotation_queue_interactions(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsDeleteAnnotationQueueInteractionsRequest,
    ) -> Result<(), datadog::Error<DeleteLLMObsAnnotationQueueInteractionsError>> {
        match self
            .delete_llm_obs_annotation_queue_interactions_with_http_info(queue_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete one or more interactions from an annotation queue.
    pub async fn delete_llm_obs_annotation_queue_interactions_with_http_info(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsDeleteAnnotationQueueInteractionsRequest,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteLLMObsAnnotationQueueInteractionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_annotation_queue_interactions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_annotation_queue_interactions' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}/interactions/delete",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsAnnotationQueueInteractionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a custom LLM Observability evaluator configuration by its name.
    pub async fn delete_llm_obs_custom_eval_config(
        &self,
        eval_name: String,
    ) -> Result<(), datadog::Error<DeleteLLMObsCustomEvalConfigError>> {
        match self
            .delete_llm_obs_custom_eval_config_with_http_info(eval_name)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a custom LLM Observability evaluator configuration by its name.
    pub async fn delete_llm_obs_custom_eval_config_with_http_info(
        &self,
        eval_name: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsCustomEvalConfigError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_custom_eval_config";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_custom_eval_config' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/llm-obs/config/evaluators/custom/{eval_name}",
            local_configuration.get_operation_host(operation_id),
            eval_name = datadog::urlencode(eval_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsCustomEvalConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Submit a request to delete LLM Observability span data matching a trace ID filter within a specified time range.
    pub async fn delete_llm_obs_data(
        &self,
        body: crate::datadogV2::model::LLMObsDataDeletionRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDataDeletionResponse,
        datadog::Error<DeleteLLMObsDataError>,
    > {
        match self.delete_llm_obs_data_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Submit a request to delete LLM Observability span data matching a trace ID filter within a specified time range.
    pub async fn delete_llm_obs_data_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsDataDeletionRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDataDeletionResponse>,
        datadog::Error<DeleteLLMObsDataError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_data";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_data' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/deletion/data/llmobs",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDataDeletionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<DeleteLLMObsDataError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete one or more records from an LLM Observability dataset.
    pub async fn delete_llm_obs_dataset_records(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDeleteDatasetRecordsRequest,
    ) -> Result<(), datadog::Error<DeleteLLMObsDatasetRecordsError>> {
        match self
            .delete_llm_obs_dataset_records_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete one or more records from an LLM Observability dataset.
    pub async fn delete_llm_obs_dataset_records_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDeleteDatasetRecordsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsDatasetRecordsError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_dataset_records";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_dataset_records' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/records/delete",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsDatasetRecordsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete one or more LLM Observability datasets within the specified project.
    pub async fn delete_llm_obs_datasets(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsDeleteDatasetsRequest,
    ) -> Result<(), datadog::Error<DeleteLLMObsDatasetsError>> {
        match self
            .delete_llm_obs_datasets_with_http_info(project_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete one or more LLM Observability datasets within the specified project.
    pub async fn delete_llm_obs_datasets_with_http_info(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsDeleteDatasetsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsDatasetsError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_datasets";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_datasets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/delete",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsDatasetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete one or more LLM Observability experiments.
    pub async fn delete_llm_obs_experiments(
        &self,
        body: crate::datadogV2::model::LLMObsDeleteExperimentsRequest,
    ) -> Result<(), datadog::Error<DeleteLLMObsExperimentsError>> {
        match self.delete_llm_obs_experiments_with_http_info(body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete one or more LLM Observability experiments.
    pub async fn delete_llm_obs_experiments_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsDeleteExperimentsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsExperimentsError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_experiments";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_experiments' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments/delete",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsExperimentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete one or more LLM Observability projects.
    pub async fn delete_llm_obs_projects(
        &self,
        body: crate::datadogV2::model::LLMObsDeleteProjectsRequest,
    ) -> Result<(), datadog::Error<DeleteLLMObsProjectsError>> {
        match self.delete_llm_obs_projects_with_http_info(body).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete one or more LLM Observability projects.
    pub async fn delete_llm_obs_projects_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsDeleteProjectsRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteLLMObsProjectsError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_llm_obs_projects";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_llm_obs_projects' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/projects/delete",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteLLMObsProjectsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Download the contents of a dataset as a CSV file. The download is streamed and includes one row per dataset record.
    pub async fn export_llm_obs_dataset(
        &self,
        project_id: String,
        dataset_id: String,
        params: ExportLLMObsDatasetOptionalParams,
    ) -> Result<String, datadog::Error<ExportLLMObsDatasetError>> {
        match self
            .export_llm_obs_dataset_with_http_info(project_id, dataset_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Download the contents of a dataset as a CSV file. The download is streamed and includes one row per dataset record.
    pub async fn export_llm_obs_dataset_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        params: ExportLLMObsDatasetOptionalParams,
    ) -> Result<datadog::ResponseContent<String>, datadog::Error<ExportLLMObsDatasetError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.export_llm_obs_dataset";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.export_llm_obs_dataset' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let format = params.format;
        let version = params.version;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/export",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = format {
            local_req_builder =
                local_req_builder.query(&[("format", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = version {
            local_req_builder =
                local_req_builder.query(&[("version", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<String>(&local_content) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ExportLLMObsDatasetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve all interactions (traces and sessions) and their annotations for a given annotation queue.
    pub async fn get_llm_obs_annotated_interactions(
        &self,
        queue_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotatedInteractionsResponse,
        datadog::Error<GetLLMObsAnnotatedInteractionsError>,
    > {
        match self
            .get_llm_obs_annotated_interactions_with_http_info(queue_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve all interactions (traces and sessions) and their annotations for a given annotation queue.
    pub async fn get_llm_obs_annotated_interactions_with_http_info(
        &self,
        queue_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotatedInteractionsResponse>,
        datadog::Error<GetLLMObsAnnotatedInteractionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_llm_obs_annotated_interactions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_llm_obs_annotated_interactions' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}/annotated-interactions",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsAnnotatedInteractionsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetLLMObsAnnotatedInteractionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns annotated interactions across all annotation queues for the given content IDs. Results include queue metadata (ID and name) for each interaction.
    pub async fn get_llm_obs_annotated_interactions_by_trace_i_ds(
        &self,
        content_ids: Vec<String>,
        params: GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotatedInteractionsByTraceResponse,
        datadog::Error<GetLLMObsAnnotatedInteractionsByTraceIDsError>,
    > {
        match self
            .get_llm_obs_annotated_interactions_by_trace_i_ds_with_http_info(content_ids, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Returns annotated interactions across all annotation queues for the given content IDs. Results include queue metadata (ID and name) for each interaction.
    pub async fn get_llm_obs_annotated_interactions_by_trace_i_ds_with_http_info(
        &self,
        content_ids: Vec<String>,
        params: GetLLMObsAnnotatedInteractionsByTraceIDsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::LLMObsAnnotatedInteractionsByTraceResponse,
        >,
        datadog::Error<GetLLMObsAnnotatedInteractionsByTraceIDsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_llm_obs_annotated_interactions_by_trace_i_ds";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg:
                    "Operation 'v2.get_llm_obs_annotated_interactions_by_trace_i_ds' is not enabled"
                        .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotated-interactions",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "contentIds",
            &content_ids
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]);
        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsAnnotatedInteractionsByTraceResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetLLMObsAnnotatedInteractionsByTraceIDsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the label schema for a given annotation queue.
    pub async fn get_llm_obs_annotation_queue_label_schema(
        &self,
        queue_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse,
        datadog::Error<GetLLMObsAnnotationQueueLabelSchemaError>,
    > {
        match self
            .get_llm_obs_annotation_queue_label_schema_with_http_info(queue_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve the label schema for a given annotation queue.
    pub async fn get_llm_obs_annotation_queue_label_schema_with_http_info(
        &self,
        queue_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse>,
        datadog::Error<GetLLMObsAnnotationQueueLabelSchemaError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_llm_obs_annotation_queue_label_schema";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_llm_obs_annotation_queue_label_schema' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}/label-schema",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetLLMObsAnnotationQueueLabelSchemaError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a custom LLM Observability evaluator configuration by its name.
    pub async fn get_llm_obs_custom_eval_config(
        &self,
        eval_name: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsCustomEvalConfigResponse,
        datadog::Error<GetLLMObsCustomEvalConfigError>,
    > {
        match self
            .get_llm_obs_custom_eval_config_with_http_info(eval_name)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve a custom LLM Observability evaluator configuration by its name.
    pub async fn get_llm_obs_custom_eval_config_with_http_info(
        &self,
        eval_name: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsCustomEvalConfigResponse>,
        datadog::Error<GetLLMObsCustomEvalConfigError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_llm_obs_custom_eval_config";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_llm_obs_custom_eval_config' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/llm-obs/config/evaluators/custom/{eval_name}",
            local_configuration.get_operation_host(operation_id),
            eval_name = datadog::urlencode(eval_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsCustomEvalConfigResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetLLMObsCustomEvalConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the draft state of a dataset, including whether it is currently locked for editing and which user holds the lock.
    pub async fn get_llm_obs_dataset_draft_state(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetDraftStateResponse,
        datadog::Error<GetLLMObsDatasetDraftStateError>,
    > {
        match self
            .get_llm_obs_dataset_draft_state_with_http_info(project_id, dataset_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve the draft state of a dataset, including whether it is currently locked for editing and which user holds the lock.
    pub async fn get_llm_obs_dataset_draft_state_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetDraftStateResponse>,
        datadog::Error<GetLLMObsDatasetDraftStateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_llm_obs_dataset_draft_state";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_llm_obs_dataset_draft_state' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/draft_state",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetDraftStateResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetLLMObsDatasetDraftStateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List annotation queues. Optionally filter by project ID or queue IDs. These parameters are mutually exclusive.
    /// If neither is provided, all queues in the organization are returned.
    pub async fn list_llm_obs_annotation_queues(
        &self,
        params: ListLLMObsAnnotationQueuesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueuesResponse,
        datadog::Error<ListLLMObsAnnotationQueuesError>,
    > {
        match self
            .list_llm_obs_annotation_queues_with_http_info(params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List annotation queues. Optionally filter by project ID or queue IDs. These parameters are mutually exclusive.
    /// If neither is provided, all queues in the organization are returned.
    pub async fn list_llm_obs_annotation_queues_with_http_info(
        &self,
        params: ListLLMObsAnnotationQueuesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotationQueuesResponse>,
        datadog::Error<ListLLMObsAnnotationQueuesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_annotation_queues";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_annotation_queues' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let project_id = params.project_id;
        let queue_ids = params.queue_ids;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = project_id {
            local_req_builder =
                local_req_builder.query(&[("projectId", &local_query_param.to_string())]);
        };
        if let Some(ref local) = queue_ids {
            for param in local {
                local_req_builder = local_req_builder.query(&[("queueIds", &param.to_string())]);
            }
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsAnnotationQueuesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsAnnotationQueuesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all records in an LLM Observability dataset, sorted by creation date, newest first.
    pub async fn list_llm_obs_dataset_records(
        &self,
        project_id: String,
        dataset_id: String,
        params: ListLLMObsDatasetRecordsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetRecordsListResponse,
        datadog::Error<ListLLMObsDatasetRecordsError>,
    > {
        match self
            .list_llm_obs_dataset_records_with_http_info(project_id, dataset_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all records in an LLM Observability dataset, sorted by creation date, newest first.
    pub async fn list_llm_obs_dataset_records_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        params: ListLLMObsDatasetRecordsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetRecordsListResponse>,
        datadog::Error<ListLLMObsDatasetRecordsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_dataset_records";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_dataset_records' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_version = params.filter_version;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/records",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_version {
            local_req_builder =
                local_req_builder.query(&[("filter[version]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetRecordsListResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsDatasetRecordsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the active versions of a dataset. A version is created each time a dataset is referenced by an experiment run.
    pub async fn list_llm_obs_dataset_versions(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetVersionsResponse,
        datadog::Error<ListLLMObsDatasetVersionsError>,
    > {
        match self
            .list_llm_obs_dataset_versions_with_http_info(project_id, dataset_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List the active versions of a dataset. A version is created each time a dataset is referenced by an experiment run.
    pub async fn list_llm_obs_dataset_versions_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetVersionsResponse>,
        datadog::Error<ListLLMObsDatasetVersionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_dataset_versions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_dataset_versions' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/versions",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetVersionsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsDatasetVersionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all LLM Observability datasets for a project, sorted by creation date, newest first.
    pub async fn list_llm_obs_datasets(
        &self,
        project_id: String,
        params: ListLLMObsDatasetsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetsResponse,
        datadog::Error<ListLLMObsDatasetsError>,
    > {
        match self
            .list_llm_obs_datasets_with_http_info(project_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all LLM Observability datasets for a project, sorted by creation date, newest first.
    pub async fn list_llm_obs_datasets_with_http_info(
        &self,
        project_id: String,
        params: ListLLMObsDatasetsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetsResponse>,
        datadog::Error<ListLLMObsDatasetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_datasets";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_datasets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_name = params.filter_name;
        let filter_id = params.filter_id;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_id {
            local_req_builder =
                local_req_builder.query(&[("filter[id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsDatasetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve spans and experiment-level summary metrics for a given experiment with cursor-based pagination.
    pub async fn list_llm_obs_experiment_events(
        &self,
        experiment_id: String,
        params: ListLLMObsExperimentEventsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentEventsV2Response,
        datadog::Error<ListLLMObsExperimentEventsError>,
    > {
        match self
            .list_llm_obs_experiment_events_with_http_info(experiment_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve spans and experiment-level summary metrics for a given experiment with cursor-based pagination.
    pub async fn list_llm_obs_experiment_events_with_http_info(
        &self,
        experiment_id: String,
        params: ListLLMObsExperimentEventsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentEventsV2Response>,
        datadog::Error<ListLLMObsExperimentEventsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_experiment_events";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_experiment_events' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_limit = params.page_limit;
        let page_cursor = params.page_cursor;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v3/experiments/{experiment_id}/events",
            local_configuration.get_operation_host(operation_id),
            experiment_id = datadog::urlencode(experiment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentEventsV2Response>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsExperimentEventsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve spans with their evaluation metrics for a given experiment. Returns spans only, with no summary metrics and no pagination. Deprecated in favor of `ListLLMObsExperimentEventsV3`.
    pub async fn list_llm_obs_experiment_events_v1(
        &self,
        experiment_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentSpansResponse,
        datadog::Error<ListLLMObsExperimentEventsV1Error>,
    > {
        match self
            .list_llm_obs_experiment_events_v1_with_http_info(experiment_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve spans with their evaluation metrics for a given experiment. Returns spans only, with no summary metrics and no pagination. Deprecated in favor of `ListLLMObsExperimentEventsV3`.
    pub async fn list_llm_obs_experiment_events_v1_with_http_info(
        &self,
        experiment_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentSpansResponse>,
        datadog::Error<ListLLMObsExperimentEventsV1Error>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_experiment_events_v1";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_experiment_events_v1' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments/{experiment_id}/events",
            local_configuration.get_operation_host(operation_id),
            experiment_id = datadog::urlencode(experiment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentSpansResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsExperimentEventsV1Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve spans and experiment-level summary metrics for a given experiment. Returns the full events payload without pagination. Deprecated: use `ListLLMObsExperimentEventsV3` instead.
    pub async fn list_llm_obs_experiment_events_v2(
        &self,
        experiment_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentEventsV2Response,
        datadog::Error<ListLLMObsExperimentEventsV2Error>,
    > {
        match self
            .list_llm_obs_experiment_events_v2_with_http_info(experiment_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve spans and experiment-level summary metrics for a given experiment. Returns the full events payload without pagination. Deprecated: use `ListLLMObsExperimentEventsV3` instead.
    pub async fn list_llm_obs_experiment_events_v2_with_http_info(
        &self,
        experiment_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentEventsV2Response>,
        datadog::Error<ListLLMObsExperimentEventsV2Error>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_experiment_events_v2";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_experiment_events_v2' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v2/experiments/{experiment_id}/events",
            local_configuration.get_operation_host(operation_id),
            experiment_id = datadog::urlencode(experiment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentEventsV2Response>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsExperimentEventsV2Error> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all LLM Observability experiments sorted by creation date, newest first.
    pub async fn list_llm_obs_experiments(
        &self,
        params: ListLLMObsExperimentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentsResponse,
        datadog::Error<ListLLMObsExperimentsError>,
    > {
        match self.list_llm_obs_experiments_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all LLM Observability experiments sorted by creation date, newest first.
    pub async fn list_llm_obs_experiments_with_http_info(
        &self,
        params: ListLLMObsExperimentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentsResponse>,
        datadog::Error<ListLLMObsExperimentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_experiments";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_experiments' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_project_id = params.filter_project_id;
        let filter_dataset_id = params.filter_dataset_id;
        let filter_id = params.filter_id;
        let filter_name = params.filter_name;
        let filter_experiment = params.filter_experiment;
        let filter_metadata = params.filter_metadata;
        let filter_parent_experiment_id = params.filter_parent_experiment_id;
        let filter_is_deleted = params.filter_is_deleted;
        let include_user_data = params.include_user_data;
        let include_dataset_names = params.include_dataset_names;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_project_id {
            local_req_builder =
                local_req_builder.query(&[("filter[project_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_dataset_id {
            local_req_builder =
                local_req_builder.query(&[("filter[dataset_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_id {
            local_req_builder =
                local_req_builder.query(&[("filter[id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_experiment {
            local_req_builder =
                local_req_builder.query(&[("filter[experiment]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_metadata {
            local_req_builder =
                local_req_builder.query(&[("filter[metadata]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_parent_experiment_id {
            local_req_builder = local_req_builder.query(&[(
                "filter[parent_experiment_id]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_is_deleted {
            local_req_builder =
                local_req_builder.query(&[("filter[is_deleted]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_user_data {
            local_req_builder =
                local_req_builder.query(&[("include[user_data]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_dataset_names {
            local_req_builder = local_req_builder
                .query(&[("include[dataset_names]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsExperimentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the list of configured accounts for the specified LLM provider integration.
    pub async fn list_llm_obs_integration_accounts(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
    ) -> Result<
        Vec<crate::datadogV2::model::LLMObsIntegrationAccount>,
        datadog::Error<ListLLMObsIntegrationAccountsError>,
    > {
        match self
            .list_llm_obs_integration_accounts_with_http_info(integration)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve the list of configured accounts for the specified LLM provider integration.
    pub async fn list_llm_obs_integration_accounts_with_http_info(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
    ) -> Result<
        datadog::ResponseContent<Vec<crate::datadogV2::model::LLMObsIntegrationAccount>>,
        datadog::Error<ListLLMObsIntegrationAccountsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_integration_accounts";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_integration_accounts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/integrations/{integration}/accounts",
            local_configuration.get_operation_host(operation_id),
            integration = datadog::urlencode(integration.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<Vec<crate::datadogV2::model::LLMObsIntegrationAccount>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsIntegrationAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve the list of models available for the specified LLM provider integration and account.
    pub async fn list_llm_obs_integration_models(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
        account_id: String,
    ) -> Result<
        Vec<crate::datadogV2::model::LLMObsIntegrationModel>,
        datadog::Error<ListLLMObsIntegrationModelsError>,
    > {
        match self
            .list_llm_obs_integration_models_with_http_info(integration, account_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Retrieve the list of models available for the specified LLM provider integration and account.
    pub async fn list_llm_obs_integration_models_with_http_info(
        &self,
        integration: crate::datadogV2::model::LLMObsIntegrationName,
        account_id: String,
    ) -> Result<
        datadog::ResponseContent<Vec<crate::datadogV2::model::LLMObsIntegrationModel>>,
        datadog::Error<ListLLMObsIntegrationModelsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_integration_models";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_integration_models' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/integrations/{integration}/{account_id}/models",
            local_configuration.get_operation_host(operation_id),
            integration = datadog::urlencode(integration.to_string()),
            account_id = datadog::urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<Vec<crate::datadogV2::model::LLMObsIntegrationModel>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsIntegrationModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all LLM Observability projects sorted by creation date, newest first.
    pub async fn list_llm_obs_projects(
        &self,
        params: ListLLMObsProjectsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::LLMObsProjectsResponse,
        datadog::Error<ListLLMObsProjectsError>,
    > {
        match self.list_llm_obs_projects_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List all LLM Observability projects sorted by creation date, newest first.
    pub async fn list_llm_obs_projects_with_http_info(
        &self,
        params: ListLLMObsProjectsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsProjectsResponse>,
        datadog::Error<ListLLMObsProjectsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_projects";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_projects' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_id = params.filter_id;
        let filter_name = params.filter_name;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/projects",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_id {
            local_req_builder =
                local_req_builder.query(&[("filter[id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsProjectsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsProjectsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List LLM Observability spans matching the specified filters.
    pub async fn list_llm_obs_spans(
        &self,
        params: ListLLMObsSpansOptionalParams,
    ) -> Result<crate::datadogV2::model::LLMObsSpansResponse, datadog::Error<ListLLMObsSpansError>>
    {
        match self.list_llm_obs_spans_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List LLM Observability spans matching the specified filters.
    pub async fn list_llm_obs_spans_with_http_info(
        &self,
        params: ListLLMObsSpansOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsSpansResponse>,
        datadog::Error<ListLLMObsSpansError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_llm_obs_spans";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_llm_obs_spans' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let filter_query = params.filter_query;
        let filter_span_id = params.filter_span_id;
        let filter_trace_id = params.filter_trace_id;
        let filter_span_kind = params.filter_span_kind;
        let filter_span_name = params.filter_span_name;
        let filter_ml_app = params.filter_ml_app;
        let page_limit = params.page_limit;
        let page_cursor = params.page_cursor;
        let sort = params.sort;
        let include_attachments = params.include_attachments;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/spans/events",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_span_id {
            local_req_builder =
                local_req_builder.query(&[("filter[span_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_trace_id {
            local_req_builder =
                local_req_builder.query(&[("filter[trace_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_span_kind {
            local_req_builder =
                local_req_builder.query(&[("filter[span_kind]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_span_name {
            local_req_builder =
                local_req_builder.query(&[("filter[span_name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_ml_app {
            local_req_builder =
                local_req_builder.query(&[("filter[ml_app]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_attachments {
            local_req_builder =
                local_req_builder.query(&[("include_attachments", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsSpansResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLLMObsSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Acquire the draft lock on a dataset for the calling user. The lock prevents other users from concurrently editing the dataset draft.
    pub async fn lock_llm_obs_dataset_draft_state(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetDraftStateResponse,
        datadog::Error<LockLLMObsDatasetDraftStateError>,
    > {
        match self
            .lock_llm_obs_dataset_draft_state_with_http_info(project_id, dataset_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Acquire the draft lock on a dataset for the calling user. The lock prevents other users from concurrently editing the dataset draft.
    pub async fn lock_llm_obs_dataset_draft_state_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetDraftStateResponse>,
        datadog::Error<LockLLMObsDatasetDraftStateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.lock_llm_obs_dataset_draft_state";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.lock_llm_obs_dataset_draft_state' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/draft_state/lock",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetDraftStateResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<LockLLMObsDatasetDraftStateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Restore a dataset to a previous version. The dataset's current version is bumped, and its records are replaced with the records from the specified prior version.
    pub async fn restore_llm_obs_dataset_version(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRestoreVersionRequest,
    ) -> Result<(), datadog::Error<RestoreLLMObsDatasetVersionError>> {
        match self
            .restore_llm_obs_dataset_version_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Restore a dataset to a previous version. The dataset's current version is bumped, and its records are replaced with the records from the specified prior version.
    pub async fn restore_llm_obs_dataset_version_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRestoreVersionRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<RestoreLLMObsDatasetVersionError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.restore_llm_obs_dataset_version";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.restore_llm_obs_dataset_version' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/restore",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<RestoreLLMObsDatasetVersionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search across LLM Observability experimentation entities — projects, datasets, dataset records, experiments, and experiment runs — using cursor-based pagination.
    ///
    /// The `filter.scope` field controls which entity types are returned. At least one valid scope must be provided.
    ///
    /// Returns `200 OK` when all results fit in a single page. Returns `206 Partial Content` with a cursor in `meta.after` when additional pages are available.
    pub async fn search_llm_obs_experimentation(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationSearchRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentationSearchResponse,
        datadog::Error<SearchLLMObsExperimentationError>,
    > {
        match self
            .search_llm_obs_experimentation_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Search across LLM Observability experimentation entities — projects, datasets, dataset records, experiments, and experiment runs — using cursor-based pagination.
    ///
    /// The `filter.scope` field controls which entity types are returned. At least one valid scope must be provided.
    ///
    /// Returns `200 OK` when all results fit in a single page. Returns `206 Partial Content` with a cursor in `meta.after` when additional pages are available.
    pub async fn search_llm_obs_experimentation_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationSearchRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentationSearchResponse>,
        datadog::Error<SearchLLMObsExperimentationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_llm_obs_experimentation";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.search_llm_obs_experimentation' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experimentation/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentationSearchResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SearchLLMObsExperimentationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search LLM Observability spans using structured filters in the request body.
    pub async fn search_llm_obs_spans(
        &self,
        body: crate::datadogV2::model::LLMObsSearchSpansRequest,
    ) -> Result<crate::datadogV2::model::LLMObsSpansResponse, datadog::Error<SearchLLMObsSpansError>>
    {
        match self.search_llm_obs_spans_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Search LLM Observability spans using structured filters in the request body.
    pub async fn search_llm_obs_spans_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsSearchSpansRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsSpansResponse>,
        datadog::Error<SearchLLMObsSpansError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_llm_obs_spans";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.search_llm_obs_spans' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/spans/events/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsSpansResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SearchLLMObsSpansError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Search across LLM Observability experimentation entities using offset-based (page-number) pagination.
    /// Use this endpoint when you need total page count or want to navigate to a specific page number.
    ///
    /// The `filter.scope` field controls which entity types are returned. At least one valid scope must be provided.
    pub async fn simple_search_llm_obs_experimentation(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationSimpleSearchRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentationSimpleSearchResponse,
        datadog::Error<SimpleSearchLLMObsExperimentationError>,
    > {
        match self
            .simple_search_llm_obs_experimentation_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Search across LLM Observability experimentation entities using offset-based (page-number) pagination.
    /// Use this endpoint when you need total page count or want to navigate to a specific page number.
    ///
    /// The `filter.scope` field controls which entity types are returned. At least one valid scope must be provided.
    pub async fn simple_search_llm_obs_experimentation_with_http_info(
        &self,
        body: crate::datadogV2::model::LLMObsExperimentationSimpleSearchRequest,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::LLMObsExperimentationSimpleSearchResponse,
        >,
        datadog::Error<SimpleSearchLLMObsExperimentationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.simple_search_llm_obs_experimentation";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.simple_search_llm_obs_experimentation' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experimentation/simple-search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsExperimentationSimpleSearchResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SimpleSearchLLMObsExperimentationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Release the draft lock on a dataset held by the calling user, allowing other users to edit the dataset draft.
    pub async fn unlock_llm_obs_dataset_draft_state(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<(), datadog::Error<UnlockLLMObsDatasetDraftStateError>> {
        match self
            .unlock_llm_obs_dataset_draft_state_with_http_info(project_id, dataset_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Release the draft lock on a dataset held by the calling user, allowing other users to edit the dataset draft.
    pub async fn unlock_llm_obs_dataset_draft_state_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UnlockLLMObsDatasetDraftStateError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.unlock_llm_obs_dataset_draft_state";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.unlock_llm_obs_dataset_draft_state' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/draft_state/unlock",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UnlockLLMObsDatasetDraftStateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update an annotation queue. The `name`, `description`, and `annotation_schema` fields can be updated.
    pub async fn update_llm_obs_annotation_queue(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueueResponse,
        datadog::Error<UpdateLLMObsAnnotationQueueError>,
    > {
        match self
            .update_llm_obs_annotation_queue_with_http_info(queue_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update an annotation queue. The `name`, `description`, and `annotation_schema` fields can be updated.
    pub async fn update_llm_obs_annotation_queue_with_http_info(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotationQueueResponse>,
        datadog::Error<UpdateLLMObsAnnotationQueueError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_annotation_queue";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_annotation_queue' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsAnnotationQueueResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsAnnotationQueueError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or replace the label schema for a given annotation queue.
    /// The label schema defines the labels annotators can apply to interactions in the queue.
    /// Label names must be unique within the queue and match the pattern `^[a-zA-Z0-9_-]+$`.
    /// Each label must have a valid type: score, categorical, boolean, or text.
    pub async fn update_llm_obs_annotation_queue_label_schema(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse,
        datadog::Error<UpdateLLMObsAnnotationQueueLabelSchemaError>,
    > {
        match self
            .update_llm_obs_annotation_queue_label_schema_with_http_info(queue_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create or replace the label schema for a given annotation queue.
    /// The label schema defines the labels annotators can apply to interactions in the queue.
    /// Label names must be unique within the queue and match the pattern `^[a-zA-Z0-9_-]+$`.
    /// Each label must have a valid type: score, categorical, boolean, or text.
    pub async fn update_llm_obs_annotation_queue_label_schema_with_http_info(
        &self,
        queue_id: String,
        body: crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse>,
        datadog::Error<UpdateLLMObsAnnotationQueueLabelSchemaError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_annotation_queue_label_schema";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_annotation_queue_label_schema' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/annotation-queues/{queue_id}/label-schema",
            local_configuration.get_operation_host(operation_id),
            queue_id = datadog::urlencode(queue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsAnnotationQueueLabelSchemaResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsAnnotationQueueLabelSchemaError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or update a custom LLM Observability evaluator configuration by its name.
    pub async fn update_llm_obs_custom_eval_config(
        &self,
        eval_name: String,
        body: crate::datadogV2::model::LLMObsCustomEvalConfigUpdateRequest,
    ) -> Result<(), datadog::Error<UpdateLLMObsCustomEvalConfigError>> {
        match self
            .update_llm_obs_custom_eval_config_with_http_info(eval_name, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Create or update a custom LLM Observability evaluator configuration by its name.
    pub async fn update_llm_obs_custom_eval_config_with_http_info(
        &self,
        eval_name: String,
        body: crate::datadogV2::model::LLMObsCustomEvalConfigUpdateRequest,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UpdateLLMObsCustomEvalConfigError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_custom_eval_config";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_custom_eval_config' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/unstable/llm-obs/config/evaluators/custom/{eval_name}",
            local_configuration.get_operation_host(operation_id),
            eval_name = datadog::urlencode(eval_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdateLLMObsCustomEvalConfigError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update an existing LLM Observability dataset within the specified project.
    pub async fn update_llm_obs_dataset(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetResponse,
        datadog::Error<UpdateLLMObsDatasetError>,
    > {
        match self
            .update_llm_obs_dataset_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update an existing LLM Observability dataset within the specified project.
    pub async fn update_llm_obs_dataset_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetResponse>,
        datadog::Error<UpdateLLMObsDatasetError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_dataset";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_dataset' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsDatasetResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsDatasetError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update one or more existing records in an LLM Observability dataset.
    pub async fn update_llm_obs_dataset_records(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRecordsUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
        datadog::Error<UpdateLLMObsDatasetRecordsError>,
    > {
        match self
            .update_llm_obs_dataset_records_with_http_info(project_id, dataset_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update one or more existing records in an LLM Observability dataset.
    pub async fn update_llm_obs_dataset_records_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        body: crate::datadogV2::model::LLMObsDatasetRecordsUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse>,
        datadog::Error<UpdateLLMObsDatasetRecordsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_dataset_records";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_dataset_records' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/{project_id}/datasets/{dataset_id}/records",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV2::model::LLMObsDatasetRecordsMutationResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsDatasetRecordsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update an existing LLM Observability experiment.
    pub async fn update_llm_obs_experiment(
        &self,
        experiment_id: String,
        body: crate::datadogV2::model::LLMObsExperimentUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsExperimentResponse,
        datadog::Error<UpdateLLMObsExperimentError>,
    > {
        match self
            .update_llm_obs_experiment_with_http_info(experiment_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update an existing LLM Observability experiment.
    pub async fn update_llm_obs_experiment_with_http_info(
        &self,
        experiment_id: String,
        body: crate::datadogV2::model::LLMObsExperimentUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsExperimentResponse>,
        datadog::Error<UpdateLLMObsExperimentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_experiment";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_experiment' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/experiments/{experiment_id}",
            local_configuration.get_operation_host(operation_id),
            experiment_id = datadog::urlencode(experiment_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsExperimentResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Partially update an existing LLM Observability project.
    pub async fn update_llm_obs_project(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsProjectUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::LLMObsProjectResponse,
        datadog::Error<UpdateLLMObsProjectError>,
    > {
        match self
            .update_llm_obs_project_with_http_info(project_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Partially update an existing LLM Observability project.
    pub async fn update_llm_obs_project_with_http_info(
        &self,
        project_id: String,
        body: crate::datadogV2::model::LLMObsProjectUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::LLMObsProjectResponse>,
        datadog::Error<UpdateLLMObsProjectError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_llm_obs_project";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_llm_obs_project' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v1/projects/{project_id}",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::LLMObsProjectResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateLLMObsProjectError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Upload records to a dataset from a file. The request is a `multipart/form-data` upload containing a single `file` part.
    /// Currently only CSV is supported. The CSV must include an `input` column. Optional columns are `id`, `expected_output`, `metadata`, and `tags`.
    ///
    /// The response is a Server-Sent Events stream (`text/event-stream`) emitting progress updates while records are processed. The stream emits the following named events:
    ///   - `progress`: incremental record counts written so far.
    ///   - `completed`: terminal event with a JSON body containing `records_created`.
    ///   - `error`: terminal event with a JSON body containing an error `message`.
    pub async fn upload_llm_obs_dataset_records_file(
        &self,
        project_id: String,
        dataset_id: String,
        params: UploadLLMObsDatasetRecordsFileOptionalParams,
    ) -> Result<(), datadog::Error<UploadLLMObsDatasetRecordsFileError>> {
        match self
            .upload_llm_obs_dataset_records_file_with_http_info(project_id, dataset_id, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Upload records to a dataset from a file. The request is a `multipart/form-data` upload containing a single `file` part.
    /// Currently only CSV is supported. The CSV must include an `input` column. Optional columns are `id`, `expected_output`, `metadata`, and `tags`.
    ///
    /// The response is a Server-Sent Events stream (`text/event-stream`) emitting progress updates while records are processed. The stream emits the following named events:
    ///   - `progress`: incremental record counts written so far.
    ///   - `completed`: terminal event with a JSON body containing `records_created`.
    ///   - `error`: terminal event with a JSON body containing an error `message`.
    pub async fn upload_llm_obs_dataset_records_file_with_http_info(
        &self,
        project_id: String,
        dataset_id: String,
        params: UploadLLMObsDatasetRecordsFileOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UploadLLMObsDatasetRecordsFileError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.upload_llm_obs_dataset_records_file";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.upload_llm_obs_dataset_records_file' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let file = params.file;
        let deduplicate = params.deduplicate;
        let overwrite = params.overwrite;
        let tags = params.tags;
        let include_user_data = params.include_user_data;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/llm-obs/v2/{project_id}/datasets/{dataset_id}/records/upload",
            local_configuration.get_operation_host(operation_id),
            project_id = datadog::urlencode(project_id),
            dataset_id = datadog::urlencode(dataset_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = deduplicate {
            local_req_builder =
                local_req_builder.query(&[("deduplicate", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = overwrite {
            local_req_builder =
                local_req_builder.query(&[("overwrite", &local_query_param.to_string())]);
        };
        if let Some(ref local) = tags {
            for param in local {
                local_req_builder = local_req_builder.query(&[("tags", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = include_user_data {
            local_req_builder =
                local_req_builder.query(&[("include[user_data]", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("multipart/form-data"),
        );
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build form parameters
        if let Some(file) = file {
            let mut local_form = form_data_builder::FormData::new(Vec::new());
            let cursor = std::io::Cursor::new(file);
            if let Err(e) = local_form.write_file(
                "file",
                cursor,
                Some("file".as_ref()),
                "application/octet-stream",
            ) {
                return Err(crate::datadog::Error::Io(e));
            };
            headers.insert(
                "Content-Type",
                local_form.content_type_header().parse().unwrap(),
            );
            let form_result = local_form.finish();
            match form_result {
                Ok(form) => local_req_builder = local_req_builder.body(form),
                Err(e) => return Err(crate::datadog::Error::Io(e)),
            };
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UploadLLMObsDatasetRecordsFileError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}
