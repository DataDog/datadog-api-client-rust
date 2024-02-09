// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSecurityFilterParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::create_security_filter`]
#[derive(Clone, Debug)]
pub struct CreateSecurityFilterParams {
    /// The definition of the new security filter.
    pub body: crate::datadogV2::model::SecurityFilterCreateRequest,
}

/// CreateSecurityMonitoringRuleParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::create_security_monitoring_rule`]
#[derive(Clone, Debug)]
pub struct CreateSecurityMonitoringRuleParams {
    pub body: crate::datadogV2::model::SecurityMonitoringRuleCreatePayload,
}

/// DeleteSecurityFilterParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::delete_security_filter`]
#[derive(Clone, Debug)]
pub struct DeleteSecurityFilterParams {
    /// The ID of the security filter.
    pub security_filter_id: String,
}

/// DeleteSecurityMonitoringRuleParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::delete_security_monitoring_rule`]
#[derive(Clone, Debug)]
pub struct DeleteSecurityMonitoringRuleParams {
    /// The ID of the rule.
    pub rule_id: String,
}

/// EditSecurityMonitoringSignalAssigneeParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::edit_security_monitoring_signal_assignee`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalAssigneeParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV2::model::SecurityMonitoringSignalAssigneeUpdateRequest,
}

/// EditSecurityMonitoringSignalIncidentsParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::edit_security_monitoring_signal_incidents`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalIncidentsParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateRequest,
}

/// EditSecurityMonitoringSignalStateParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::edit_security_monitoring_signal_state`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalStateParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateRequest,
}

/// GetFindingParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_finding`]
#[derive(Clone, Debug)]
pub struct GetFindingParams {
    /// The ID of the finding.
    pub finding_id: String,
    /// Return the finding for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
}

/// GetSecurityFilterParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_security_filter`]
#[derive(Clone, Debug)]
pub struct GetSecurityFilterParams {
    /// The ID of the security filter.
    pub security_filter_id: String,
}

/// GetSecurityMonitoringRuleParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_security_monitoring_rule`]
#[derive(Clone, Debug)]
pub struct GetSecurityMonitoringRuleParams {
    /// The ID of the rule.
    pub rule_id: String,
}

/// GetSecurityMonitoringSignalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_security_monitoring_signal`]
#[derive(Clone, Debug)]
pub struct GetSecurityMonitoringSignalParams {
    /// The ID of the signal.
    pub signal_id: String,
}

/// ListFindingsParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_findings`]
#[derive(Clone, Debug)]
pub struct ListFindingsParams {
    /// Limit the number of findings returned. Must be <= 1000.
    pub page_limit: Option<i64>,
    /// Return findings for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
    /// Return the next page of findings pointed to by the cursor.
    pub page_cursor: Option<String>,
    /// Return findings that have these associated tags (repeatable).
    pub filter_tags: Option<String>,
    /// Return findings that have changed from pass to fail or vice versa on a specified date (Unix ms) or date range (using comparison operators).
    pub filter_evaluation_changed_at: Option<String>,
    /// Set to `true` to return findings that are muted. Set to `false` to return unmuted findings.
    pub filter_muted: Option<bool>,
    /// Return findings for the specified rule ID.
    pub filter_rule_id: Option<String>,
    /// Return findings for the specified rule.
    pub filter_rule_name: Option<String>,
    /// Return only findings for the specified resource type.
    pub filter_resource_type: Option<String>,
    /// Return findings that were found on a specified date (Unix ms) or date range (using comparison operators).
    pub filter_discovery_timestamp: Option<String>,
    /// Return only `pass` or `fail` findings.
    pub filter_evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// Return only findings with the specified status.
    pub filter_status: Option<crate::datadogV2::model::FindingStatus>,
}

/// ListSecurityMonitoringRulesParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_rules`]
#[derive(Clone, Debug)]
pub struct ListSecurityMonitoringRulesParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
}

/// ListSecurityMonitoringSignalsParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_signals`]
#[derive(Clone, Debug)]
pub struct ListSecurityMonitoringSignalsParams {
    /// The search query for security signals.
    pub filter_query: Option<String>,
    /// The minimum timestamp for requested security signals.
    pub filter_from: Option<String>,
    /// The maximum timestamp for requested security signals.
    pub filter_to: Option<String>,
    /// The order of the security signals in results.
    pub sort: Option<crate::datadogV2::model::SecurityMonitoringSignalsSort>,
    /// A list of results using the cursor provided in the previous query.
    pub page_cursor: Option<String>,
    /// The maximum number of security signals in the response.
    pub page_limit: Option<i32>,
}

/// MuteFindingsParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::mute_findings`]
#[derive(Clone, Debug)]
pub struct MuteFindingsParams {
    /// ### Attributes
    ///
    /// All findings are updated with the same attributes. The request body must include at least two attributes: `muted` and `reason`.
    /// The allowed reasons depend on whether the finding is being muted or unmuted:
    ///   - To mute a finding: `PENDING_FIX`, `FALSE_POSITIVE`, `ACCEPTED_RISK`, `OTHER`.
    ///   - To unmute a finding : `NO_PENDING_FIX`, `HUMAN_ERROR`, `NO_LONGER_ACCEPTED_RISK`, `OTHER`.
    ///
    /// ### Meta
    ///
    /// The request body must include a list of the finding IDs to be updated.
    ///
    pub body: crate::datadogV2::model::BulkMuteFindingsRequest,
}

/// SearchSecurityMonitoringSignalsParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::search_security_monitoring_signals`]
#[derive(Clone, Debug)]
pub struct SearchSecurityMonitoringSignalsParams {
    pub body: Option<Option<crate::datadogV2::model::SecurityMonitoringSignalListRequest>>,
}

/// UpdateSecurityFilterParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::update_security_filter`]
#[derive(Clone, Debug)]
pub struct UpdateSecurityFilterParams {
    /// The ID of the security filter.
    pub security_filter_id: String,
    /// New definition of the security filter.
    pub body: crate::datadogV2::model::SecurityFilterUpdateRequest,
}

/// UpdateSecurityMonitoringRuleParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::update_security_monitoring_rule`]
#[derive(Clone, Debug)]
pub struct UpdateSecurityMonitoringRuleParams {
    /// The ID of the rule.
    pub rule_id: String,
    pub body: crate::datadogV2::model::SecurityMonitoringRuleUpdatePayload,
}

/// CreateSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::create_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityFilterError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::create_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSecurityMonitoringRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::delete_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSecurityMonitoringRuleError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_assignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalIncidentsError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_incidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalIncidentsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalStateError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetFindingError is a struct for typed errors of method [`SecurityMonitoringAPI::get_finding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFindingError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringRuleError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSecurityMonitoringSignalError is a struct for typed errors of method [`SecurityMonitoringAPI::get_security_monitoring_signal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSecurityMonitoringSignalError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListFindingsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_findings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFindingsError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSecurityFiltersError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityFiltersError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringRulesError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_monitoring_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringRulesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSecurityMonitoringSignalsError is a struct for typed errors of method [`SecurityMonitoringAPI::list_security_monitoring_signals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSecurityMonitoringSignalsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// MuteFindingsError is a struct for typed errors of method [`SecurityMonitoringAPI::mute_findings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MuteFindingsError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status422(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchSecurityMonitoringSignalsError is a struct for typed errors of method [`SecurityMonitoringAPI::search_security_monitoring_signals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchSecurityMonitoringSignalsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityFilterError is a struct for typed errors of method [`SecurityMonitoringAPI::update_security_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityFilterError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSecurityMonitoringRuleError is a struct for typed errors of method [`SecurityMonitoringAPI::update_security_monitoring_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSecurityMonitoringRuleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SecurityMonitoringAPI {
    config: configuration::Configuration,
}

