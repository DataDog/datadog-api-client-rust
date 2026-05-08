// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListReplayAnalysisIssueSessionsOptionalParams is a struct for passing parameters to the method [`RumReplayAnalysisAPI::list_replay_analysis_issue_sessions`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListReplayAnalysisIssueSessionsOptionalParams {
    /// Sort order for the results. Valid values are `last_seen_at`, `-last_seen_at`, `proximity`, and `-proximity`. Defaults to `-last_seen_at`.
    pub sort: Option<String>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Number of items per page. Must be between 1 and 100.
    pub page_size: Option<i32>,
}

impl ListReplayAnalysisIssueSessionsOptionalParams {
    /// Sort order for the results. Valid values are `last_seen_at`, `-last_seen_at`, `proximity`, and `-proximity`. Defaults to `-last_seen_at`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of items per page. Must be between 1 and 100.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// ListReplayAnalysisIssuesOptionalParams is a struct for passing parameters to the method [`RumReplayAnalysisAPI::list_replay_analysis_issues`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListReplayAnalysisIssuesOptionalParams {
    /// Filter issues by application UUID.
    pub filter_application_id: Option<String>,
    /// Filter issues by comma-separated severity values. Valid values are `high`, `medium`, and `low`.
    pub filter_severity: Option<String>,
    /// Filter issues by comma-separated view names.
    pub filter_view_name: Option<String>,
    /// Filter issues by comma-separated issue categories.
    pub filter_issue_category: Option<String>,
    /// Sort order for the results. Valid values are `created_at`, `-created_at`, `severity`, `-severity`, `session_count`, and `-session_count`. Defaults to `-created_at`.
    pub sort: Option<String>,
    /// Page number for pagination (0-indexed).
    pub page_number: Option<i32>,
    /// Number of items per page. Must be between 1 and 100.
    pub page_size: Option<i32>,
}

impl ListReplayAnalysisIssuesOptionalParams {
    /// Filter issues by application UUID.
    pub fn filter_application_id(mut self, value: String) -> Self {
        self.filter_application_id = Some(value);
        self
    }
    /// Filter issues by comma-separated severity values. Valid values are `high`, `medium`, and `low`.
    pub fn filter_severity(mut self, value: String) -> Self {
        self.filter_severity = Some(value);
        self
    }
    /// Filter issues by comma-separated view names.
    pub fn filter_view_name(mut self, value: String) -> Self {
        self.filter_view_name = Some(value);
        self
    }
    /// Filter issues by comma-separated issue categories.
    pub fn filter_issue_category(mut self, value: String) -> Self {
        self.filter_issue_category = Some(value);
        self
    }
    /// Sort order for the results. Valid values are `created_at`, `-created_at`, `severity`, `-severity`, `session_count`, and `-session_count`. Defaults to `-created_at`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Page number for pagination (0-indexed).
    pub fn page_number(mut self, value: i32) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Number of items per page. Must be between 1 and 100.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// GetReplayAnalysisIssueError is a struct for typed errors of method [`RumReplayAnalysisAPI::get_replay_analysis_issue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetReplayAnalysisIssueError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListReplayAnalysisIssueSessionsError is a struct for typed errors of method [`RumReplayAnalysisAPI::list_replay_analysis_issue_sessions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListReplayAnalysisIssueSessionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListReplayAnalysisIssuesError is a struct for typed errors of method [`RumReplayAnalysisAPI::list_replay_analysis_issues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListReplayAnalysisIssuesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Analyze RUM replay sessions to identify and investigate user-facing issues. Retrieve issues detected by AI analysis, get details for individual issues, and explore the sessions associated with each issue.
#[derive(Debug, Clone)]
pub struct RumReplayAnalysisAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RumReplayAnalysisAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RumReplayAnalysisAPI {
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

    /// Retrieve details of a specific RUM replay analysis issue by its identifier.
    pub async fn get_replay_analysis_issue(
        &self,
        issue_id: String,
    ) -> Result<
        crate::datadogV2::model::ReplayAnalysisIssueResponse,
        datadog::Error<GetReplayAnalysisIssueError>,
    > {
        match self
            .get_replay_analysis_issue_with_http_info(issue_id)
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

    /// Retrieve details of a specific RUM replay analysis issue by its identifier.
    pub async fn get_replay_analysis_issue_with_http_info(
        &self,
        issue_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ReplayAnalysisIssueResponse>,
        datadog::Error<GetReplayAnalysisIssueError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_replay_analysis_issue";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_replay_analysis_issue' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/analysis/issues/{issue_id}",
            local_configuration.get_operation_host(operation_id),
            issue_id = datadog::urlencode(issue_id)
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
            match serde_json::from_str::<crate::datadogV2::model::ReplayAnalysisIssueResponse>(
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
            let local_entity: Option<GetReplayAnalysisIssueError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of sessions related to a specific RUM replay analysis issue.
    pub async fn list_replay_analysis_issue_sessions(
        &self,
        issue_id: String,
        params: ListReplayAnalysisIssueSessionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ReplayAnalysisIssueSessionsResponse,
        datadog::Error<ListReplayAnalysisIssueSessionsError>,
    > {
        match self
            .list_replay_analysis_issue_sessions_with_http_info(issue_id, params)
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

    /// Retrieve a paginated list of sessions related to a specific RUM replay analysis issue.
    pub async fn list_replay_analysis_issue_sessions_with_http_info(
        &self,
        issue_id: String,
        params: ListReplayAnalysisIssueSessionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ReplayAnalysisIssueSessionsResponse>,
        datadog::Error<ListReplayAnalysisIssueSessionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_replay_analysis_issue_sessions";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_replay_analysis_issue_sessions' is not enabled"
                    .to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let sort = params.sort;
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/analysis/issues/{issue_id}/sessions",
            local_configuration.get_operation_host(operation_id),
            issue_id = datadog::urlencode(issue_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ReplayAnalysisIssueSessionsResponse>(
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
            let local_entity: Option<ListReplayAnalysisIssueSessionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieve a paginated list of RUM replay analysis issues, optionally filtered by application, severity, view name, or issue category.
    pub async fn list_replay_analysis_issues(
        &self,
        params: ListReplayAnalysisIssuesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ReplayAnalysisIssuesResponse,
        datadog::Error<ListReplayAnalysisIssuesError>,
    > {
        match self
            .list_replay_analysis_issues_with_http_info(params)
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

    /// Retrieve a paginated list of RUM replay analysis issues, optionally filtered by application, severity, view name, or issue category.
    pub async fn list_replay_analysis_issues_with_http_info(
        &self,
        params: ListReplayAnalysisIssuesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ReplayAnalysisIssuesResponse>,
        datadog::Error<ListReplayAnalysisIssuesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_replay_analysis_issues";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_replay_analysis_issues' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let filter_application_id = params.filter_application_id;
        let filter_severity = params.filter_severity;
        let filter_view_name = params.filter_view_name;
        let filter_issue_category = params.filter_issue_category;
        let sort = params.sort;
        let page_number = params.page_number;
        let page_size = params.page_size;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/replay/analysis/issues",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = filter_application_id {
            local_req_builder = local_req_builder
                .query(&[("filter[application_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_severity {
            local_req_builder =
                local_req_builder.query(&[("filter[severity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_view_name {
            local_req_builder =
                local_req_builder.query(&[("filter[view_name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_issue_category {
            local_req_builder = local_req_builder
                .query(&[("filter[issue_category]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ReplayAnalysisIssuesResponse>(
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
            let local_entity: Option<ListReplayAnalysisIssuesError> =
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
