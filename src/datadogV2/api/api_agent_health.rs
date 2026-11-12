// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListAgentHealthHostIssuesOptionalParams is a struct for passing parameters to the method [`AgentHealthAPI::list_agent_health_host_issues`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAgentHealthHostIssuesOptionalParams {
    /// Agent type associated with the resource. Defaults to `node` when omitted.
    pub agent_type: Option<crate::datadogV2::model::AgentHealthAgentType>,
}

impl ListAgentHealthHostIssuesOptionalParams {
    /// Agent type associated with the resource. Defaults to `node` when omitted.
    pub fn agent_type(mut self, value: crate::datadogV2::model::AgentHealthAgentType) -> Self {
        self.agent_type = Some(value);
        self
    }
}

/// ListAgentHealthIssuesOptionalParams is a struct for passing parameters to the method [`AgentHealthAPI::list_agent_health_issues`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAgentHealthIssuesOptionalParams {
    /// The number of issues to return per page, from 1 to 100. Defaults to 25.
    pub page_size: Option<i64>,
    /// The zero-based page number, from 0 to 1000000. Defaults to 0.
    pub page_number: Option<i64>,
    /// Filter by a comma-separated list of exact `severity` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub filter_severity: Option<String>,
    /// Filter by a comma-separated list of exact `category` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub filter_category: Option<String>,
    /// Filter by a comma-separated list of exact `issue_type` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub filter_issue_type: Option<String>,
    /// Filter by a comma-separated list of exact `resource_kind` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub filter_resource_kind: Option<String>,
    /// Filter by a comma-separated list of exact `resource_id` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub filter_resource_id: Option<String>,
}

impl ListAgentHealthIssuesOptionalParams {
    /// The number of issues to return per page, from 1 to 100. Defaults to 25.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The zero-based page number, from 0 to 1000000. Defaults to 0.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by a comma-separated list of exact `severity` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub fn filter_severity(mut self, value: String) -> Self {
        self.filter_severity = Some(value);
        self
    }
    /// Filter by a comma-separated list of exact `category` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub fn filter_category(mut self, value: String) -> Self {
        self.filter_category = Some(value);
        self
    }
    /// Filter by a comma-separated list of exact `issue_type` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub fn filter_issue_type(mut self, value: String) -> Self {
        self.filter_issue_type = Some(value);
        self
    }
    /// Filter by a comma-separated list of exact `resource_kind` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub fn filter_resource_kind(mut self, value: String) -> Self {
        self.filter_resource_kind = Some(value);
        self
    }
    /// Filter by a comma-separated list of exact `resource_id` values, up to 50 values
    /// of at most 256 characters each. Values are combined with OR within this filter and AND across filters.
    /// Blank entries are ignored; an all-blank filter is treated as absent.
    pub fn filter_resource_id(mut self, value: String) -> Self {
        self.filter_resource_id = Some(value);
        self
    }
}

/// GetAgentHealthIssueError is a struct for typed errors of method [`AgentHealthAPI::get_agent_health_issue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgentHealthIssueError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAgentHealthHostIssuesError is a struct for typed errors of method [`AgentHealthAPI::list_agent_health_host_issues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAgentHealthHostIssuesError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAgentHealthIssuesError is a struct for typed errors of method [`AgentHealthAPI::list_agent_health_issues`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAgentHealthIssuesError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// List open Agent Health issues across your organization and get an issue with its available remediation. Issues are reported by node Agents, Cluster Agents, and OpenTelemetry collectors. The meaning of `resource_id` depends on `resource_kind`: hostname for `host`, DaemonSet UID for `deployment` (node Agent), cluster UUID for `cluster`, hostname of the host running the Cluster Agent for `cluster_agent`, or OpenTelemetry collector ID for `otel`.
#[derive(Debug, Clone)]
pub struct AgentHealthAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for AgentHealthAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl AgentHealthAPI {
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