impl Default for SecurityMonitoringAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SecurityMonitoringAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn create_security_filter(
        &self,
        params: CreateSecurityFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<CreateSecurityFilterError>,
    > {
        match self.create_security_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn create_security_filter_with_http_info(
        &self,
        params: CreateSecurityFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<CreateSecurityFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a detection rule.
    pub async fn create_security_monitoring_rule(
        &self,
        params: CreateSecurityMonitoringRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<CreateSecurityMonitoringRuleError>,
    > {
        match self
            .create_security_monitoring_rule_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a detection rule.
    pub async fn create_security_monitoring_rule_with_http_info(
        &self,
        params: CreateSecurityMonitoringRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<CreateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityMonitoringRuleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a specific security filter.
    pub async fn delete_security_filter(
        &self,
        params: DeleteSecurityFilterParams,
    ) -> Result<Option<()>, Error<DeleteSecurityFilterError>> {
        match self.delete_security_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific security filter.
    pub async fn delete_security_filter_with_http_info(
        &self,
        params: DeleteSecurityFilterParams,
    ) -> Result<ResponseContent<()>, Error<DeleteSecurityFilterError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let security_filter_id = params.security_filter_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.base_path,
            security_filter_id = urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an existing rule. Default rules cannot be deleted.
    pub async fn delete_security_monitoring_rule(
        &self,
        params: DeleteSecurityMonitoringRuleParams,
    ) -> Result<Option<()>, Error<DeleteSecurityMonitoringRuleError>> {
        match self
            .delete_security_monitoring_rule_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing rule. Default rules cannot be deleted.
    pub async fn delete_security_monitoring_rule_with_http_info(
        &self,
        params: DeleteSecurityMonitoringRuleParams,
    ) -> Result<ResponseContent<()>, Error<DeleteSecurityMonitoringRuleError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let rule_id = params.rule_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee(
        &self,
        params: EditSecurityMonitoringSignalAssigneeParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        match self
            .edit_security_monitoring_signal_assignee_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee_with_http_info(
        &self,
        params: EditSecurityMonitoringSignalAssigneeParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/assignee",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalAssigneeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Change the related incidents for a security signal.
    pub async fn edit_security_monitoring_signal_incidents(
        &self,
        params: EditSecurityMonitoringSignalIncidentsParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        match self
            .edit_security_monitoring_signal_incidents_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the related incidents for a security signal.
    pub async fn edit_security_monitoring_signal_incidents_with_http_info(
        &self,
        params: EditSecurityMonitoringSignalIncidentsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/incidents",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalIncidentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state(
        &self,
        params: EditSecurityMonitoringSignalStateParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        match self
            .edit_security_monitoring_signal_state_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state_with_http_info(
        &self,
        params: EditSecurityMonitoringSignalStateParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}/state",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalStateError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns a single finding with message and resource configuration.
    pub async fn get_finding(
        &self,
        params: GetFindingParams,
    ) -> Result<Option<crate::datadogV2::model::GetFindingResponse>, Error<GetFindingError>> {
        match self.get_finding_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a single finding with message and resource configuration.
    pub async fn get_finding_with_http_info(
        &self,
        params: GetFindingParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::GetFindingResponse>, Error<GetFindingError>>
    {
        let operation_id = "v2.get_finding".to_string();
        if self.config.is_unstable_operation_enabled(&operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_finding' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_configuration = &self.config;

        // unbox and build parameters
        let finding_id = params.finding_id;
        let snapshot_timestamp = params.snapshot_timestamp;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings/{finding_id}",
            local_configuration.base_path,
            finding_id = urlencode(finding_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_str.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::GetFindingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetFindingError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the details of a specific security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn get_security_filter(
        &self,
        params: GetSecurityFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<GetSecurityFilterError>,
    > {
        match self.get_security_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the details of a specific security filter.
    ///
    /// See the [security filter guide](<https://docs.datadoghq.com/security_platform/guide/how-to-setup-security-filters-using-security-monitoring-api/>)
    /// for more examples.
    pub async fn get_security_filter_with_http_info(
        &self,
        params: GetSecurityFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<GetSecurityFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let security_filter_id = params.security_filter_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.base_path,
            security_filter_id = urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a rule's details.
    pub async fn get_security_monitoring_rule(
        &self,
        params: GetSecurityMonitoringRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<GetSecurityMonitoringRuleError>,
    > {
        match self
            .get_security_monitoring_rule_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a rule's details.
    pub async fn get_security_monitoring_rule_with_http_info(
        &self,
        params: GetSecurityMonitoringRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<GetSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let rule_id = params.rule_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityMonitoringRuleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a signal's details.
    pub async fn get_security_monitoring_signal(
        &self,
        params: GetSecurityMonitoringSignalParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalResponse>,
        Error<GetSecurityMonitoringSignalError>,
    > {
        match self
            .get_security_monitoring_signal_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a signal's details.
    pub async fn get_security_monitoring_signal_with_http_info(
        &self,
        params: GetSecurityMonitoringSignalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalResponse>,
        Error<GetSecurityMonitoringSignalError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/{signal_id}",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityMonitoringSignalResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSecurityMonitoringSignalError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of CSPM findings.
    ///
    /// ### Filtering
    ///
    /// Filters can be applied by appending query parameters to the URL.
    ///
    ///   - Using a single filter: `?filter[attribute_key]=attribute_value`
    ///   - Chaining filters: `?filter[attribute_key]=attribute_value&filter[attribute_key]=attribute_value...`
    ///   - Filtering on tags: `?filter[tags]=tag_key:tag_value&filter[tags]=tag_key_2:tag_value_2`
    ///
    /// Here, `attribute_key` can be any of the filter keys described further below.
    ///
    /// Query parameters of type `integer` support comparison operators (`>`, `>=`, `<`, `<=`). This is particularly useful when filtering by `evaluation_changed_at` or `resource_discovery_timestamp`. For example: `?filter[evaluation_changed_at]=>20123123121`.
    ///
    /// You can also use the negation operator on strings. For example, use `filter[resource_type]=-aws*` to filter for any non-AWS resources.
    ///
    /// The operator must come after the equal sign. For example, to filter with the `>=` operator, add the operator after the equal sign: `filter[evaluation_changed_at]=>=1678809373257`.
    ///
    /// Query parameters must be only among the documented ones and with values of correct types. Duplicated query parameters (e.g. `filter[status]=low&filter[status]=info`) are not allowed.
    ///
    /// ### Response
    ///
    /// The response includes an array of finding objects, pagination metadata, and a count of items that match the query.
    ///
    /// Each finding object contains the following:
    ///
    /// - The finding ID that can be used in a `GetFinding` request to retrieve the full finding details.
    /// - Core attributes, including status, evaluation, high-level resource details, muted state, and rule details.
    /// - `evaluation_changed_at` and `resource_discovery_date` time stamps.
    /// - An array of associated tags.
    ///
    pub async fn list_findings(
        &self,
        params: ListFindingsParams,
    ) -> Result<Option<crate::datadogV2::model::ListFindingsResponse>, Error<ListFindingsError>>
    {
        match self.list_findings_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of CSPM findings.
    ///
    /// ### Filtering
    ///
    /// Filters can be applied by appending query parameters to the URL.
    ///
    ///   - Using a single filter: `?filter[attribute_key]=attribute_value`
    ///   - Chaining filters: `?filter[attribute_key]=attribute_value&filter[attribute_key]=attribute_value...`
    ///   - Filtering on tags: `?filter[tags]=tag_key:tag_value&filter[tags]=tag_key_2:tag_value_2`
    ///
    /// Here, `attribute_key` can be any of the filter keys described further below.
    ///
    /// Query parameters of type `integer` support comparison operators (`>`, `>=`, `<`, `<=`). This is particularly useful when filtering by `evaluation_changed_at` or `resource_discovery_timestamp`. For example: `?filter[evaluation_changed_at]=>20123123121`.
    ///
    /// You can also use the negation operator on strings. For example, use `filter[resource_type]=-aws*` to filter for any non-AWS resources.
    ///
    /// The operator must come after the equal sign. For example, to filter with the `>=` operator, add the operator after the equal sign: `filter[evaluation_changed_at]=>=1678809373257`.
    ///
    /// Query parameters must be only among the documented ones and with values of correct types. Duplicated query parameters (e.g. `filter[status]=low&filter[status]=info`) are not allowed.
    ///
    /// ### Response
    ///
    /// The response includes an array of finding objects, pagination metadata, and a count of items that match the query.
    ///
    /// Each finding object contains the following:
    ///
    /// - The finding ID that can be used in a `GetFinding` request to retrieve the full finding details.
    /// - Core attributes, including status, evaluation, high-level resource details, muted state, and rule details.
    /// - `evaluation_changed_at` and `resource_discovery_date` time stamps.
    /// - An array of associated tags.
    ///
    pub async fn list_findings_with_http_info(
        &self,
        params: ListFindingsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ListFindingsResponse>,
        Error<ListFindingsError>,
    > {
        let operation_id = "v2.list_findings".to_string();
        if self.config.is_unstable_operation_enabled(&operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_findings' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_configuration = &self.config;

        // unbox and build parameters
        let page_limit = params.page_limit;
        let snapshot_timestamp = params.snapshot_timestamp;
        let page_cursor = params.page_cursor;
        let filter_tags = params.filter_tags;
        let filter_evaluation_changed_at = params.filter_evaluation_changed_at;
        let filter_muted = params.filter_muted;
        let filter_rule_id = params.filter_rule_id;
        let filter_rule_name = params.filter_rule_name;
        let filter_resource_type = params.filter_resource_type;
        let filter_discovery_timestamp = params.filter_discovery_timestamp;
        let filter_evaluation = params.filter_evaluation;
        let filter_status = params.filter_status;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_limit {
            local_req_builder = local_req_builder.query(&[("page[limit]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_evaluation_changed_at {
            local_req_builder = local_req_builder
                .query(&[("filter[evaluation_changed_at]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_muted {
            local_req_builder =
                local_req_builder.query(&[("filter[muted]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_id]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_name]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_resource_type {
            local_req_builder =
                local_req_builder.query(&[("filter[resource_type]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_discovery_timestamp {
            local_req_builder =
                local_req_builder.query(&[("filter[discovery_timestamp]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_evaluation {
            local_req_builder =
                local_req_builder.query(&[("filter[evaluation]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_str.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::ListFindingsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListFindingsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of configured security filters with their definitions.
    pub async fn list_security_filters(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFiltersResponse>,
        Error<ListSecurityFiltersError>,
    > {
        match self.list_security_filters_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured security filters with their definitions.
    pub async fn list_security_filters_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFiltersResponse>,
        Error<ListSecurityFiltersError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityFiltersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSecurityFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List rules.
    pub async fn list_security_monitoring_rules(
        &self,
        params: ListSecurityMonitoringRulesParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringListRulesResponse>,
        Error<ListSecurityMonitoringRulesError>,
    > {
        match self
            .list_security_monitoring_rules_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// List rules.
    pub async fn list_security_monitoring_rules_with_http_info(
        &self,
        params: ListSecurityMonitoringRulesParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringListRulesResponse>,
        Error<ListSecurityMonitoringRulesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityMonitoringListRulesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSecurityMonitoringRulesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The list endpoint returns security signals that match a search query.
    /// Both this endpoint and the POST endpoint can be used interchangeably when listing
    /// security signals.
    pub async fn list_security_monitoring_signals(
        &self,
        params: ListSecurityMonitoringSignalsParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<ListSecurityMonitoringSignalsError>,
    > {
        match self
            .list_security_monitoring_signals_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The list endpoint returns security signals that match a search query.
    /// Both this endpoint and the POST endpoint can be used interchangeably when listing
    /// security signals.
    pub async fn list_security_monitoring_signals_with_http_info(
        &self,
        params: ListSecurityMonitoringSignalsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<ListSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_query = params.filter_query;
        let filter_from = params.filter_from;
        let filter_to = params.filter_to;
        let sort = params.sort;
        let page_cursor = params.page_cursor;
        let page_limit = params.page_limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_to {
            local_req_builder = local_req_builder.query(&[("filter[to]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_limit {
            local_req_builder = local_req_builder.query(&[("page[limit]", &local_str.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSecurityMonitoringSignalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Mute or unmute findings.
    pub async fn mute_findings(
        &self,
        params: MuteFindingsParams,
    ) -> Result<Option<crate::datadogV2::model::BulkMuteFindingsResponse>, Error<MuteFindingsError>>
    {
        match self.mute_findings_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Mute or unmute findings.
    pub async fn mute_findings_with_http_info(
        &self,
        params: MuteFindingsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::BulkMuteFindingsResponse>,
        Error<MuteFindingsError>,
    > {
        let operation_id = "v2.mute_findings".to_string();
        if self.config.is_unstable_operation_enabled(&operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.mute_findings' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::BulkMuteFindingsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<MuteFindingsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns security signals that match a search query.
    /// Both this endpoint and the GET endpoint can be used interchangeably for listing
    /// security signals.
    pub async fn search_security_monitoring_signals(
        &self,
        params: SearchSecurityMonitoringSignalsParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<SearchSecurityMonitoringSignalsError>,
    > {
        match self
            .search_security_monitoring_signals_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns security signals that match a search query.
    /// Both this endpoint and the GET endpoint can be used interchangeably for listing
    /// security signals.
    pub async fn search_security_monitoring_signals_with_http_info(
        &self,
        params: SearchSecurityMonitoringSignalsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<SearchSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/signals/search",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV2::model::SecurityMonitoringSignalsListResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SearchSecurityMonitoringSignalsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a specific security filter.
    /// Returns the security filter object when the request is successful.
    pub async fn update_security_filter(
        &self,
        params: UpdateSecurityFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<UpdateSecurityFilterError>,
    > {
        match self.update_security_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a specific security filter.
    /// Returns the security filter object when the request is successful.
    pub async fn update_security_filter_with_http_info(
        &self,
        params: UpdateSecurityFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<UpdateSecurityFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let security_filter_id = params.security_filter_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/configuration/security_filters/{security_filter_id}",
            local_configuration.base_path,
            security_filter_id = urlencode(security_filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateSecurityFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an existing rule. When updating `cases`, `queries` or `options`, the whole field
    /// must be included. For example, when modifying a query all queries must be included.
    /// Default rules can only be updated to be enabled and to change notifications.
    pub async fn update_security_monitoring_rule(
        &self,
        params: UpdateSecurityMonitoringRuleParams,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<UpdateSecurityMonitoringRuleError>,
    > {
        match self
            .update_security_monitoring_rule_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an existing rule. When updating `cases`, `queries` or `options`, the whole field
    /// must be included. For example, when modifying a query all queries must be included.
    /// Default rules can only be updated to be enabled and to change notifications.
    pub async fn update_security_monitoring_rule_with_http_info(
        &self,
        params: UpdateSecurityMonitoringRuleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<UpdateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let rule_id = params.rule_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules/{rule_id}",
            local_configuration.base_path,
            rule_id = urlencode(rule_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::SecurityMonitoringRuleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateSecurityMonitoringRuleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
