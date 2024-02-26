// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetFindingOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::get_finding`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetFindingOptionalParams {
    /// Return the finding for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
}

impl GetFindingOptionalParams {
    /// Return the finding for a given snapshot of time (Unix ms).
    pub fn snapshot_timestamp(&mut self, value: i64) -> &mut Self {
        self.snapshot_timestamp = Some(value);
        self
    }
}

/// ListFindingsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_findings`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListFindingsOptionalParams {
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

impl ListFindingsOptionalParams {
    /// Limit the number of findings returned. Must be <= 1000.
    pub fn page_limit(&mut self, value: i64) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
    /// Return findings for a given snapshot of time (Unix ms).
    pub fn snapshot_timestamp(&mut self, value: i64) -> &mut Self {
        self.snapshot_timestamp = Some(value);
        self
    }
    /// Return the next page of findings pointed to by the cursor.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
    /// Return findings that have these associated tags (repeatable).
    pub fn filter_tags(&mut self, value: String) -> &mut Self {
        self.filter_tags = Some(value);
        self
    }
    /// Return findings that have changed from pass to fail or vice versa on a specified date (Unix ms) or date range (using comparison operators).
    pub fn filter_evaluation_changed_at(&mut self, value: String) -> &mut Self {
        self.filter_evaluation_changed_at = Some(value);
        self
    }
    /// Set to `true` to return findings that are muted. Set to `false` to return unmuted findings.
    pub fn filter_muted(&mut self, value: bool) -> &mut Self {
        self.filter_muted = Some(value);
        self
    }
    /// Return findings for the specified rule ID.
    pub fn filter_rule_id(&mut self, value: String) -> &mut Self {
        self.filter_rule_id = Some(value);
        self
    }
    /// Return findings for the specified rule.
    pub fn filter_rule_name(&mut self, value: String) -> &mut Self {
        self.filter_rule_name = Some(value);
        self
    }
    /// Return only findings for the specified resource type.
    pub fn filter_resource_type(&mut self, value: String) -> &mut Self {
        self.filter_resource_type = Some(value);
        self
    }
    /// Return findings that were found on a specified date (Unix ms) or date range (using comparison operators).
    pub fn filter_discovery_timestamp(&mut self, value: String) -> &mut Self {
        self.filter_discovery_timestamp = Some(value);
        self
    }
    /// Return only `pass` or `fail` findings.
    pub fn filter_evaluation(
        &mut self,
        value: crate::datadogV2::model::FindingEvaluation,
    ) -> &mut Self {
        self.filter_evaluation = Some(value);
        self
    }
    /// Return only findings with the specified status.
    pub fn filter_status(&mut self, value: crate::datadogV2::model::FindingStatus) -> &mut Self {
        self.filter_status = Some(value);
        self
    }
}

/// ListSecurityMonitoringRulesOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_rules`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSecurityMonitoringRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
}

impl ListSecurityMonitoringRulesOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
}

/// ListSecurityMonitoringSignalsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::list_security_monitoring_signals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSecurityMonitoringSignalsOptionalParams {
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

impl ListSecurityMonitoringSignalsOptionalParams {
    /// The search query for security signals.
    pub fn filter_query(&mut self, value: String) -> &mut Self {
        self.filter_query = Some(value);
        self
    }
    /// The minimum timestamp for requested security signals.
    pub fn filter_from(&mut self, value: String) -> &mut Self {
        self.filter_from = Some(value);
        self
    }
    /// The maximum timestamp for requested security signals.
    pub fn filter_to(&mut self, value: String) -> &mut Self {
        self.filter_to = Some(value);
        self
    }
    /// The order of the security signals in results.
    pub fn sort(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalsSort,
    ) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// A list of results using the cursor provided in the previous query.
    pub fn page_cursor(&mut self, value: String) -> &mut Self {
        self.page_cursor = Some(value);
        self
    }
    /// The maximum number of security signals in the response.
    pub fn page_limit(&mut self, value: i32) -> &mut Self {
        self.page_limit = Some(value);
        self
    }
}

/// SearchSecurityMonitoringSignalsOptionalParams is a struct for passing parameters to the method [`SecurityMonitoringAPI::search_security_monitoring_signals`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchSecurityMonitoringSignalsOptionalParams {
    pub body: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequest>,
}