    /// Get an Agent Health issue and its available remediation in your organization.
    /// Only open issues are returned. The `remediation` attribute is omitted when
    /// the issue has no remediation.
    ///
    /// Returns `404` if the issue does not exist, is not open, belongs to another
    /// organization, or you do not have access to its data. This response does not
    /// reveal whether the issue exists. Returns `403` if you lack the `hosts_read` permission.
    pub async fn get_agent_health_issue(
        &self,
        issue_id: String,
    ) -> Result<
        crate::datadogV2::model::AgentHealthIssueResponse,
        datadog::Error<GetAgentHealthIssueError>,
    > {
        match self.get_agent_health_issue_with_http_info(issue_id).await {
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

    /// Get an Agent Health issue and its available remediation in your organization.
    /// Only open issues are returned. The `remediation` attribute is omitted when
    /// the issue has no remediation.
    ///
    /// Returns `404` if the issue does not exist, is not open, belongs to another
    /// organization, or you do not have access to its data. This response does not
    /// reveal whether the issue exists. Returns `403` if you lack the `hosts_read` permission.
    pub async fn get_agent_health_issue_with_http_info(
        &self,
        issue_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AgentHealthIssueResponse>,
        datadog::Error<GetAgentHealthIssueError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_agent_health_issue";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_agent_health_issue' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/agenthealth/issues/{issue_id}",
            local_configuration.get_operation_host(local_operation_id),
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
            match serde_json::from_str::<crate::datadogV2::model::AgentHealthIssueResponse>(
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
            let local_entity: Option<GetAgentHealthIssueError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Returns the active Agent Health issues for a node Agent or Cluster Agent resource.
    pub async fn list_agent_health_host_issues(
        &self,
        resource_id: String,
        params: ListAgentHealthHostIssuesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AgentHealthHostIssuesResponse,
        datadog::Error<ListAgentHealthHostIssuesError>,
    > {
        match self
            .list_agent_health_host_issues_with_http_info(resource_id, params)
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

    /// Returns the active Agent Health issues for a node Agent or Cluster Agent resource.
    pub async fn list_agent_health_host_issues_with_http_info(
        &self,
        resource_id: String,
        params: ListAgentHealthHostIssuesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AgentHealthHostIssuesResponse>,
        datadog::Error<ListAgentHealthHostIssuesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_agent_health_host_issues";

        // unbox and build optional parameters
        let agent_type = params.agent_type;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/agenthealth/hosts/{resource_id}/issues",
            local_configuration.get_operation_host(local_operation_id),
            resource_id = datadog::urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = agent_type {
            local_req_builder =
                local_req_builder.query(&[("agent_type", &local_query_param.to_string())]);
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

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::AgentHealthHostIssuesResponse>(
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
            let local_entity: Option<ListAgentHealthHostIssuesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List Agent Health issues across your organization. Only open issues are returned.
    /// Returns `403` if you lack the `hosts_read` permission.
    ///
    /// Results use a stable order: severity rank (`high`, `medium`, `low`, then other values),
    /// then `first_seen_at` descending with missing timestamps last, then issue ID ascending.
    /// Use `page[size]` and the zero-based `page[number]` to paginate. Invalid page values
    /// return a `400` JSON:API error response. `meta.page.total_filtered_count` is the number
    /// of issues matching the filters, and `meta.page.total_count` the number of open issues
    /// in the organization.
    ///
    /// Each filter accepts up to 50 comma-separated exact values of at most 256 characters
    /// each, combined with OR within a filter and AND across filters. Blank entries are ignored; an all-blank filter
    /// is treated as absent.
    pub async fn list_agent_health_issues(
        &self,
        params: ListAgentHealthIssuesOptionalParams,
    ) -> Result<
        crate::datadogV2::model::AgentHealthIssuesResponse,
        datadog::Error<ListAgentHealthIssuesError>,
    > {
        match self.list_agent_health_issues_with_http_info(params).await {
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

    /// List Agent Health issues across your organization. Only open issues are returned.
    /// Returns `403` if you lack the `hosts_read` permission.
    ///
    /// Results use a stable order: severity rank (`high`, `medium`, `low`, then other values),
    /// then `first_seen_at` descending with missing timestamps last, then issue ID ascending.
    /// Use `page[size]` and the zero-based `page[number]` to paginate. Invalid page values
    /// return a `400` JSON:API error response. `meta.page.total_filtered_count` is the number
    /// of issues matching the filters, and `meta.page.total_count` the number of open issues
    /// in the organization.
    ///
    /// Each filter accepts up to 50 comma-separated exact values of at most 256 characters
    /// each, combined with OR within a filter and AND across filters. Blank entries are ignored; an all-blank filter
    /// is treated as absent.
    pub async fn list_agent_health_issues_with_http_info(
        &self,
        params: ListAgentHealthIssuesOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::AgentHealthIssuesResponse>,
        datadog::Error<ListAgentHealthIssuesError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_agent_health_issues";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_agent_health_issues' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let filter_severity = params.filter_severity;
        let filter_category = params.filter_category;
        let filter_issue_type = params.filter_issue_type;
        let filter_resource_kind = params.filter_resource_kind;
        let filter_resource_id = params.filter_resource_id;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/agenthealth/issues",
            local_configuration.get_operation_host(local_operation_id)
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
        if let Some(ref local_query_param) = filter_severity {
            local_req_builder =
                local_req_builder.query(&[("filter[severity]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_category {
            local_req_builder =
                local_req_builder.query(&[("filter[category]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_issue_type {
            local_req_builder =
                local_req_builder.query(&[("filter[issue_type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_resource_kind {
            local_req_builder = local_req_builder
                .query(&[("filter[resource_kind]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_resource_id {
            local_req_builder =
                local_req_builder.query(&[("filter[resource_id]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::AgentHealthIssuesResponse>(
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
            let local_entity: Option<ListAgentHealthIssuesError> =
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
