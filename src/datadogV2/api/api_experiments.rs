// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetExposureSQLModelOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::get_exposure_sql_model`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetExposureSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count.
    pub include: Option<Vec<String>>,
}

impl GetExposureSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetMetricOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::get_metric`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMetricOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count: how many experiments currently reference this metric.
    pub include: Option<Vec<String>>,
}

impl GetMetricOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count: how many experiments currently reference this metric.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetMetricSQLModelOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::get_metric_sql_model`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMetricSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count
    /// and experiment_count.
    pub include: Option<Vec<String>>,
}

impl GetMetricSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count
    /// and experiment_count.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
}

/// GetSubjectTypeOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::get_subject_type`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetSubjectTypeOptionalParams {
    /// Set to `counts` to add experiment_count, exposure_source_count, metric_sql_model_count and protocol_count.
    pub include: Option<String>,
}

impl GetSubjectTypeOptionalParams {
    /// Set to `counts` to add experiment_count, exposure_source_count, metric_sql_model_count and protocol_count.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListExperimentProtocolsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_experiment_protocols`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListExperimentProtocolsOptionalParams {
    /// Filter by protocol status. Repeat this parameter to select more than one status.
    pub filter_status:
        Option<Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus>>,
    /// Filter by the UUID of the primary metric in the protocol template.
    pub filter_primary_metric_id: Option<uuid::Uuid>,
    /// Find protocols whose names contain the search text, regardless of case. Leading and trailing spaces are
    /// ignored. Blank values apply no filter. The maximum length is 1024 UTF-8 bytes.
    pub filter_query: Option<String>,
    /// Filter by the UUID of the subject type in the protocol template.
    pub filter_subject_type_id: Option<uuid::Uuid>,
    /// Number of results per page. The default is 25. Values above 50 are reduced to 50.
    pub page_limit: Option<i64>,
    /// Number of results to skip before returning this page.
    pub page_offset: Option<i64>,
    /// Sort by `name`, `subject_type_name`, `primary_metric_name`, or `updated_at`. Prefix with `-` for
    /// descending order. The default is `-updated_at`.
    pub sort: Option<String>,
}

impl ListExperimentProtocolsOptionalParams {
    /// Filter by protocol status. Repeat this parameter to select more than one status.
    pub fn filter_status(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus>,
    ) -> Self {
        self.filter_status = Some(value);
        self
    }
    /// Filter by the UUID of the primary metric in the protocol template.
    pub fn filter_primary_metric_id(mut self, value: uuid::Uuid) -> Self {
        self.filter_primary_metric_id = Some(value);
        self
    }
    /// Find protocols whose names contain the search text, regardless of case. Leading and trailing spaces are
    /// ignored. Blank values apply no filter. The maximum length is 1024 UTF-8 bytes.
    pub fn filter_query(mut self, value: String) -> Self {
        self.filter_query = Some(value);
        self
    }
    /// Filter by the UUID of the subject type in the protocol template.
    pub fn filter_subject_type_id(mut self, value: uuid::Uuid) -> Self {
        self.filter_subject_type_id = Some(value);
        self
    }
    /// Number of results per page. The default is 25. Values above 50 are reduced to 50.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip before returning this page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Sort by `name`, `subject_type_name`, `primary_metric_name`, or `updated_at`. Prefix with `-` for
    /// descending order. The default is `-updated_at`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListExperimentsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_experiments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListExperimentsOptionalParams {
    /// Return only experiments concluded at or after this RFC3339 timestamp. Inclusive, and excludes experiments that have not concluded.
    pub concluded_since: Option<chrono::DateTime<chrono::Utc>>,
    /// Return only experiments created at or after this RFC3339 timestamp. Inclusive.
    pub created_since: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub page_limit: Option<i64>,
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub page_offset: Option<i64>,
    /// Filter by protocol UUID. Repeat this parameter to supply several IDs. An experiment matches if it uses any
    /// listed protocol.
    pub protocol_id: Option<Vec<uuid::Uuid>>,
    /// Return only experiments whose results_last_updated is before this RFC3339 timestamp. results_last_updated is the later of the latest successful run completion and the latest stored result refresh. Exclusive, and excludes experiments without successful results.
    pub results_updated_before: Option<chrono::DateTime<chrono::Utc>>,
    /// Return only experiments whose results_last_updated is at or after this RFC3339 timestamp. results_last_updated is the later of the latest successful run completion and the latest stored result refresh. Inclusive, and excludes experiments without successful results. This filter does not include all metadata edits or deletions.
    pub results_updated_since: Option<chrono::DateTime<chrono::Utc>>,
    /// Find experiments whose names contain the search text, regardless of case.
    pub search: Option<String>,
    /// Sort fields: name, created_at, or updated_at. Use a comma-separated list in priority order, for example name,-created_at. Prefix each field with `-` for descending. Defaults to created_at descending.
    pub sort: Option<String>,
    /// Filter by experiment status. Accepted values are DRAFT, SCHEDULED, IN_PROGRESS, READY_FOR_DECISION,
    /// DECISION_MADE, and CANCELLED. Repeat this parameter to select several statuses, for example
    /// `status=IN_PROGRESS&status=READY_FOR_DECISION`.
    pub status: Option<Vec<String>>,
    /// Filter by tag name. Repeat this parameter to supply several tags. An experiment matches if it has at least
    /// one listed tag.
    pub tags: Option<Vec<String>>,
}

impl ListExperimentsOptionalParams {
    /// Return only experiments concluded at or after this RFC3339 timestamp. Inclusive, and excludes experiments that have not concluded.
    pub fn concluded_since(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.concluded_since = Some(value);
        self
    }
    /// Return only experiments created at or after this RFC3339 timestamp. Inclusive.
    pub fn created_since(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_since = Some(value);
        self
    }
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Filter by protocol UUID. Repeat this parameter to supply several IDs. An experiment matches if it uses any
    /// listed protocol.
    pub fn protocol_id(mut self, value: Vec<uuid::Uuid>) -> Self {
        self.protocol_id = Some(value);
        self
    }
    /// Return only experiments whose results_last_updated is before this RFC3339 timestamp. results_last_updated is the later of the latest successful run completion and the latest stored result refresh. Exclusive, and excludes experiments without successful results.
    pub fn results_updated_before(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.results_updated_before = Some(value);
        self
    }
    /// Return only experiments whose results_last_updated is at or after this RFC3339 timestamp. results_last_updated is the later of the latest successful run completion and the latest stored result refresh. Inclusive, and excludes experiments without successful results. This filter does not include all metadata edits or deletions.
    pub fn results_updated_since(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.results_updated_since = Some(value);
        self
    }
    /// Find experiments whose names contain the search text, regardless of case.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Sort fields: name, created_at, or updated_at. Use a comma-separated list in priority order, for example name,-created_at. Prefix each field with `-` for descending. Defaults to created_at descending.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Filter by experiment status. Accepted values are DRAFT, SCHEDULED, IN_PROGRESS, READY_FOR_DECISION,
    /// DECISION_MADE, and CANCELLED. Repeat this parameter to select several statuses, for example
    /// `status=IN_PROGRESS&status=READY_FOR_DECISION`.
    pub fn status(mut self, value: Vec<String>) -> Self {
        self.status = Some(value);
        self
    }
    /// Filter by tag name. Repeat this parameter to supply several tags. An experiment matches if it has at least
    /// one listed tag.
    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

/// ListExposureSQLModelsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_exposure_sql_models`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListExposureSQLModelsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count, which costs an extra aggregate query.
    pub include: Option<Vec<String>>,
    /// When true, archived models are included in the result. Defaults to false, so archived models are hidden.
    pub include_archived: Option<bool>,
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub page_limit: Option<i64>,
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub page_offset: Option<i64>,
    /// Find exposure SQL models whose names contain the search text, regardless of case.
    pub search: Option<String>,
    /// Sort field: name, created_at, or updated_at. Prefix with `-` for descending (for example, `-created_at`).
    /// Defaults to created_at descending.
    pub sort: Option<String>,
}

impl ListExposureSQLModelsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count, which costs an extra aggregate query.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
    /// When true, archived models are included in the result. Defaults to false, so archived models are hidden.
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Find exposure SQL models whose names contain the search text, regardless of case.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Sort field: name, created_at, or updated_at. Prefix with `-` for descending (for example, `-created_at`).
    /// Defaults to created_at descending.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListMetricCollectionsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_metric_collections`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMetricCollectionsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count.
    pub include: Option<Vec<String>>,
    /// Number of results per page. The default is 25. Values above 50 are reduced to 50.
    pub page_limit: Option<i64>,
    /// Number of results to skip before returning this page.
    pub page_offset: Option<i64>,
    /// Find collections whose names contain the search text, regardless of case. Leading and trailing spaces are
    /// ignored. Blank values apply no filter. The maximum length is 1024 UTF-8 bytes.
    pub search: Option<String>,
    /// Sort by one field: `name`, `created_at`, or `updated_at`. Prefix with `-` for descending order. The
    /// default is `-created_at`.
    pub sort: Option<String>,
}

impl ListMetricCollectionsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
    /// Number of results per page. The default is 25. Values above 50 are reduced to 50.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip before returning this page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Find collections whose names contain the search text, regardless of case. Leading and trailing spaces are
    /// ignored. Blank values apply no filter. The maximum length is 1024 UTF-8 bytes.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Sort by one field: `name`, `created_at`, or `updated_at`. Prefix with `-` for descending order. The
    /// default is `-created_at`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListMetricSQLModelsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_metric_sql_models`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMetricSQLModelsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count
    /// and experiment_count, which cost an extra aggregate query.
    pub include: Option<Vec<String>>,
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub page_limit: Option<i64>,
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub page_offset: Option<i64>,
    /// Sort field: name, created_at, updated_at, metric_count, or experiment_count. A single field only; a
    /// comma-separated list is rejected. Prefix with `-` for descending (for example, `-created_at`). Defaults to
    /// created_at descending.
    pub sort: Option<String>,
}