impl SearchSecurityMonitoringSignalsOptionalParams {
    pub fn body(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequest,
    ) -> &mut Self {
        self.body = Some(value);
        self
    }
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
        body: crate::datadogV2::model::SecurityFilterCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<CreateSecurityFilterError>,
    > {
        match self.create_security_filter_with_http_info(body).await {
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
        body: crate::datadogV2::model::SecurityFilterCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<CreateSecurityFilterError>,
    > {
        let local_configuration = &self.config;

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
        body: crate::datadogV2::model::SecurityMonitoringRuleCreatePayload,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<CreateSecurityMonitoringRuleError>,
    > {
        match self
            .create_security_monitoring_rule_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a detection rule.
    pub async fn create_security_monitoring_rule_with_http_info(
        &self,
        body: crate::datadogV2::model::SecurityMonitoringRuleCreatePayload,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<CreateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

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
        security_filter_id: String,
    ) -> Result<Option<()>, Error<DeleteSecurityFilterError>> {
        match self
            .delete_security_filter_with_http_info(security_filter_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific security filter.
    pub async fn delete_security_filter_with_http_info(
        &self,
        security_filter_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteSecurityFilterError>> {
        let local_configuration = &self.config;

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
        rule_id: String,
    ) -> Result<Option<()>, Error<DeleteSecurityMonitoringRuleError>> {
        match self
            .delete_security_monitoring_rule_with_http_info(rule_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing rule. Default rules cannot be deleted.
    pub async fn delete_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteSecurityMonitoringRuleError>> {
        let local_configuration = &self.config;

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
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalAssigneeUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        match self
            .edit_security_monitoring_signal_assignee_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalAssigneeUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        let local_configuration = &self.config;

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
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        match self
            .edit_security_monitoring_signal_incidents_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the related incidents for a security signal.
    pub async fn edit_security_monitoring_signal_incidents_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalIncidentsError>,
    > {
        let local_configuration = &self.config;

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
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        match self
            .edit_security_monitoring_signal_state_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalTriageUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        let local_configuration = &self.config;

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
        finding_id: String,
        params: GetFindingOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::GetFindingResponse>, Error<GetFindingError>> {
        match self.get_finding_with_http_info(finding_id, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a single finding with message and resource configuration.
    pub async fn get_finding_with_http_info(
        &self,
        finding_id: String,
        params: GetFindingOptionalParams,
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

        // unbox and build optional parameters
        let snapshot_timestamp = params.snapshot_timestamp;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings/{finding_id}",
            local_configuration.base_path,
            finding_id = urlencode(finding_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_query_param.to_string())]);
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
        security_filter_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<GetSecurityFilterError>,
    > {
        match self
            .get_security_filter_with_http_info(security_filter_id)
            .await
        {
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
        security_filter_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<GetSecurityFilterError>,
    > {
        let local_configuration = &self.config;

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
        rule_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<GetSecurityMonitoringRuleError>,
    > {
        match self
            .get_security_monitoring_rule_with_http_info(rule_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a rule's details.
    pub async fn get_security_monitoring_rule_with_http_info(
        &self,
        rule_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<GetSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

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
        signal_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringSignalResponse>,
        Error<GetSecurityMonitoringSignalError>,
    > {
        match self
            .get_security_monitoring_signal_with_http_info(signal_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a signal's details.
    pub async fn get_security_monitoring_signal_with_http_info(
        &self,
        signal_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalResponse>,
        Error<GetSecurityMonitoringSignalError>,
    > {
        let local_configuration = &self.config;

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
        params: ListFindingsOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::ListFindingsResponse>, Error<ListFindingsError>>
    {
        match self.list_findings_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    pub fn list_findings_with_pagination(
        &self,
        mut params: ListFindingsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::Finding, Error<ListFindingsError>>> + '_
    {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_findings(params.clone()).await?;

                let Some(resp) = resp else { break };

                let r = resp.data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(page) = resp.meta.page else { break };
                let Some(cursor) = page.cursor else { break };

                params.page_cursor = Some(cursor);
            }
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
        params: ListFindingsOptionalParams,
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

        // unbox and build optional parameters
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

        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = snapshot_timestamp {
            local_req_builder =
                local_req_builder.query(&[("snapshot_timestamp", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_tags {
            local_req_builder =
                local_req_builder.query(&[("filter[tags]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_evaluation_changed_at {
            local_req_builder = local_req_builder.query(&[(
                "filter[evaluation_changed_at]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_muted {
            local_req_builder =
                local_req_builder.query(&[("filter[muted]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_id {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_rule_name {
            local_req_builder =
                local_req_builder.query(&[("filter[rule_name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_resource_type {
            local_req_builder = local_req_builder
                .query(&[("filter[resource_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_discovery_timestamp {
            local_req_builder = local_req_builder.query(&[(
                "filter[discovery_timestamp]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_evaluation {
            local_req_builder =
                local_req_builder.query(&[("filter[evaluation]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
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
        params: ListSecurityMonitoringRulesOptionalParams,
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
        params: ListSecurityMonitoringRulesOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringListRulesResponse>,
        Error<ListSecurityMonitoringRulesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/security_monitoring/rules",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
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
        params: ListSecurityMonitoringSignalsOptionalParams,
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

    pub fn list_security_monitoring_signals_with_pagination(
        &self,
        mut params: ListSecurityMonitoringSignalsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::SecurityMonitoringSignal,
            Error<ListSecurityMonitoringSignalsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_security_monitoring_signals(params.clone()).await?;

                let Some(resp) = resp else { break };
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.page_cursor = Some(after);
            }
        }
    }

    /// The list endpoint returns security signals that match a search query.
    /// Both this endpoint and the POST endpoint can be used interchangeably when listing
    /// security signals.
    pub async fn list_security_monitoring_signals_with_http_info(
        &self,
        params: ListSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<ListSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
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

        if let Some(ref local_query_param) = filter_query {
            local_req_builder =
                local_req_builder.query(&[("filter[query]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from {
            local_req_builder =
                local_req_builder.query(&[("filter[from]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to {
            local_req_builder =
                local_req_builder.query(&[("filter[to]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_cursor {
            local_req_builder =
                local_req_builder.query(&[("page[cursor]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
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
        body: crate::datadogV2::model::BulkMuteFindingsRequest,
    ) -> Result<Option<crate::datadogV2::model::BulkMuteFindingsResponse>, Error<MuteFindingsError>>
    {
        match self.mute_findings_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Mute or unmute findings.
    pub async fn mute_findings_with_http_info(
        &self,
        body: crate::datadogV2::model::BulkMuteFindingsRequest,
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
        params: SearchSecurityMonitoringSignalsOptionalParams,
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

    pub fn search_security_monitoring_signals_with_pagination(
        &self,
        mut params: SearchSecurityMonitoringSignalsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::SecurityMonitoringSignal,
            Error<SearchSecurityMonitoringSignalsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i32 = 10;
            if params.body.is_none() {
                params.body = Some(crate::datadogV2::model::SecurityMonitoringSignalListRequest::new());
            }
            if params.body.as_ref().unwrap().page.is_none() {
                params.body.as_mut().unwrap().page = Some(crate::datadogV2::model::SecurityMonitoringSignalListRequestPage::new());
            }
            if params.body.as_ref().unwrap().page.as_ref().unwrap().limit.is_none() {
                params.body.as_mut().unwrap().page.as_mut().unwrap().limit = Some(page_size);
            } else {
                page_size = params.body.as_ref().unwrap().page.as_ref().unwrap().limit.unwrap().clone();
            }
            loop {
                let resp = self.search_security_monitoring_signals(params.clone()).await?;

                let Some(resp) = resp else { break };
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                let Some(meta) = resp.meta else { break };
                let Some(page) = meta.page else { break };
                let Some(after) = page.after else { break };

                params.body.as_mut().unwrap().page.as_mut().unwrap().cursor = Some(after);
            }
        }
    }

    /// Returns security signals that match a search query.
    /// Both this endpoint and the GET endpoint can be used interchangeably for listing
    /// security signals.
    pub async fn search_security_monitoring_signals_with_http_info(
        &self,
        params: SearchSecurityMonitoringSignalsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringSignalsListResponse>,
        Error<SearchSecurityMonitoringSignalsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build optional parameters
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
        security_filter_id: String,
        body: crate::datadogV2::model::SecurityFilterUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityFilterResponse>,
        Error<UpdateSecurityFilterError>,
    > {
        match self
            .update_security_filter_with_http_info(security_filter_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a specific security filter.
    /// Returns the security filter object when the request is successful.
    pub async fn update_security_filter_with_http_info(
        &self,
        security_filter_id: String,
        body: crate::datadogV2::model::SecurityFilterUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityFilterResponse>,
        Error<UpdateSecurityFilterError>,
    > {
        let local_configuration = &self.config;

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
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleUpdatePayload,
    ) -> Result<
        Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<UpdateSecurityMonitoringRuleError>,
    > {
        match self
            .update_security_monitoring_rule_with_http_info(rule_id, body)
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
        rule_id: String,
        body: crate::datadogV2::model::SecurityMonitoringRuleUpdatePayload,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
        Error<UpdateSecurityMonitoringRuleError>,
    > {
        let local_configuration = &self.config;

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
