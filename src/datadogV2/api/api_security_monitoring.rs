// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetFindingParams is a struct for passing parameters to the method [`GetFinding`]
#[derive(Clone, Debug, Default)]
pub struct GetFindingParams {
    /// The ID of the finding.
    pub finding_id: String,
    /// Return the finding for a given snapshot of time (Unix ms).
    pub snapshot_timestamp: Option<i64>,
}

/// ListFindingsParams is a struct for passing parameters to the method [`ListFindings`]
#[derive(Clone, Debug, Default)]
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

/// UpdateFindingParams is a struct for passing parameters to the method [`UpdateFinding`]
#[derive(Clone, Debug, Default)]
pub struct UpdateFindingParams {
    /// The ID of the finding.
    pub finding_id: String,
    /// To mute or unmute a finding, the request body should include at least two attributes: `muted` and `reason`. The allowed reasons depend on whether the finding is being muted or unmuted:
    /// - To mute a finding: `PENDING_FIX`, `FALSE_POSITIVE`, `ACCEPTED_RISK`, `OTHER`.
    /// - To unmute a finding : `NO_PENDING_FIX`, `HUMAN_ERROR`, `NO_LONGER_ACCEPTED_RISK`, `OTHER`.
    ///
    pub body: crate::datadogV2::model::MuteFindingRequest,
}

/// GetFindingError is a struct for typed errors of method [`GetFinding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFindingError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListFindingsError is a struct for typed errors of method [`ListFindings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFindingsError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateFindingError is a struct for typed errors of method [`UpdateFinding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFindingError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status404(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status409(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status422(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status429(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
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
    ) -> Result<ResponseContent<crate::datadogV2::model::GetFindingResponse>, Error<GetFindingError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let finding_id = params.finding_id;
        let snapshot_timestamp = params.snapshot_timestamp;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings/{finding_id}",
            local_configuration.base_path,
            finding_id = urlencode(finding_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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
    ) -> Result<Option<crate::datadogV2::model::ListFindingsResponse>, Error<ListFindingsError>> {
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
    ) -> Result<ResponseContent<crate::datadogV2::model::ListFindingsResponse>, Error<ListFindingsError>> {
        let local_configuration = &self.config;

        // unbox the parameters
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

        let local_uri_str = format!("{}/api/v2/posture_management/findings", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

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

    /// Mute or unmute a specific finding.
    /// The API returns the updated finding object when the request is successful.
    pub async fn update_finding(
        &self,
        params: UpdateFindingParams,
    ) -> Result<Option<crate::datadogV2::model::MuteFindingResponse>, Error<UpdateFindingError>> {
        match self.update_finding_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Mute or unmute a specific finding.
    /// The API returns the updated finding object when the request is successful.
    pub async fn update_finding_with_http_info(
        &self,
        params: UpdateFindingParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::MuteFindingResponse>, Error<UpdateFindingError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let finding_id = params.finding_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/posture_management/findings/{finding_id}",
            local_configuration.base_path,
            finding_id = urlencode(finding_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::MuteFindingResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateFindingError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
