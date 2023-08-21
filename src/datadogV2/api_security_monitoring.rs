// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateSecurityFilterParams is a struct for passing parameters to the method [`CreateSecurityFilter`]
#[derive(Clone, Debug)]
pub struct CreateSecurityFilterParams {
    /* The definition of the new security filter. */
    pub body: SecurityFilterCreateRequest,
}

// CreateSecurityMonitoringRuleParams is a struct for passing parameters to the method [`CreateSecurityMonitoringRule`]
#[derive(Clone, Debug)]
pub struct CreateSecurityMonitoringRuleParams {
    pub body: SecurityMonitoringRuleCreatePayload,
}

// DeleteSecurityFilterParams is a struct for passing parameters to the method [`DeleteSecurityFilter`]
#[derive(Clone, Debug)]
pub struct DeleteSecurityFilterParams {
    /* The ID of the security filter. */
    pub security_filter_id: String,
}

// DeleteSecurityMonitoringRuleParams is a struct for passing parameters to the method [`DeleteSecurityMonitoringRule`]
#[derive(Clone, Debug)]
pub struct DeleteSecurityMonitoringRuleParams {
    /* The ID of the rule. */
    pub rule_id: String,
}

// EditSecurityMonitoringSignalAssigneeParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalAssigneeParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: SecurityMonitoringSignalAssigneeUpdateRequest,
}

// EditSecurityMonitoringSignalIncidentsParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalIncidents`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalIncidentsParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: SecurityMonitoringSignalIncidentsUpdateRequest,
}

// EditSecurityMonitoringSignalStateParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalState`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalStateParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: SecurityMonitoringSignalStateUpdateRequest,
}

// GetFindingParams is a struct for passing parameters to the method [`GetFinding`]
#[derive(Clone, Debug)]
pub struct GetFindingParams {
    /* The ID of the finding. */
    pub finding_id: String,
    /* Return the finding for a given snapshot of time (Unix ms). */
    pub snapshot_timestamp: i64,
}

// GetSecurityFilterParams is a struct for passing parameters to the method [`GetSecurityFilter`]
#[derive(Clone, Debug)]
pub struct GetSecurityFilterParams {
    /* The ID of the security filter. */
    pub security_filter_id: String,
}

// GetSecurityMonitoringRuleParams is a struct for passing parameters to the method [`GetSecurityMonitoringRule`]
#[derive(Clone, Debug)]
pub struct GetSecurityMonitoringRuleParams {
    /* The ID of the rule. */
    pub rule_id: String,
}

// GetSecurityMonitoringSignalParams is a struct for passing parameters to the method [`GetSecurityMonitoringSignal`]
#[derive(Clone, Debug)]
pub struct GetSecurityMonitoringSignalParams {
    /* The ID of the signal. */
    pub signal_id: String,
}

// ListFindingsParams is a struct for passing parameters to the method [`ListFindings`]
#[derive(Clone, Debug)]
pub struct ListFindingsParams {
    /* Limit the number of findings returned. Must be <= 1000. */
    pub page_limit: i64,
    /* Return findings for a given snapshot of time (Unix ms). */
    pub snapshot_timestamp: i64,
    /* Return the next page of findings pointed to by the cursor. */
    pub page_cursor: String,
    /* Return findings that have these associated tags (repeatable). */
    pub filter_tags: String,
    /* Return findings that have changed from pass to fail or vice versa on a specified date (Unix ms) or date range (using comparison operators). */
    pub filter_evaluation_changed_at: String,
    /* Set to `true` to return findings that are muted. Set to `false` to return unmuted findings. */
    pub filter_muted: bool,
    /* Return findings for the specified rule ID. */
    pub filter_rule_id: String,
    /* Return findings for the specified rule. */
    pub filter_rule_name: String,
    /* Return only findings for the specified resource type. */
    pub filter_resource_type: String,
    /* Return findings that were found on a specified date (Unix ms) or date range (using comparison operators). */
    pub filter_discovery_timestamp: String,
    /* Return only `pass` or `fail` findings. */
    pub filter_evaluation: FindingEvaluation,
    /* Return only findings with the specified status. */
    pub filter_status: FindingStatus,
}

// ListSecurityMonitoringRulesParams is a struct for passing parameters to the method [`ListSecurityMonitoringRules`]
#[derive(Clone, Debug)]
pub struct ListSecurityMonitoringRulesParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
}