impl ListMetricSQLModelsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds metric_count
    /// and experiment_count, which cost an extra aggregate query.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Sort field: name, created_at, updated_at, metric_count, or experiment_count. A single field only; a
    /// comma-separated list is rejected. Prefix with `-` for descending (for example, `-created_at`). Defaults to
    /// created_at descending.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListMetricsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_metrics`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMetricsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count on each metric, which costs an extra aggregate query.
    pub include: Option<Vec<String>>,
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub page_limit: Option<i64>,
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub page_offset: Option<i64>,
    /// Find metrics whose names contain the search text, regardless of case.
    pub search: Option<String>,
    /// Sort field: name, created_at, or updated_at. Prefix with `-` for descending (for example, `-created_at`).
    /// A single field only; a comma-separated list is rejected. Defaults to created_at descending.
    pub sort: Option<String>,
}

impl ListMetricsOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count on each metric, which costs an extra aggregate query.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Find metrics whose names contain the search text, regardless of case.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Sort field: name, created_at, or updated_at. Prefix with `-` for descending (for example, `-created_at`).
    /// A single field only; a comma-separated list is rejected. Defaults to created_at descending.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListSubjectTypesOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::list_subject_types`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSubjectTypesOptionalParams {
    /// Set to `counts` to add experiment_count, exposure_source_count, metric_sql_model_count and protocol_count to each subject type. Each costs an extra query, so they are omitted unless asked for.
    pub include: Option<String>,
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub page_limit: Option<i64>,
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub page_offset: Option<i64>,
    /// Find subject types whose names contain the search text, regardless of case.
    pub search: Option<String>,
    /// Sort fields: name, created_at, or updated_at. Use a comma-separated list in priority order, for example name,-created_at. Prefix each field with `-` for descending. Defaults to created_at descending.
    pub sort: Option<String>,
}

impl ListSubjectTypesOptionalParams {
    /// Set to `counts` to add experiment_count, exposure_source_count, metric_sql_model_count and protocol_count to each subject type. Each costs an extra query, so they are omitted unless asked for.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Maximum number of results to return. Defaults to 25 when omitted, and is capped at 50 (larger values are clamped to 50). The response includes meta.page (with total) and pagination links.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Number of results to skip for pagination. Defaults to 0 when omitted.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Find subject types whose names contain the search text, regardless of case.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// Sort fields: name, created_at, or updated_at. Use a comma-separated list in priority order, for example name,-created_at. Prefix each field with `-` for descending. Defaults to created_at descending.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// RefreshExperimentResultsOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::refresh_experiment_results`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct RefreshExperimentResultsOptionalParams {
    /// Force a full warehouse rebuild. Defaults to false when omitted.
    pub full_refresh: Option<bool>,
}

impl RefreshExperimentResultsOptionalParams {
    /// Force a full warehouse rebuild. Defaults to false when omitted.
    pub fn full_refresh(mut self, value: bool) -> Self {
        self.full_refresh = Some(value);
        self
    }
}

/// RefreshExperimentResultsForOrgOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::refresh_experiment_results_for_org`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct RefreshExperimentResultsForOrgOptionalParams {
    /// Force a full warehouse rebuild. Defaults to false when omitted.
    pub full_refresh: Option<bool>,
}

impl RefreshExperimentResultsForOrgOptionalParams {
    /// Force a full warehouse rebuild. Defaults to false when omitted.
    pub fn full_refresh(mut self, value: bool) -> Self {
        self.full_refresh = Some(value);
        self
    }
}

/// StartExperimentOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::start_experiment`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct StartExperimentOptionalParams {
    pub body: Option<crate::datadogV2::model::ExperimentsStartExperimentV2Request>,
}

impl StartExperimentOptionalParams {
    pub fn body(
        mut self,
        value: crate::datadogV2::model::ExperimentsStartExperimentV2Request,
    ) -> Self {
        self.body = Some(value);
        self
    }
}

/// UpdateExposureSQLModelOptionalParams is a struct for passing parameters to the method [`ExperimentsAPI::update_exposure_sql_model`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateExposureSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count to the updated model.
    pub include: Option<Vec<String>>,
}

impl UpdateExposureSQLModelOptionalParams {
    /// Optional fields to include. Repeat this parameter to request several fields. `counts` adds
    /// experiment_count to the updated model.
    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }
}

