// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CheckCanDeleteSLOParams is a struct for passing parameters to the method [`CheckCanDeleteSLO`]
#[derive(Clone, Debug)]
pub struct CheckCanDeleteSLOParams {
    /* A comma separated list of the IDs of the service level objectives objects. */
    pub ids: String,
}

// CreateSLOParams is a struct for passing parameters to the method [`CreateSLO`]
#[derive(Clone, Debug)]
pub struct CreateSLOParams {
    /* Service level objective request object. */
    pub body: ServiceLevelObjectiveRequest,
}

// DeleteSLOParams is a struct for passing parameters to the method [`DeleteSLO`]
#[derive(Clone, Debug)]
pub struct DeleteSLOParams {
    /* The ID of the service level objective. */
    pub slo_id: String,
    /* Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor). */
    pub force: String,
}

// DeleteSLOTimeframeInBulkParams is a struct for passing parameters to the method [`DeleteSLOTimeframeInBulk`]
#[derive(Clone, Debug)]
pub struct DeleteSLOTimeframeInBulkParams {
    /* Delete multiple service level objective objects request body. */
    pub body: map[string]Vec<SLOTimeframe>,
}

// GetSLOParams is a struct for passing parameters to the method [`GetSLO`]
#[derive(Clone, Debug)]
pub struct GetSLOParams {
    /* The ID of the service level objective object. */
    pub slo_id: String,
    /* Get the IDs of SLO monitors that reference this SLO. */
    pub with_configured_alert_ids: bool,
}

// GetSLOCorrectionsParams is a struct for passing parameters to the method [`GetSLOCorrections`]
#[derive(Clone, Debug)]
pub struct GetSLOCorrectionsParams {
    /* The ID of the service level objective object. */
    pub slo_id: String,
}

// GetSLOHistoryParams is a struct for passing parameters to the method [`GetSLOHistory`]
#[derive(Clone, Debug)]
pub struct GetSLOHistoryParams {
    /* The ID of the service level objective object. */
    pub slo_id: String,
    /* The `from` timestamp for the query window in epoch seconds. */
    pub from_ts: i64,
    /* The `to` timestamp for the query window in epoch seconds. */
    pub to_ts: i64,
    /* The SLO target. If `target` is passed in, the response will include the remaining error budget and a timeframe value of `custom`. */
    pub target: f64,
    /* Defaults to `true`. If any SLO corrections are applied and this parameter is set to `false`,
then the corrections will not be applied and the SLI values will not be affected. */
    pub apply_correction: bool,
}

// ListSLOsParams is a struct for passing parameters to the method [`ListSLOs`]
#[derive(Clone, Debug)]
pub struct ListSLOsParams {
    /* A comma separated list of the IDs of the service level objectives objects. */
    pub ids: String,
    /* The query string to filter results based on SLO names. */
    pub query: String,
    /* The query string to filter results based on a single SLO tag. */
    pub tags_query: String,
    /* The query string to filter results based on SLO numerator and denominator. */
    pub metrics_query: String,
    /* The number of SLOs to return in the response. */
    pub limit: i64,
    /* The specific offset to use as the beginning of the returned response. */
    pub offset: i64,
}

// SearchSLOParams is a struct for passing parameters to the method [`SearchSLO`]
#[derive(Clone, Debug)]
pub struct SearchSLOParams {
    /* The query string to filter results based on SLO names.
Some examples of queries include `service:<service-name>`
and `<slo-name>`. */
    pub query: String,
    /* The number of files to return in the response `[default=10]`. */
    pub page_size: i64,
    /* The identifier of the first page to return. This parameter is used for the pagination feature `[default=0]`. */
    pub page_number: i64,
    /* Whether or not to return facet information in the response `[default=false]`. */
    pub include_facets: bool,
}

// UpdateSLOParams is a struct for passing parameters to the method [`UpdateSLO`]
#[derive(Clone, Debug)]
pub struct UpdateSLOParams {
    /* The ID of the service level objective object. */
    pub slo_id: String,
    /* The edited service level objective request object. */
    pub body: ServiceLevelObjective,
}




/// CheckCanDeleteSLOError is a struct for typed errors of method [`CheckCanDeleteSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCanDeleteSLOError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    Status409(CheckCanDeleteSLOResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSLOError is a struct for typed errors of method [`CreateSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOError is a struct for typed errors of method [`DeleteSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    Status409(SLODeleteResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOTimeframeInBulkError is a struct for typed errors of method [`DeleteSLOTimeframeInBulk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOTimeframeInBulkError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSLOError is a struct for typed errors of method [`GetSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionsError is a struct for typed errors of method [`GetSLOCorrections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSLOHistoryError is a struct for typed errors of method [`GetSLOHistory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOHistoryError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSLOsError is a struct for typed errors of method [`ListSLOs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchSLOError is a struct for typed errors of method [`SearchSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSLOError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOError is a struct for typed errors of method [`UpdateSLO`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSLOError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}