// ListSecurityMonitoringSignalsParams is a struct for passing parameters to the method [`ListSecurityMonitoringSignals`]
#[derive(Clone, Debug)]
pub struct ListSecurityMonitoringSignalsParams {
    /* The search query for security signals. */
    pub filter_query: String,
    /* The minimum timestamp for requested security signals. */
    pub filter_from: String,
    /* The maximum timestamp for requested security signals. */
    pub filter_to: String,
    /* The order of the security signals in results. */
    pub sort: SecurityMonitoringSignalsSort,
    /* A list of results using the cursor provided in the previous query. */
    pub page_cursor: String,
    /* The maximum number of security signals in the response. */
    pub page_limit: i32,
}

// SearchSecurityMonitoringSignalsParams is a struct for passing parameters to the method [`SearchSecurityMonitoringSignals`]
#[derive(Clone, Debug)]
pub struct SearchSecurityMonitoringSignalsParams {
    pub body: SecurityMonitoringSignalListRequest,
}

// UpdateFindingParams is a struct for passing parameters to the method [`UpdateFinding`]
#[derive(Clone, Debug)]
pub struct UpdateFindingParams {
    /* The ID of the finding. */
    pub finding_id: String,
    /* To mute or unmute a finding, the request body should include at least two attributes: `muted` and `reason`. The allowed reasons depend on whether the finding is being muted or unmuted:
- To mute a finding: `PENDING_FIX`, `FALSE_POSITIVE`, `ACCEPTED_RISK`, `OTHER`.
- To unmute a finding : `NO_PENDING_FIX`, `HUMAN_ERROR`, `NO_LONGER_ACCEPTED_RISK`, `OTHER`.
 */
    pub body: MuteFindingRequest,
}

// UpdateSecurityFilterParams is a struct for passing parameters to the method [`UpdateSecurityFilter`]
#[derive(Clone, Debug)]
pub struct UpdateSecurityFilterParams {
    /* The ID of the security filter. */
    pub security_filter_id: String,
    /* New definition of the security filter. */
    pub body: SecurityFilterUpdateRequest,
}

// UpdateSecurityMonitoringRuleParams is a struct for passing parameters to the method [`UpdateSecurityMonitoringRule`]
#[derive(Clone, Debug)]
pub struct UpdateSecurityMonitoringRuleParams {
    /* The ID of the rule. */
    pub rule_id: String,
    pub body: SecurityMonitoringRuleUpdatePayload,
}




/// CreateSecurityFilterError is a struct for typed errors of method [`CreateSecurityFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityFilterError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSecurityMonitoringRuleError is a struct for typed errors of method [`CreateSecurityMonitoringRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityMonitoringRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityFilterError is a struct for typed errors of method [`DeleteSecurityFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityFilterError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityMonitoringRuleError is a struct for typed errors of method [`DeleteSecurityMonitoringRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityMonitoringRuleError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalIncidentsError is a struct for typed errors of method [`EditSecurityMonitoringSignalIncidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalIncidentsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`EditSecurityMonitoringSignalState`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalStateError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetFindingError is a struct for typed errors of method [`GetFinding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFindingError {
    Status400(JSONAPIErrorResponse),
    Status403(JSONAPIErrorResponse),
    Status404(JSONAPIErrorResponse),
    Status429(JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityFilterError is a struct for typed errors of method [`GetSecurityFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityFilterError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringRuleError is a struct for typed errors of method [`GetSecurityMonitoringRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringRuleError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringSignalError is a struct for typed errors of method [`GetSecurityMonitoringSignal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringSignalError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListFindingsError is a struct for typed errors of method [`ListFindings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFindingsError {
    Status400(JSONAPIErrorResponse),
    Status403(JSONAPIErrorResponse),
    Status404(JSONAPIErrorResponse),
    Status429(JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityFiltersError is a struct for typed errors of method [`ListSecurityFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityFiltersError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringRulesError is a struct for typed errors of method [`ListSecurityMonitoringRules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringRulesError {
    Status400(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringSignalsError is a struct for typed errors of method [`ListSecurityMonitoringSignals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringSignalsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchSecurityMonitoringSignalsError is a struct for typed errors of method [`SearchSecurityMonitoringSignals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSecurityMonitoringSignalsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateFindingError is a struct for typed errors of method [`UpdateFinding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFindingError {
    Status400(JSONAPIErrorResponse),
    Status403(JSONAPIErrorResponse),
    Status404(JSONAPIErrorResponse),
    Status409(JSONAPIErrorResponse),
    Status422(JSONAPIErrorResponse),
    Status429(JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityFilterError is a struct for typed errors of method [`UpdateSecurityFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityFilterError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityMonitoringRuleError is a struct for typed errors of method [`UpdateSecurityMonitoringRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityMonitoringRuleError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}