/// ArchiveExposureSQLModelError is a struct for typed errors of method [`ExperimentsAPI::archive_exposure_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveExposureSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CancelExperimentError is a struct for typed errors of method [`ExperimentsAPI::cancel_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ConcludeExperimentError is a struct for typed errors of method [`ExperimentsAPI::conclude_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConcludeExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateExperimentError is a struct for typed errors of method [`ExperimentsAPI::create_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateExperimentMetricGroupError is a struct for typed errors of method [`ExperimentsAPI::create_experiment_metric_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExperimentMetricGroupError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateExperimentMetricGroupFromCollectionError is a struct for typed errors of method [`ExperimentsAPI::create_experiment_metric_group_from_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExperimentMetricGroupFromCollectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateExposureSQLModelError is a struct for typed errors of method [`ExperimentsAPI::create_exposure_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateExposureSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMetricError is a struct for typed errors of method [`ExperimentsAPI::create_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMetricError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMetricCollectionError is a struct for typed errors of method [`ExperimentsAPI::create_metric_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMetricCollectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMetricSQLModelError is a struct for typed errors of method [`ExperimentsAPI::create_metric_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMetricSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSubjectTypeError is a struct for typed errors of method [`ExperimentsAPI::create_subject_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSubjectTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteExperimentError is a struct for typed errors of method [`ExperimentsAPI::delete_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteExperimentMetricGroupError is a struct for typed errors of method [`ExperimentsAPI::delete_experiment_metric_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteExperimentMetricGroupError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteMetricError is a struct for typed errors of method [`ExperimentsAPI::delete_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMetricError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteMetricCollectionError is a struct for typed errors of method [`ExperimentsAPI::delete_metric_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMetricCollectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSubjectTypeError is a struct for typed errors of method [`ExperimentsAPI::delete_subject_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSubjectTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentError is a struct for typed errors of method [`ExperimentsAPI::get_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentAnalysisPlanError is a struct for typed errors of method [`ExperimentsAPI::get_experiment_analysis_plan`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentAnalysisPlanError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentDiagnosticsError is a struct for typed errors of method [`ExperimentsAPI::get_experiment_diagnostics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentDiagnosticsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentProtocolError is a struct for typed errors of method [`ExperimentsAPI::get_experiment_protocol`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentProtocolError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentResultsError is a struct for typed errors of method [`ExperimentsAPI::get_experiment_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentResultsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExperimentTrafficSummaryError is a struct for typed errors of method [`ExperimentsAPI::get_experiment_traffic_summary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExperimentTrafficSummaryError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetExposureSQLModelError is a struct for typed errors of method [`ExperimentsAPI::get_exposure_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetExposureSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMetricError is a struct for typed errors of method [`ExperimentsAPI::get_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMetricCollectionError is a struct for typed errors of method [`ExperimentsAPI::get_metric_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricCollectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMetricSQLModelError is a struct for typed errors of method [`ExperimentsAPI::get_metric_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMetricSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSubjectTypeError is a struct for typed errors of method [`ExperimentsAPI::get_subject_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSubjectTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListExperimentMetricGroupsError is a struct for typed errors of method [`ExperimentsAPI::list_experiment_metric_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListExperimentMetricGroupsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListExperimentProtocolsError is a struct for typed errors of method [`ExperimentsAPI::list_experiment_protocols`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListExperimentProtocolsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListExperimentsError is a struct for typed errors of method [`ExperimentsAPI::list_experiments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListExperimentsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListExposureSQLModelsError is a struct for typed errors of method [`ExperimentsAPI::list_exposure_sql_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListExposureSQLModelsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMetricCollectionsError is a struct for typed errors of method [`ExperimentsAPI::list_metric_collections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricCollectionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMetricSQLModelsError is a struct for typed errors of method [`ExperimentsAPI::list_metric_sql_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricSQLModelsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMetricsError is a struct for typed errors of method [`ExperimentsAPI::list_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMetricsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSubjectTypesError is a struct for typed errors of method [`ExperimentsAPI::list_subject_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSubjectTypesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchExperimentError is a struct for typed errors of method [`ExperimentsAPI::patch_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// PatchSubjectTypeError is a struct for typed errors of method [`ExperimentsAPI::patch_subject_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchSubjectTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RefreshExperimentResultsError is a struct for typed errors of method [`ExperimentsAPI::refresh_experiment_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshExperimentResultsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RefreshExperimentResultsForOrgError is a struct for typed errors of method [`ExperimentsAPI::refresh_experiment_results_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshExperimentResultsForOrgError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SetDefaultSubjectTypeError is a struct for typed errors of method [`ExperimentsAPI::set_default_subject_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDefaultSubjectTypeError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// StartExperimentError is a struct for typed errors of method [`ExperimentsAPI::start_experiment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartExperimentError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnarchiveExposureSQLModelError is a struct for typed errors of method [`ExperimentsAPI::unarchive_exposure_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnarchiveExposureSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateExperimentAnalysisPlanAttributesError is a struct for typed errors of method [`ExperimentsAPI::update_experiment_analysis_plan_attributes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateExperimentAnalysisPlanAttributesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateExperimentMetricGroupError is a struct for typed errors of method [`ExperimentsAPI::update_experiment_metric_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateExperimentMetricGroupError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateExposureSQLModelError is a struct for typed errors of method [`ExperimentsAPI::update_exposure_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateExposureSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMetricError is a struct for typed errors of method [`ExperimentsAPI::update_metric`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMetricCollectionError is a struct for typed errors of method [`ExperimentsAPI::update_metric_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricCollectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMetricSQLModelError is a struct for typed errors of method [`ExperimentsAPI::update_metric_sql_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMetricSQLModelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Create and manage experiments, metrics, subject types, SQL models, and protocols.
#[derive(Debug, Clone)]
pub struct ExperimentsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ExperimentsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ExperimentsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
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

    /// Archive an exposure SQL model. Archived models are hidden from the default list and are no longer refreshed for new feature flags. Experiments already reading from the model keep working. Archiving is how a model that is in use by an experiment, and therefore cannot be deleted, is retired.
    pub async fn archive_exposure_sql_model(
        &self,
        exposure_sql_model_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<ArchiveExposureSQLModelError>> {
        match self
            .archive_exposure_sql_model_with_http_info(exposure_sql_model_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Archive an exposure SQL model. Archived models are hidden from the default list and are no longer refreshed for new feature flags. Experiments already reading from the model keep working. Archiving is how a model that is in use by an experiment, and therefore cannot be deleted, is retired.
    pub async fn archive_exposure_sql_model_with_http_info(
        &self,
        exposure_sql_model_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ArchiveExposureSQLModelError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.archive_exposure_sql_model";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models/{exposure_sql_model_id}/archive",
            local_configuration.get_operation_host(local_operation_id),
            exposure_sql_model_id = datadog::urlencode(exposure_sql_model_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<ArchiveExposureSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Cancel an experiment, ending it without a winning variant. The experiment moves to CANCELLED status, the
    /// supplied reason is recorded in its conclusion as the decision reason, and the experiment is unlinked from the
    /// feature flag allocations that exposed it, which stops its exposure. An experiment that has already completed
    /// its rollout, had its code removed, or been canceled cannot be canceled again. Canceling is not reversible: an
    /// experiment cannot be returned to a running state afterward. It is also not idempotent: canceling an
    /// already-canceled experiment returns 409, so a retry after a timeout cannot be distinguished from a
    /// cancellation made by someone else.
    pub async fn cancel_experiment(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCancelExperimentV2Request,
    ) -> Result<(), datadog::Error<CancelExperimentError>> {
        match self
            .cancel_experiment_with_http_info(experiment_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Cancel an experiment, ending it without a winning variant. The experiment moves to CANCELLED status, the
    /// supplied reason is recorded in its conclusion as the decision reason, and the experiment is unlinked from the
    /// feature flag allocations that exposed it, which stops its exposure. An experiment that has already completed
    /// its rollout, had its code removed, or been canceled cannot be canceled again. Canceling is not reversible: an
    /// experiment cannot be returned to a running state afterward. It is also not idempotent: canceling an
    /// already-canceled experiment returns 409, so a retry after a timeout cannot be distinguished from a
    /// cancellation made by someone else.
    pub async fn cancel_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCancelExperimentV2Request,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<CancelExperimentError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.cancel_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/cancel",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            let local_entity: Option<CancelExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Conclude an experiment on a winning variant. The experiment moves to DECISION_MADE status, the outcome is recorded in its conclusion, and for a flag-backed experiment the winning variant is rolled out to 100% of the linked feature flag allocation. `decision_variant_key` must match a variant in the experiment. Only an experiment that is currently running or ready for a decision can be concluded. Concluding is not reversible and is not idempotent: concluding an already-concluded experiment returns 409.
    pub async fn conclude_experiment(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsConcludeExperimentV2Request,
    ) -> Result<(), datadog::Error<ConcludeExperimentError>> {
        match self
            .conclude_experiment_with_http_info(experiment_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Conclude an experiment on a winning variant. The experiment moves to DECISION_MADE status, the outcome is recorded in its conclusion, and for a flag-backed experiment the winning variant is rolled out to 100% of the linked feature flag allocation. `decision_variant_key` must match a variant in the experiment. Only an experiment that is currently running or ready for a decision can be concluded. Concluding is not reversible and is not idempotent: concluding an already-concluded experiment returns 409.
    pub async fn conclude_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsConcludeExperimentV2Request,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<ConcludeExperimentError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.conclude_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/conclude",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            let local_entity: Option<ConcludeExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a draft experiment. `name` is required. `structured_metadata` identifies each metadata field by `field_key`; use `freetext_value` for free-text fields and `enum_values` for enum fields. When this attribute is present, the request must include a value for every required metadata field. When `protocol_id` is present, the published protocol supplies the subject type, decision metrics, analysis-plan defaults, and configuration and enforcement baselines. The request may also include hypothesis, tags, teams, related links, and assignment or event date overrides that satisfy the protocol's duration rules; omit subject_type_id, decision_metrics, variants, warehouse_exposure_configuration, datadog_flag_configuration, traffic_exposure, split_by_properties, and structured_metadata. The protocol association cannot be changed after creation. Without `protocol_id`, a complete Warehouse or Datadog configuration saves the experiment and its configuration in one transaction. For Datadog flag configuration, send `name`, `subject_type_id`, `decision_metrics`, `variants`, `traffic_exposure`, `assignments_start_date`, `assignments_end_date`, `events_start_date`, and `events_end_date`. The four date fields can be null. Inside `datadog_flag_configuration`, send `feature_flag_id`, `environment_id`, `targeting_rules`, and `entry_point`. Use `targeting_rules: []` and `entry_point: null` when unused. This creates one saved draft allocation that does not serve traffic. Omit all configuration fields to create an experiment without an allocation. This endpoint is not idempotent.
    pub async fn create_experiment(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateExperimentV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentV2DTO,
        datadog::Error<CreateExperimentError>,
    > {
        match self.create_experiment_with_http_info(body).await {
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

    /// Create a draft experiment. `name` is required. `structured_metadata` identifies each metadata field by `field_key`; use `freetext_value` for free-text fields and `enum_values` for enum fields. When this attribute is present, the request must include a value for every required metadata field. When `protocol_id` is present, the published protocol supplies the subject type, decision metrics, analysis-plan defaults, and configuration and enforcement baselines. The request may also include hypothesis, tags, teams, related links, and assignment or event date overrides that satisfy the protocol's duration rules; omit subject_type_id, decision_metrics, variants, warehouse_exposure_configuration, datadog_flag_configuration, traffic_exposure, split_by_properties, and structured_metadata. The protocol association cannot be changed after creation. Without `protocol_id`, a complete Warehouse or Datadog configuration saves the experiment and its configuration in one transaction. For Datadog flag configuration, send `name`, `subject_type_id`, `decision_metrics`, `variants`, `traffic_exposure`, `assignments_start_date`, `assignments_end_date`, `events_start_date`, and `events_end_date`. The four date fields can be null. Inside `datadog_flag_configuration`, send `feature_flag_id`, `environment_id`, `targeting_rules`, and `entry_point`. Use `targeting_rules: []` and `entry_point: null` when unused. This creates one saved draft allocation that does not serve traffic. Omit all configuration fields to create an experiment without an allocation. This endpoint is not idempotent.
    pub async fn create_experiment_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateExperimentV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExperimentV2DTO>,
        datadog::Error<CreateExperimentError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsExperimentV2DTO>(
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
            let local_entity: Option<CreateExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a non-decision metric group. The optional metrics array is ordered. Decision groups remain managed through decision_metrics on the experiment resource. This operation does not synchronously recompute results.
    pub async fn create_experiment_metric_group(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        datadog::Error<CreateExperimentMetricGroupError>,
    > {
        match self
            .create_experiment_metric_group_with_http_info(experiment_id, body)
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

    /// Create a non-decision metric group. The optional metrics array is ordered. Decision groups remain managed through decision_metrics on the experiment resource. This operation does not synchronously recompute results.
    pub async fn create_experiment_metric_group_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateExperimentMetricGroupV2Request,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        >,
        datadog::Error<CreateExperimentMetricGroupError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_experiment_metric_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/metric-groups",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
                crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
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
            let local_entity: Option<CreateExperimentMetricGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Copy a metric collection into a new non-decision metric group. The group is a request-time snapshot: later changes to the collection do not affect the experiment. Metric order is preserved. The operation is not idempotent, and incompatible or empty collections are rejected without creating a group.
    pub async fn create_experiment_metric_group_from_collection(
        &self,
        experiment_id: uuid::Uuid,
        metric_collection_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        datadog::Error<CreateExperimentMetricGroupFromCollectionError>,
    > {
        match self
            .create_experiment_metric_group_from_collection_with_http_info(
                experiment_id,
                metric_collection_id,
            )
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

    /// Copy a metric collection into a new non-decision metric group. The group is a request-time snapshot: later changes to the collection do not affect the experiment. Metric order is preserved. The operation is not idempotent, and incompatible or empty collections are rejected without creating a group.
    pub async fn create_experiment_metric_group_from_collection_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        metric_collection_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        >,
        datadog::Error<CreateExperimentMetricGroupFromCollectionError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_experiment_metric_group_from_collection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/metric-groups/from-collection/{metric_collection_id}",
            local_configuration.get_operation_host(local_operation_id), experiment_id=
            datadog::urlencode(experiment_id.to_string())
            , metric_collection_id=
            datadog::urlencode(metric_collection_id.to_string())
            );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
                crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
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
            let local_entity: Option<CreateExperimentMetricGroupFromCollectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create an exposure SQL model. Requires at least one subject type. The warehouse connection is resolved from the organization, which has exactly one.
    pub async fn create_exposure_sql_model(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO,
        datadog::Error<CreateExposureSQLModelError>,
    > {
        match self.create_exposure_sql_model_with_http_info(body).await {
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

    /// Create an exposure SQL model. Requires at least one subject type. The warehouse connection is resolved from the organization, which has exactly one.
    pub async fn create_exposure_sql_model_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO>,
        datadog::Error<CreateExposureSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_exposure_sql_model";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO>(
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
            let local_entity: Option<CreateExposureSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a metric. The metric's type is derived from the aggregation shape: a numerator alone is SIMPLE, a numerator with a denominator is RATIO, and a percentile aggregation is PERCENTILE. Warehouse aggregations reference measures by UUID. Property filters use property_id or measure_id UUIDs returned by the same metric SQL model; every reference must belong to the aggregation's data source. The is_certified attribute is rejected. Certification cannot be changed through this endpoint.
    pub async fn create_metric(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricV2Request,
    ) -> Result<crate::datadogV2::model::ExperimentsMetricV2DTO, datadog::Error<CreateMetricError>>
    {
        match self.create_metric_with_http_info(body).await {
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

    /// Create a metric. The metric's type is derived from the aggregation shape: a numerator alone is SIMPLE, a numerator with a denominator is RATIO, and a percentile aggregation is PERCENTILE. Warehouse aggregations reference measures by UUID. Property filters use property_id or measure_id UUIDs returned by the same metric SQL model; every reference must belong to the aggregation's data source. The is_certified attribute is rejected. Certification cannot be changed through this endpoint.
    pub async fn create_metric_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricV2DTO>,
        datadog::Error<CreateMetricError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_metric";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metrics",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricV2DTO>(
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
            let local_entity: Option<CreateMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create metric collection.
    pub async fn create_metric_collection(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricCollectionV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricCollectionV2DTO,
        datadog::Error<CreateMetricCollectionError>,
    > {
        match self.create_metric_collection_with_http_info(body).await {
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

    /// Create metric collection.
    pub async fn create_metric_collection_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricCollectionV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>,
        datadog::Error<CreateMetricCollectionError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_metric_collection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-collections",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>(
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
            let local_entity: Option<CreateMetricCollectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a metric SQL model. Requires at least one subject type, whose subject_type_id must already exist for the organization (list them with GET /api/v2/experiments/subject-types). The model is created against the organization's warehouse connection, which is resolved server-side. Only customer-defined measures belong in measures. The response provides unique_subject_count_measure_id for each subject type and event_count_measure_id for use in metric aggregations. column_type is required for every measure and property. Certification is read-only and cannot be changed through this endpoint.
    pub async fn create_metric_sql_model(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO,
        datadog::Error<CreateMetricSQLModelError>,
    > {
        match self.create_metric_sql_model_with_http_info(body).await {
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

    /// Create a metric SQL model. Requires at least one subject type, whose subject_type_id must already exist for the organization (list them with GET /api/v2/experiments/subject-types). The model is created against the organization's warehouse connection, which is resolved server-side. Only customer-defined measures belong in measures. The response provides unique_subject_count_measure_id for each subject type and event_count_measure_id for use in metric aggregations. column_type is required for every measure and property. Certification is read-only and cannot be changed through this endpoint.
    pub async fn create_metric_sql_model_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO>,
        datadog::Error<CreateMetricSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_metric_sql_model";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-sql-models",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO>(
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
            let local_entity: Option<CreateMetricSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create a subject type for the organization.
    pub async fn create_subject_type(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateSubjectTypeV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsSubjectTypeV2DTO,
        datadog::Error<CreateSubjectTypeError>,
    > {
        match self.create_subject_type_with_http_info(body).await {
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

    /// Create a subject type for the organization.
    pub async fn create_subject_type_with_http_info(
        &self,
        body: crate::datadogV2::model::ExperimentsCreateSubjectTypeV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>,
        datadog::Error<CreateSubjectTypeError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_subject_type";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types",
            local_configuration.get_operation_host(local_operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>(
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
            let local_entity: Option<CreateSubjectTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an experiment and its linked feature flag allocations in one database transaction. After deletion, the experiment is no longer returned by the API. If the transaction fails, neither the experiment nor its allocations are deleted. Deleting an experiment cannot be undone.
    pub async fn delete_experiment(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteExperimentError>> {
        match self.delete_experiment_with_http_info(experiment_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an experiment and its linked feature flag allocations in one database transaction. After deletion, the experiment is no longer returned by the API. If the transaction fails, neither the experiment nor its allocations are deleted. Deleting an experiment cannot be undone.
    pub async fn delete_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteExperimentError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            let local_entity: Option<DeleteExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a non-decision metric group and its memberships. Decision groups remain managed through the experiment resource. This operation does not start a pipeline. Read experiment results after deletion to check stale metadata, then explicitly refresh results when required.
    pub async fn delete_experiment_metric_group(
        &self,
        experiment_id: uuid::Uuid,
        metric_group_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteExperimentMetricGroupError>> {
        match self
            .delete_experiment_metric_group_with_http_info(experiment_id, metric_group_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a non-decision metric group and its memberships. Decision groups remain managed through the experiment resource. This operation does not start a pipeline. Read experiment results after deletion to check stale metadata, then explicitly refresh results when required.
    pub async fn delete_experiment_metric_group_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        metric_group_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteExperimentMetricGroupError>>
    {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_experiment_metric_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/metric-groups/{metric_group_id}",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string()),
            metric_group_id = datadog::urlencode(metric_group_id.to_string())
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
            let local_entity: Option<DeleteExperimentMetricGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a metric. Certified metrics are read-only through this endpoint. The record is soft-deleted and stops appearing in reads. A metric still referenced by an experiment cannot be deleted; detach it from those experiments first.
    pub async fn delete_metric(
        &self,
        metric_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteMetricError>> {
        match self.delete_metric_with_http_info(metric_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a metric. Certified metrics are read-only through this endpoint. The record is soft-deleted and stops appearing in reads. A metric still referenced by an experiment cannot be deleted; detach it from those experiments first.
    pub async fn delete_metric_with_http_info(
        &self,
        metric_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteMetricError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_metric";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metrics/{metric_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_id = datadog::urlencode(metric_id.to_string())
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
            let local_entity: Option<DeleteMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete metric collection.
    pub async fn delete_metric_collection(
        &self,
        metric_collection_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteMetricCollectionError>> {
        match self
            .delete_metric_collection_with_http_info(metric_collection_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete metric collection.
    pub async fn delete_metric_collection_with_http_info(
        &self,
        metric_collection_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteMetricCollectionError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_metric_collection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-collections/{metric_collection_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_collection_id = datadog::urlencode(metric_collection_id.to_string())
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
            let local_entity: Option<DeleteMetricCollectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete a subject type. The record is soft-deleted and stops appearing in reads. The call is idempotent: deleting the same subject type again also returns 204. The organization's default subject type cannot be deleted; make another one the default first. A subject type that experiments, exposure SQL models, metric SQL models or protocols still reference cannot be deleted either; the refusal names the blockers.
    pub async fn delete_subject_type(
        &self,
        subject_type_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<DeleteSubjectTypeError>> {
        match self
            .delete_subject_type_with_http_info(subject_type_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a subject type. The record is soft-deleted and stops appearing in reads. The call is idempotent: deleting the same subject type again also returns 204. The organization's default subject type cannot be deleted; make another one the default first. A subject type that experiments, exposure SQL models, metric SQL models or protocols still reference cannot be deleted either; the refusal names the blockers.
    pub async fn delete_subject_type_with_http_info(
        &self,
        subject_type_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteSubjectTypeError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_subject_type";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types/{subject_type_id}",
            local_configuration.get_operation_host(local_operation_id),
            subject_type_id = datadog::urlencode(subject_type_id.to_string())
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
            let local_entity: Option<DeleteSubjectTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a complete experiment by ID. The response includes structured metadata, related links, and, when a complete setup exists, decision metrics, variants, `warehouse_exposure_configuration`, `datadog_flag_configuration`, STATIC or STEPS traffic exposure, and assignment and event dates. STEPS describes the configured plan rather than wall-clock history; Datadog step durations exclude pauses. The list endpoint omits these setup details.
    pub async fn get_experiment(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentV2DTO,
        datadog::Error<GetExperimentError>,
    > {
        match self.get_experiment_with_http_info(experiment_id).await {
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

    /// Get a complete experiment by ID. The response includes structured metadata, related links, and, when a complete setup exists, decision metrics, variants, `warehouse_exposure_configuration`, `datadog_flag_configuration`, STATIC or STEPS traffic exposure, and assignment and event dates. STEPS describes the configured plan rather than wall-clock history; Datadog step durations exclude pauses. The list endpoint omits these setup details.
    pub async fn get_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExperimentV2DTO>,
        datadog::Error<GetExperimentError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsExperimentV2DTO>(
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
            let local_entity: Option<GetExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the effective public statistical analysis settings for an experiment. has_custom_analysis_settings compares only settings the caller can edit; protocol-required differences from company defaults do not make the plan custom.
    pub async fn get_experiment_analysis_plan(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsAnalysisPlanV2DTO,
        datadog::Error<GetExperimentAnalysisPlanError>,
    > {
        match self
            .get_experiment_analysis_plan_with_http_info(experiment_id)
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

    /// Get the effective public statistical analysis settings for an experiment. has_custom_analysis_settings compares only settings the caller can edit; protocol-required differences from company defaults do not make the plan custom.
    pub async fn get_experiment_analysis_plan_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsAnalysisPlanV2DTO>,
        datadog::Error<GetExperimentAnalysisPlanError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment_analysis_plan";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/analysis-plan",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsAnalysisPlanV2DTO>(
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
            let local_entity: Option<GetExperimentAnalysisPlanError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the diagnostics produced by an experiment's latest analysis run. Each diagnostic includes its category, and the response includes an overall diagnostic or pipeline lifecycle status.
    pub async fn get_experiment_diagnostics(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTO,
        datadog::Error<GetExperimentDiagnosticsError>,
    > {
        match self
            .get_experiment_diagnostics_with_http_info(experiment_id)
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

    /// Get the diagnostics produced by an experiment's latest analysis run. Each diagnostic includes its category, and the response includes an overall diagnostic or pipeline lifecycle status.
    pub async fn get_experiment_diagnostics_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTO>,
        datadog::Error<GetExperimentDiagnosticsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment_diagnostics";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/diagnostics",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
                crate::datadogV2::model::ExperimentsExperimentDiagnosticsV2DTO,
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
            let local_entity: Option<GetExperimentDiagnosticsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a draft, published, or archived experiment protocol by ID.
    pub async fn get_experiment_protocol(
        &self,
        protocol_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsPublicProtocolResponse,
        datadog::Error<GetExperimentProtocolError>,
    > {
        match self
            .get_experiment_protocol_with_http_info(protocol_id)
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

    /// Get a draft, published, or archived experiment protocol by ID.
    pub async fn get_experiment_protocol_with_http_info(
        &self,
        protocol_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsPublicProtocolResponse>,
        datadog::Error<GetExperimentProtocolError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment_protocol";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/protocols/{protocol_id}",
            local_configuration.get_operation_host(local_operation_id),
            protocol_id = datadog::urlencode(protocol_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsPublicProtocolResponse>(
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
            let local_entity: Option<GetExperimentProtocolError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an experiment's computed results: per-variant statistical analysis for the latest successful run. A historical run cannot be selected. Unavailable analysis statistics, including `p_value` and `confidence_interval`, are omitted. The `numerator`, `denominator`, and `variant_metric_value` fields can be null when their values are unavailable. Do not treat an omitted or null value as zero.
    pub async fn get_experiment_results(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsVariantResultsV2DTOArray,
        datadog::Error<GetExperimentResultsError>,
    > {
        match self
            .get_experiment_results_with_http_info(experiment_id)
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

    /// Get an experiment's computed results: per-variant statistical analysis for the latest successful run. A historical run cannot be selected. Unavailable analysis statistics, including `p_value` and `confidence_interval`, are omitted. The `numerator`, `denominator`, and `variant_metric_value` fields can be null when their values are unavailable. Do not treat an omitted or null value as zero.
    pub async fn get_experiment_results_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsVariantResultsV2DTOArray>,
        datadog::Error<GetExperimentResultsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment_results";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/results",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsVariantResultsV2DTOArray>(
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
            let local_entity: Option<GetExperimentResultsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an experiment's traffic summary: per-variant exposure counts and a sample-ratio-mismatch flag (is_traffic_imbalanced). SRM statistics live on the diagnostics endpoint.
    pub async fn get_experiment_traffic_summary(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsTrafficSummaryV2DTO,
        datadog::Error<GetExperimentTrafficSummaryError>,
    > {
        match self
            .get_experiment_traffic_summary_with_http_info(experiment_id)
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

    /// Get an experiment's traffic summary: per-variant exposure counts and a sample-ratio-mismatch flag (is_traffic_imbalanced). SRM statistics live on the diagnostics endpoint.
    pub async fn get_experiment_traffic_summary_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsTrafficSummaryV2DTO>,
        datadog::Error<GetExperimentTrafficSummaryError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_experiment_traffic_summary";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/traffic-summary",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsTrafficSummaryV2DTO>(
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
            let local_entity: Option<GetExperimentTrafficSummaryError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an exposure SQL model. Returns a single model by its ID for the organization, including its subject types and properties.
    pub async fn get_exposure_sql_model(
        &self,
        exposure_sql_model_id: uuid::Uuid,
        params: GetExposureSQLModelOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO,
        datadog::Error<GetExposureSQLModelError>,
    > {
        match self
            .get_exposure_sql_model_with_http_info(exposure_sql_model_id, params)
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

    /// Get an exposure SQL model. Returns a single model by its ID for the organization, including its subject types and properties.
    pub async fn get_exposure_sql_model_with_http_info(
        &self,
        exposure_sql_model_id: uuid::Uuid,
        params: GetExposureSQLModelOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO>,
        datadog::Error<GetExposureSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_exposure_sql_model";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models/{exposure_sql_model_id}",
            local_configuration.get_operation_host(local_operation_id),
            exposure_sql_model_id = datadog::urlencode(exposure_sql_model_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTO>(
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
            let local_entity: Option<GetExposureSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a metric. Returns a single experiment metric by its ID for the organization.
    pub async fn get_metric(
        &self,
        metric_id: uuid::Uuid,
        params: GetMetricOptionalParams,
    ) -> Result<crate::datadogV2::model::ExperimentsMetricV2DTO, datadog::Error<GetMetricError>>
    {
        match self.get_metric_with_http_info(metric_id, params).await {
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

    /// Get a metric. Returns a single experiment metric by its ID for the organization.
    pub async fn get_metric_with_http_info(
        &self,
        metric_id: uuid::Uuid,
        params: GetMetricOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricV2DTO>,
        datadog::Error<GetMetricError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_metric";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metrics/{metric_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_id = datadog::urlencode(metric_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricV2DTO>(
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
            let local_entity: Option<GetMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get metric collection.
    pub async fn get_metric_collection(
        &self,
        metric_collection_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricCollectionV2DTO,
        datadog::Error<GetMetricCollectionError>,
    > {
        match self
            .get_metric_collection_with_http_info(metric_collection_id)
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

    /// Get metric collection.
    pub async fn get_metric_collection_with_http_info(
        &self,
        metric_collection_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>,
        datadog::Error<GetMetricCollectionError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_metric_collection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-collections/{metric_collection_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_collection_id = datadog::urlencode(metric_collection_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>(
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
            let local_entity: Option<GetMetricCollectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a metric SQL model. Returns a single model by its ID for the organization, including its subject types, measures and properties.
    pub async fn get_metric_sql_model(
        &self,
        metric_sql_model_id: uuid::Uuid,
        params: GetMetricSQLModelOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO,
        datadog::Error<GetMetricSQLModelError>,
    > {
        match self
            .get_metric_sql_model_with_http_info(metric_sql_model_id, params)
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

    /// Get a metric SQL model. Returns a single model by its ID for the organization, including its subject types, measures and properties.
    pub async fn get_metric_sql_model_with_http_info(
        &self,
        metric_sql_model_id: uuid::Uuid,
        params: GetMetricSQLModelOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO>,
        datadog::Error<GetMetricSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_metric_sql_model";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-sql-models/{metric_sql_model_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_sql_model_id = datadog::urlencode(metric_sql_model_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTO>(
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
            let local_entity: Option<GetMetricSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a subject type. Returns a single subject type by its ID for the organization.
    pub async fn get_subject_type(
        &self,
        subject_type_id: uuid::Uuid,
        params: GetSubjectTypeOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsSubjectTypeV2DTO,
        datadog::Error<GetSubjectTypeError>,
    > {
        match self
            .get_subject_type_with_http_info(subject_type_id, params)
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

    /// Get a subject type. Returns a single subject type by its ID for the organization.
    pub async fn get_subject_type_with_http_info(
        &self,
        subject_type_id: uuid::Uuid,
        params: GetSubjectTypeOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>,
        datadog::Error<GetSubjectTypeError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_subject_type";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types/{subject_type_id}",
            local_configuration.get_operation_host(local_operation_id),
            subject_type_id = datadog::urlencode(subject_type_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>(
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
            let local_entity: Option<GetSubjectTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List every decision and non-decision metric group attached to an experiment. Metric references are returned in their stored order. An incomplete draft can have no decision group and returns only the groups that exist.
    pub async fn list_experiment_metric_groups(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentMetricGroupV2DTOArray,
        datadog::Error<ListExperimentMetricGroupsError>,
    > {
        match self
            .list_experiment_metric_groups_with_http_info(experiment_id)
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

    /// List every decision and non-decision metric group attached to an experiment. Metric references are returned in their stored order. An incomplete draft can have no decision group and returns only the groups that exist.
    pub async fn list_experiment_metric_groups_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsExperimentMetricGroupV2DTOArray,
        >,
        datadog::Error<ListExperimentMetricGroupsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_experiment_metric_groups";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/metric-groups",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
                crate::datadogV2::model::ExperimentsExperimentMetricGroupV2DTOArray,
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
            let local_entity: Option<ListExperimentMetricGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List draft, published, and archived experiment protocols. Omit filter[status] to return all statuses. Only published protocols can be used to create experiments.
    pub async fn list_experiment_protocols(
        &self,
        params: ListExperimentProtocolsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsPublicProtocolListResponseArray,
        datadog::Error<ListExperimentProtocolsError>,
    > {
        match self.list_experiment_protocols_with_http_info(params).await {
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

    /// List draft, published, and archived experiment protocols. Omit filter[status] to return all statuses. Only published protocols can be used to create experiments.
    pub async fn list_experiment_protocols_with_http_info(
        &self,
        params: ListExperimentProtocolsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsPublicProtocolListResponseArray,
        >,
        datadog::Error<ListExperimentProtocolsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_experiment_protocols";

        // unbox and build optional parameters
        let filter_status = params.filter_status;
        let filter_primary_metric_id = params.filter_primary_metric_id;
        let filter_query = params.filter_query;
        let filter_subject_type_id = params.filter_subject_type_id;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/protocols",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = filter_status {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[status]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_primary_metric_id {
            local_req_builder = local_req_builder
                .query(&[("filter[primary_metric_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_subject_type_id {
            local_req_builder = local_req_builder
                .query(&[("filter[subject_type_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
                crate::datadogV2::model::ExperimentsPublicProtocolListResponseArray,
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
            let local_entity: Option<ListExperimentProtocolsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List experiments. Returns a paginated list of experiments and their structured metadata for the organization. Supports filtering and pagination. Use Get experiment for variants, decision metrics, traffic exposure, and assignment configuration.
    pub async fn list_experiments(
        &self,
        params: ListExperimentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentV2ListDTOArray,
        datadog::Error<ListExperimentsError>,
    > {
        match self.list_experiments_with_http_info(params).await {
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

    /// List experiments. Returns a paginated list of experiments and their structured metadata for the organization. Supports filtering and pagination. Use Get experiment for variants, decision metrics, traffic exposure, and assignment configuration.
    pub async fn list_experiments_with_http_info(
        &self,
        params: ListExperimentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExperimentV2ListDTOArray>,
        datadog::Error<ListExperimentsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_experiments";

        // unbox and build optional parameters
        let concluded_since = params.concluded_since;
        let created_since = params.created_since;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let protocol_id = params.protocol_id;
        let results_updated_before = params.results_updated_before;
        let results_updated_since = params.results_updated_since;
        let search = params.search;
        let sort = params.sort;
        let status = params.status;
        let tags = params.tags;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = concluded_since {
            local_req_builder = local_req_builder.query(&[(
                "concluded_since",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = created_since {
            local_req_builder = local_req_builder.query(&[(
                "created_since",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = protocol_id {
            for param in local {
                local_req_builder = local_req_builder.query(&[("protocol_id", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = results_updated_before {
            local_req_builder = local_req_builder.query(&[(
                "results_updated_before",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = results_updated_since {
            local_req_builder = local_req_builder.query(&[(
                "results_updated_since",
                &local_query_param.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            )]);
        };
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local) = status {
            for param in local {
                local_req_builder = local_req_builder.query(&[("status", &param.to_string())]);
            }
        };
        if let Some(ref local) = tags {
            for param in local {
                local_req_builder = local_req_builder.query(&[("tags", &param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsExperimentV2ListDTOArray>(
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
            let local_entity: Option<ListExperimentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List exposure SQL models. Returns a paginated list of the SQL models that experiment exposures are read from for the organization. Models maintained by Datadog are not included: they cannot be modified and cannot be used as an experiment's assignment source.
    pub async fn list_exposure_sql_models(
        &self,
        params: ListExposureSQLModelsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOArray,
        datadog::Error<ListExposureSQLModelsError>,
    > {
        match self.list_exposure_sql_models_with_http_info(params).await {
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

    /// List exposure SQL models. Returns a paginated list of the SQL models that experiment exposures are read from for the organization. Models maintained by Datadog are not included: they cannot be modified and cannot be used as an experiment's assignment source.
    pub async fn list_exposure_sql_models_with_http_info(
        &self,
        params: ListExposureSQLModelsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOArray>,
        datadog::Error<ListExposureSQLModelsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_exposure_sql_models";

        // unbox and build optional parameters
        let include = params.include;
        let include_archived = params.include_archived;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let search = params.search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = include_archived {
            local_req_builder =
                local_req_builder.query(&[("include_archived", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
                crate::datadogV2::model::ExperimentsExposureSQLModelV2DTOArray,
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
            let local_entity: Option<ListExposureSQLModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List metric collections for the organization. Collections are reusable ordered metric sets; attaching one to an experiment creates an independent snapshot.
    pub async fn list_metric_collections(
        &self,
        params: ListMetricCollectionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricCollectionV2DTOArray,
        datadog::Error<ListMetricCollectionsError>,
    > {
        match self.list_metric_collections_with_http_info(params).await {
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

    /// List metric collections for the organization. Collections are reusable ordered metric sets; attaching one to an experiment creates an independent snapshot.
    pub async fn list_metric_collections_with_http_info(
        &self,
        params: ListMetricCollectionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricCollectionV2DTOArray>,
        datadog::Error<ListMetricCollectionsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_metric_collections";

        // unbox and build optional parameters
        let include = params.include;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let search = params.search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-collections",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
                crate::datadogV2::model::ExperimentsMetricCollectionV2DTOArray,
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
            let local_entity: Option<ListMetricCollectionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List metric SQL models. Returns a paginated list of the SQL models that metrics are defined on for the organization.
    pub async fn list_metric_sql_models(
        &self,
        params: ListMetricSQLModelsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricSQLModelV2DTOArray,
        datadog::Error<ListMetricSQLModelsError>,
    > {
        match self.list_metric_sql_models_with_http_info(params).await {
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

    /// List metric SQL models. Returns a paginated list of the SQL models that metrics are defined on for the organization.
    pub async fn list_metric_sql_models_with_http_info(
        &self,
        params: ListMetricSQLModelsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTOArray>,
        datadog::Error<ListMetricSQLModelsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_metric_sql_models";

        // unbox and build optional parameters
        let include = params.include;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-sql-models",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricSQLModelV2DTOArray>(
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
            let local_entity: Option<ListMetricSQLModelsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List metrics. Returns a paginated list of the experiment metrics defined for the organization.
    pub async fn list_metrics(
        &self,
        params: ListMetricsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricV2DTOArray,
        datadog::Error<ListMetricsError>,
    > {
        match self.list_metrics_with_http_info(params).await {
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

    /// List metrics. Returns a paginated list of the experiment metrics defined for the organization.
    pub async fn list_metrics_with_http_info(
        &self,
        params: ListMetricsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricV2DTOArray>,
        datadog::Error<ListMetricsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_metrics";

        // unbox and build optional parameters
        let include = params.include;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let search = params.search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metrics",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricV2DTOArray>(
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
            let local_entity: Option<ListMetricsError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List subject types. Returns a paginated list of the subject types defined for the organization.
    pub async fn list_subject_types(
        &self,
        params: ListSubjectTypesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsSubjectTypeV2DTOArray,
        datadog::Error<ListSubjectTypesError>,
    > {
        match self.list_subject_types_with_http_info(params).await {
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

    /// List subject types. Returns a paginated list of the subject types defined for the organization.
    pub async fn list_subject_types_with_http_info(
        &self,
        params: ListSubjectTypesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsSubjectTypeV2DTOArray>,
        datadog::Error<ListSubjectTypesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_subject_types";

        // unbox and build optional parameters
        let include = params.include;
        let page_limit = params.page_limit;
        let page_offset = params.page_offset;
        let search = params.search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsSubjectTypeV2DTOArray>(
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
            let local_entity: Option<ListSubjectTypesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update mutable experiment fields. State and protocol restrictions apply.
    ///
    /// **PATCH behavior**
    ///
    /// - Omitted fields stay unchanged, including fields inside `datadog_flag_configuration`.
    /// - Supplied `tags`, `teams`, `related_links`, `decision_metrics`, and `variants` replace their stored lists.
    /// - Validation can return several field errors before saving any changes. The response is HTTP 400 if any error
    ///   concerns invalid input. It is HTTP 409 if all errors concern state or protocol conflicts.
    ///
    /// **Result refreshes**
    ///
    /// This endpoint does not start a pipeline run. `meta.needs_pipeline_refresh` states whether the edit requires a
    /// run. When true, POST to `meta.refresh_endpoint` after finishing your edits. Its `full_refresh` query parameter
    /// selects the run type.
    ///
    /// Keep refresh requirements across edits. A later false value does not clear an earlier requirement. Any
    /// `full_refresh=true` requirement takes priority.
    ///
    /// After start, STATIC and STEPS exposure changes for warehouse experiments without a Datadog flag attempt to
    /// recalculate stored results. Changes to decision metrics or the control variant also attempt recalculation when
    /// results exist. If stored data is insufficient or recalculation fails, the edit stays saved and
    /// `meta.needs_pipeline_refresh` is true.
    ///
    /// **Exposure rules**
    ///
    /// - Draft experiments can replace the full STATIC or STEPS plan through `traffic_exposure`.
    /// - Warehouse steps start at `assignments_start_date` and can have different durations.
    /// - Running warehouse experiments can replace step fractions, durations, and exposure mode. Retained variant
    ///   weights cannot change through this API. You can send unchanged values again.
    /// - After a warehouse experiment ends, configuration replacement supports only STATIC fraction changes.
    /// - New Datadog plans have at most five steps. The first fraction must be positive. All steps except the last
    ///   have equal durations. Durations exclude pauses.
    /// - The last step has a null duration. Its fraction stays in effect until assignment ends.
    /// - After start, use the experiment UI to change traffic exposure for experiments linked to a Datadog flag.
    ///
    /// **Metadata**
    ///
    /// `structured_metadata` updates fields by `field_key`. Use `freetext_value: ""` or `enum_values: []` to clear an
    /// optional field. Omitted fields stay unchanged. A null or empty `structured_metadata` attribute makes no
    /// change.
    ///
    /// **Flag changes**
    ///
    /// Before start, a Datadog update creates or edits the saved draft allocation. To add or replace a flag, send
    /// `variants` and `traffic_exposure`. Inside `datadog_flag_configuration`, send `feature_flag_id`,
    /// `environment_id`, `targeting_rules`, and `entry_point`. Use `targeting_rules: []` and `entry_point: null` when
    /// unused.
    ///
    /// To replace a flag, also set `reset_on_feature_flag_change: true` inside that object. The server deletes the
    /// old draft and creates a new one in the same transaction. The response includes a
    /// `datadog_flag_configuration_reset` warning in `meta.warnings`.
    ///
    /// Set `datadog_flag_configuration: null` to delete the draft allocation. This also clears the experiment's flag
    /// association, variants, assignment sources, and entry point. The experiment remains.
    ///
    /// Flag replacement and removal require a draft experiment without warehouse exposure. These actions do not
    /// convert hybrid experiments.
    pub async fn patch_experiment(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchExperimentV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsPatchExperimentV2Response,
        datadog::Error<PatchExperimentError>,
    > {
        match self
            .patch_experiment_with_http_info(experiment_id, body)
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

    /// Update mutable experiment fields. State and protocol restrictions apply.
    ///
    /// **PATCH behavior**
    ///
    /// - Omitted fields stay unchanged, including fields inside `datadog_flag_configuration`.
    /// - Supplied `tags`, `teams`, `related_links`, `decision_metrics`, and `variants` replace their stored lists.
    /// - Validation can return several field errors before saving any changes. The response is HTTP 400 if any error
    ///   concerns invalid input. It is HTTP 409 if all errors concern state or protocol conflicts.
    ///
    /// **Result refreshes**
    ///
    /// This endpoint does not start a pipeline run. `meta.needs_pipeline_refresh` states whether the edit requires a
    /// run. When true, POST to `meta.refresh_endpoint` after finishing your edits. Its `full_refresh` query parameter
    /// selects the run type.
    ///
    /// Keep refresh requirements across edits. A later false value does not clear an earlier requirement. Any
    /// `full_refresh=true` requirement takes priority.
    ///
    /// After start, STATIC and STEPS exposure changes for warehouse experiments without a Datadog flag attempt to
    /// recalculate stored results. Changes to decision metrics or the control variant also attempt recalculation when
    /// results exist. If stored data is insufficient or recalculation fails, the edit stays saved and
    /// `meta.needs_pipeline_refresh` is true.
    ///
    /// **Exposure rules**
    ///
    /// - Draft experiments can replace the full STATIC or STEPS plan through `traffic_exposure`.
    /// - Warehouse steps start at `assignments_start_date` and can have different durations.
    /// - Running warehouse experiments can replace step fractions, durations, and exposure mode. Retained variant
    ///   weights cannot change through this API. You can send unchanged values again.
    /// - After a warehouse experiment ends, configuration replacement supports only STATIC fraction changes.
    /// - New Datadog plans have at most five steps. The first fraction must be positive. All steps except the last
    ///   have equal durations. Durations exclude pauses.
    /// - The last step has a null duration. Its fraction stays in effect until assignment ends.
    /// - After start, use the experiment UI to change traffic exposure for experiments linked to a Datadog flag.
    ///
    /// **Metadata**
    ///
    /// `structured_metadata` updates fields by `field_key`. Use `freetext_value: ""` or `enum_values: []` to clear an
    /// optional field. Omitted fields stay unchanged. A null or empty `structured_metadata` attribute makes no
    /// change.
    ///
    /// **Flag changes**
    ///
    /// Before start, a Datadog update creates or edits the saved draft allocation. To add or replace a flag, send
    /// `variants` and `traffic_exposure`. Inside `datadog_flag_configuration`, send `feature_flag_id`,
    /// `environment_id`, `targeting_rules`, and `entry_point`. Use `targeting_rules: []` and `entry_point: null` when
    /// unused.
    ///
    /// To replace a flag, also set `reset_on_feature_flag_change: true` inside that object. The server deletes the
    /// old draft and creates a new one in the same transaction. The response includes a
    /// `datadog_flag_configuration_reset` warning in `meta.warnings`.
    ///
    /// Set `datadog_flag_configuration: null` to delete the draft allocation. This also clears the experiment's flag
    /// association, variants, assignment sources, and entry point. The experiment remains.
    ///
    /// Flag replacement and removal require a draft experiment without warehouse exposure. These actions do not
    /// convert hybrid experiments.
    pub async fn patch_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchExperimentV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsPatchExperimentV2Response>,
        datadog::Error<PatchExperimentError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.patch_experiment";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
                crate::datadogV2::model::ExperimentsPatchExperimentV2Response,
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
            let local_entity: Option<PatchExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update mutable fields on a subject type. Only the fields present in the body are changed. Any subject type can be updated, including the organization's default one, but the is_default flag itself is read-only here: which subject type is the default cannot be changed through this endpoint.
    pub async fn patch_subject_type(
        &self,
        subject_type_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchSubjectTypeV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsSubjectTypeV2DTO,
        datadog::Error<PatchSubjectTypeError>,
    > {
        match self
            .patch_subject_type_with_http_info(subject_type_id, body)
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

    /// Update mutable fields on a subject type. Only the fields present in the body are changed. Any subject type can be updated, including the organization's default one, but the is_default flag itself is read-only here: which subject type is the default cannot be changed through this endpoint.
    pub async fn patch_subject_type_with_http_info(
        &self,
        subject_type_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchSubjectTypeV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>,
        datadog::Error<PatchSubjectTypeError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.patch_subject_type";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types/{subject_type_id}",
            local_configuration.get_operation_host(local_operation_id),
            subject_type_id = datadog::urlencode(subject_type_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsSubjectTypeV2DTO>(
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
            let local_entity: Option<PatchSubjectTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Request a results refresh for one experiment. HTTP 202 confirms acceptance, not completed results. The response identifies the experiment and does not include a job ID. Read experiment results to check freshness. A request while a refresh is queued or running returns 409. After completion, another request can start another refresh.
    pub async fn refresh_experiment_results(
        &self,
        experiment_id: uuid::Uuid,
        params: RefreshExperimentResultsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTO,
        datadog::Error<RefreshExperimentResultsError>,
    > {
        match self
            .refresh_experiment_results_with_http_info(experiment_id, params)
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

    /// Request a results refresh for one experiment. HTTP 202 confirms acceptance, not completed results. The response identifies the experiment and does not include a job ID. Read experiment results to check freshness. A request while a refresh is queued or running returns 409. After completion, another request can start another refresh.
    pub async fn refresh_experiment_results_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        params: RefreshExperimentResultsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTO>,
        datadog::Error<RefreshExperimentResultsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.refresh_experiment_results";

        // unbox and build optional parameters
        let full_refresh = params.full_refresh;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/results/refresh",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = full_refresh {
            local_req_builder =
                local_req_builder.query(&[("full_refresh", &local_query_param.to_string())]);
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
                crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTO,
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
            let local_entity: Option<RefreshExperimentResultsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Trigger a results refresh across the organization's active experiments. Returns the count of experiments whose refresh was triggered (meta.experiments_updated) plus a per-experiment breakdown of what happened to each (meta.results).
    pub async fn refresh_experiment_results_for_org(
        &self,
        params: RefreshExperimentResultsForOrgOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTOArray,
        datadog::Error<RefreshExperimentResultsForOrgError>,
    > {
        match self
            .refresh_experiment_results_for_org_with_http_info(params)
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

    /// Trigger a results refresh across the organization's active experiments. Returns the count of experiments whose refresh was triggered (meta.experiments_updated) plus a per-experiment breakdown of what happened to each (meta.results).
    pub async fn refresh_experiment_results_for_org_with_http_info(
        &self,
        params: RefreshExperimentResultsForOrgOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTOArray,
        >,
        datadog::Error<RefreshExperimentResultsForOrgError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.refresh_experiment_results_for_org";

        // unbox and build optional parameters
        let full_refresh = params.full_refresh;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/results/refresh",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = full_refresh {
            local_req_builder =
                local_req_builder.query(&[("full_refresh", &local_query_param.to_string())]);
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
                crate::datadogV2::model::ExperimentsRefreshExperimentResultsV2DTOArray,
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
            let local_entity: Option<RefreshExperimentResultsForOrgError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Make this subject type the organization's default. Experiment creation uses the default when the request names no subject type. Promoting one subject type demotes the previous default in the same transaction, so the organization always has exactly one. The call is idempotent: promoting the current default succeeds and changes nothing. There is no matching demote, because an organization cannot have no default; promote a different subject type instead.
    pub async fn set_default_subject_type(
        &self,
        subject_type_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<SetDefaultSubjectTypeError>> {
        match self
            .set_default_subject_type_with_http_info(subject_type_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Make this subject type the organization's default. Experiment creation uses the default when the request names no subject type. Promoting one subject type demotes the previous default in the same transaction, so the organization always has exactly one. The call is idempotent: promoting the current default succeeds and changes nothing. There is no matching demote, because an organization cannot have no default; promote a different subject type instead.
    pub async fn set_default_subject_type_with_http_info(
        &self,
        subject_type_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<SetDefaultSubjectTypeError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.set_default_subject_type";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/subject-types/{subject_type_id}/default",
            local_configuration.get_operation_host(local_operation_id),
            subject_type_id = datadog::urlencode(subject_type_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<SetDefaultSubjectTypeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Start an experiment. The experiment is started exactly as it is configured; this endpoint accepts no attributes, and a request body carrying any is rejected rather than ignored. Set the run window, duration, or variants with PATCH /api/v2/experiments/{experiment_id} before starting. An unconfigured draft returns HTTP 409. Configure either warehouse_exposure_configuration or datadog_flag_configuration, plus the required experiment fields, before starting. Start validation errors can include meta.configuration_pointer to identify a field on the experiment to correct. For a flag-backed experiment this enables the linked feature flag's environment, clears any stored variant override on it, and starts the allocation's rollout. The request is idempotent: an experiment that is already running or ready for a decision still returns 204, so a retry after a timeout is safe. One exception: an experiment scheduled to start is accepted only when it is backed by your own feature flag; a Datadog-flag experiment in that state returns 409 because its stored state and flag allocation disagree. Cancel and conclude are not idempotent and return 409 when repeated.
    pub async fn start_experiment(
        &self,
        experiment_id: uuid::Uuid,
        params: StartExperimentOptionalParams,
    ) -> Result<(), datadog::Error<StartExperimentError>> {
        match self
            .start_experiment_with_http_info(experiment_id, params)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Start an experiment. The experiment is started exactly as it is configured; this endpoint accepts no attributes, and a request body carrying any is rejected rather than ignored. Set the run window, duration, or variants with PATCH /api/v2/experiments/{experiment_id} before starting. An unconfigured draft returns HTTP 409. Configure either warehouse_exposure_configuration or datadog_flag_configuration, plus the required experiment fields, before starting. Start validation errors can include meta.configuration_pointer to identify a field on the experiment to correct. For a flag-backed experiment this enables the linked feature flag's environment, clears any stored variant override on it, and starts the allocation's rollout. The request is idempotent: an experiment that is already running or ready for a decision still returns 204, so a retry after a timeout is safe. One exception: an experiment scheduled to start is accepted only when it is backed by your own feature flag; a Datadog-flag experiment in that state returns 409 because its stored state and flag allocation disagree. Cancel and conclude are not idempotent and return 409 when repeated.
    pub async fn start_experiment_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        params: StartExperimentOptionalParams,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<StartExperimentError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.start_experiment";

        // unbox and build optional parameters
        let body = params.body;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/start",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
            let local_entity: Option<StartExperimentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Unarchive an exposure SQL model. Restores an archived model to the default list.
    pub async fn unarchive_exposure_sql_model(
        &self,
        exposure_sql_model_id: uuid::Uuid,
    ) -> Result<(), datadog::Error<UnarchiveExposureSQLModelError>> {
        match self
            .unarchive_exposure_sql_model_with_http_info(exposure_sql_model_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Unarchive an exposure SQL model. Restores an archived model to the default list.
    pub async fn unarchive_exposure_sql_model_with_http_info(
        &self,
        exposure_sql_model_id: uuid::Uuid,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UnarchiveExposureSQLModelError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.unarchive_exposure_sql_model";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models/{exposure_sql_model_id}/unarchive",
            local_configuration.get_operation_host(local_operation_id),
            exposure_sql_model_id = datadog::urlencode(exposure_sql_model_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            let local_entity: Option<UnarchiveExposureSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update selected statistical analysis settings. Omitted attributes remain unchanged and nullable attributes can be cleared with null. Protocol-locked settings cannot be changed.
    pub async fn update_experiment_analysis_plan_attributes(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponse,
        datadog::Error<UpdateExperimentAnalysisPlanAttributesError>,
    > {
        match self
            .update_experiment_analysis_plan_attributes_with_http_info(experiment_id, body)
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

    /// Update selected statistical analysis settings. Omitted attributes remain unchanged and nullable attributes can be cleared with null. Protocol-locked settings cannot be changed.
    pub async fn update_experiment_analysis_plan_attributes_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsAnalysisPlanWriteV2Request,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponse,
        >,
        datadog::Error<UpdateExperimentAnalysisPlanAttributesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_experiment_analysis_plan_attributes";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/analysis-plan",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string())
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
                crate::datadogV2::model::ExperimentsAnalysisPlanV2MutationResponse,
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
            let local_entity: Option<UpdateExperimentAnalysisPlanAttributesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a non-decision metric group. Omitted attributes are unchanged; a supplied metrics array is the complete ordered replacement. Decision groups remain managed through the experiment resource. This operation does not synchronously recompute results.
    pub async fn update_experiment_metric_group(
        &self,
        experiment_id: uuid::Uuid,
        metric_group_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        datadog::Error<UpdateExperimentMetricGroupError>,
    > {
        match self
            .update_experiment_metric_group_with_http_info(experiment_id, metric_group_id, body)
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

    /// Update a non-decision metric group. Omitted attributes are unchanged; a supplied metrics array is the complete ordered replacement. Decision groups remain managed through the experiment resource. This operation does not synchronously recompute results.
    pub async fn update_experiment_metric_group_with_http_info(
        &self,
        experiment_id: uuid::Uuid,
        metric_group_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchExperimentMetricGroupV2Request,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
        >,
        datadog::Error<UpdateExperimentMetricGroupError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_experiment_metric_group";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/{experiment_id}/metric-groups/{metric_group_id}",
            local_configuration.get_operation_host(local_operation_id),
            experiment_id = datadog::urlencode(experiment_id.to_string()),
            metric_group_id = datadog::urlencode(metric_group_id.to_string())
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
                crate::datadogV2::model::ExperimentsExperimentMetricGroupMutationV2,
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
            let local_entity: Option<UpdateExperimentMetricGroupError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace an exposure SQL model. This is a full replacement and is destructive: any subject type or property not present in the body is deleted, and properties are matched on name, column_name and column_type together, so changing one of those replaces the property rather than editing it. Send the complete set you want to keep. Anything removed is listed under meta.removed_subject_type_ids and meta.removed_property_names in the response. The warehouse connection is not settable and is left as stored.
    pub async fn update_exposure_sql_model(
        &self,
        exposure_sql_model_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2Request,
        params: UpdateExposureSQLModelOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ExperimentsUpdateExposureSQLModelV2Response,
        datadog::Error<UpdateExposureSQLModelError>,
    > {
        match self
            .update_exposure_sql_model_with_http_info(exposure_sql_model_id, body, params)
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

    /// Replace an exposure SQL model. This is a full replacement and is destructive: any subject type or property not present in the body is deleted, and properties are matched on name, column_name and column_type together, so changing one of those replaces the property rather than editing it. Send the complete set you want to keep. Anything removed is listed under meta.removed_subject_type_ids and meta.removed_property_names in the response. The warehouse connection is not settable and is left as stored.
    pub async fn update_exposure_sql_model_with_http_info(
        &self,
        exposure_sql_model_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateExposureSQLModelV2Request,
        params: UpdateExposureSQLModelOptionalParams,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsUpdateExposureSQLModelV2Response,
        >,
        datadog::Error<UpdateExposureSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_exposure_sql_model";

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/exposure-sql-models/{exposure_sql_model_id}",
            local_configuration.get_operation_host(local_operation_id),
            exposure_sql_model_id = datadog::urlencode(exposure_sql_model_id.to_string())
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        if let Some(ref local) = include {
            for param in local {
                local_req_builder = local_req_builder.query(&[("include", &param.to_string())]);
            }
        };

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
                crate::datadogV2::model::ExperimentsUpdateExposureSQLModelV2Response,
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
            let local_entity: Option<UpdateExposureSQLModelError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update a metric. Certified metrics are read-only through this endpoint. This is a partial update: every attribute is optional and an omitted attribute keeps its stored value, so a body carrying only the fields being changed is enough. `guardrail_cutoff_threshold` is nullable -- send null to clear it, omit it to leave it alone. Omitting the aggregation leaves the metric's definition untouched; supplying one replaces it wholesale, and the metric's type is re-derived from the shape supplied. Property filters use property_id or measure_id UUIDs from the aggregation's data source. Attributes that are computed rather than stored (short_id, metric_type, certified_at, experiment_count, created_at, updated_at) are rejected rather than ignored, so a body copied from GET must have them removed. The is_certified attribute is rejected. Certification cannot be changed through this endpoint.
    pub async fn update_metric(
        &self,
        metric_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsUpdateMetricV2Request,
    ) -> Result<crate::datadogV2::model::ExperimentsMetricV2DTO, datadog::Error<UpdateMetricError>>
    {
        match self.update_metric_with_http_info(metric_id, body).await {
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

    /// Update a metric. Certified metrics are read-only through this endpoint. This is a partial update: every attribute is optional and an omitted attribute keeps its stored value, so a body carrying only the fields being changed is enough. `guardrail_cutoff_threshold` is nullable -- send null to clear it, omit it to leave it alone. Omitting the aggregation leaves the metric's definition untouched; supplying one replaces it wholesale, and the metric's type is re-derived from the shape supplied. Property filters use property_id or measure_id UUIDs from the aggregation's data source. Attributes that are computed rather than stored (short_id, metric_type, certified_at, experiment_count, created_at, updated_at) are rejected rather than ignored, so a body copied from GET must have them removed. The is_certified attribute is rejected. Certification cannot be changed through this endpoint.
    pub async fn update_metric_with_http_info(
        &self,
        metric_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsUpdateMetricV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricV2DTO>,
        datadog::Error<UpdateMetricError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_metric";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metrics/{metric_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_id = datadog::urlencode(metric_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricV2DTO>(
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
            let local_entity: Option<UpdateMetricError> = serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update metric collection.
    pub async fn update_metric_collection(
        &self,
        metric_collection_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchMetricCollectionV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsMetricCollectionV2DTO,
        datadog::Error<UpdateMetricCollectionError>,
    > {
        match self
            .update_metric_collection_with_http_info(metric_collection_id, body)
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

    /// Update metric collection.
    pub async fn update_metric_collection_with_http_info(
        &self,
        metric_collection_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsPatchMetricCollectionV2Request,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>,
        datadog::Error<UpdateMetricCollectionError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_metric_collection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-collections/{metric_collection_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_collection_id = datadog::urlencode(metric_collection_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::ExperimentsMetricCollectionV2DTO>(
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
            let local_entity: Option<UpdateMetricCollectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Replace a metric SQL model. This is a destructive full replace: subject types, customer-defined measures, and properties absent from the body are deleted, so send the complete set. Properties are matched by name; changing a property's column, type, or description preserves its ID. column_type is required for every measure and property. Removing a measure or property that an active metric references returns 409 Conflict. Server-generated IDs returned by GET are read-only and can be left in a replayed body. The response reports removals in meta.deleted_subject_types, meta.deleted_measures, and meta.deleted_properties. Certification is read-only. Certified models cannot be replaced through this endpoint.
    pub async fn update_metric_sql_model(
        &self,
        metric_sql_model_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2Request,
    ) -> Result<
        crate::datadogV2::model::ExperimentsUpdateMetricSQLModelV2Response,
        datadog::Error<UpdateMetricSQLModelError>,
    > {
        match self
            .update_metric_sql_model_with_http_info(metric_sql_model_id, body)
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

    /// Replace a metric SQL model. This is a destructive full replace: subject types, customer-defined measures, and properties absent from the body are deleted, so send the complete set. Properties are matched by name; changing a property's column, type, or description preserves its ID. column_type is required for every measure and property. Removing a measure or property that an active metric references returns 409 Conflict. Server-generated IDs returned by GET are read-only and can be left in a replayed body. The response reports removals in meta.deleted_subject_types, meta.deleted_measures, and meta.deleted_properties. Certification is read-only. Certified models cannot be replaced through this endpoint.
    pub async fn update_metric_sql_model_with_http_info(
        &self,
        metric_sql_model_id: uuid::Uuid,
        body: crate::datadogV2::model::ExperimentsCreateMetricSQLModelV2Request,
    ) -> Result<
        datadog::ResponseContent<
            crate::datadogV2::model::ExperimentsUpdateMetricSQLModelV2Response,
        >,
        datadog::Error<UpdateMetricSQLModelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_metric_sql_model";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/experiments/metric-sql-models/{metric_sql_model_id}",
            local_configuration.get_operation_host(local_operation_id),
            metric_sql_model_id = datadog::urlencode(metric_sql_model_id.to_string())
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
                crate::datadogV2::model::ExperimentsUpdateMetricSQLModelV2Response,
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
            let local_entity: Option<UpdateMetricSQLModelError> =
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
