// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListAllCSMAgentsOptionalParams is a struct for passing parameters to the method [`CSMAgentsAPI::list_all_csm_agents`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAllCSMAgentsOptionalParams {
    /// The page index for pagination (zero-based).
    pub page: Option<i32>,
    /// The number of items to include in a single page.
    pub size: Option<i32>,
    /// A search query string to filter results (for example, `hostname:COMP-T2H4J27423`).
    pub query: Option<String>,
    /// The sort direction for results. Use `asc` for ascending or `desc` for descending.
    pub order_direction: Option<crate::datadogV2::model::OrderDirection>,
}

impl ListAllCSMAgentsOptionalParams {
    /// The page index for pagination (zero-based).
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }
    /// The number of items to include in a single page.
    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
        self
    }
    /// A search query string to filter results (for example, `hostname:COMP-T2H4J27423`).
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// The sort direction for results. Use `asc` for ascending or `desc` for descending.
    pub fn order_direction(mut self, value: crate::datadogV2::model::OrderDirection) -> Self {
        self.order_direction = Some(value);
        self
    }
}

/// ListAllCSMServerlessAgentsOptionalParams is a struct for passing parameters to the method [`CSMAgentsAPI::list_all_csm_serverless_agents`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAllCSMServerlessAgentsOptionalParams {
    /// The page index for pagination (zero-based).
    pub page: Option<i32>,
    /// The number of items to include in a single page.
    pub size: Option<i32>,
    /// A search query string to filter results (for example, `hostname:COMP-T2H4J27423`).
    pub query: Option<String>,
    /// The sort direction for results. Use `asc` for ascending or `desc` for descending.
    pub order_direction: Option<crate::datadogV2::model::OrderDirection>,
}

impl ListAllCSMServerlessAgentsOptionalParams {
    /// The page index for pagination (zero-based).
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }
    /// The number of items to include in a single page.
    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
        self
    }
    /// A search query string to filter results (for example, `hostname:COMP-T2H4J27423`).
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// The sort direction for results. Use `asc` for ascending or `desc` for descending.
    pub fn order_direction(mut self, value: crate::datadogV2::model::OrderDirection) -> Self {
        self.order_direction = Some(value);
        self
    }
}

/// ListAllCSMAgentsError is a struct for typed errors of method [`CSMAgentsAPI::list_all_csm_agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAllCSMAgentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAllCSMServerlessAgentsError is a struct for typed errors of method [`CSMAgentsAPI::list_all_csm_serverless_agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAllCSMServerlessAgentsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Datadog Cloud Security Management (CSM) delivers real-time threat detection
/// and continuous configuration audits across your entire cloud infrastructure,
/// all in a unified view for seamless collaboration and faster remediation.
/// Go to <https://docs.datadoghq.com/security/cloud_security_management> to learn more
#[derive(Debug, Clone)]
pub struct CSMAgentsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CSMAgentsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CSMAgentsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
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

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Get the list of all CSM Agents running on your hosts and containers.
    pub async fn list_all_csm_agents(
        &self,
        params: ListAllCSMAgentsOptionalParams,
    ) -> Result<crate::datadogV2::model::CsmAgentsResponse, datadog::Error<ListAllCSMAgentsError>>
    {
        match self.list_all_csm_agents_with_http_info(params).await {
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

    /// Get the list of all CSM Agents running on your hosts and containers.
    pub async fn list_all_csm_agents_with_http_info(
        &self,
        params: ListAllCSMAgentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmAgentsResponse>,
        datadog::Error<ListAllCSMAgentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_all_csm_agents";

        // unbox and build optional parameters
        let page = params.page;
        let size = params.size;
        let query = params.query;
        let order_direction = params.order_direction;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/onboarding/agents",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = size {
            local_req_builder =
                local_req_builder.query(&[("size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = order_direction {
            local_req_builder =
                local_req_builder.query(&[("order_direction", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CsmAgentsResponse>(&local_content)
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
            let local_entity: Option<ListAllCSMAgentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of all CSM Serverless Agents running on your hosts and containers.
    pub async fn list_all_csm_serverless_agents(
        &self,
        params: ListAllCSMServerlessAgentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CsmAgentsResponse,
        datadog::Error<ListAllCSMServerlessAgentsError>,
    > {
        match self
            .list_all_csm_serverless_agents_with_http_info(params)
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

    /// Get the list of all CSM Serverless Agents running on your hosts and containers.
    pub async fn list_all_csm_serverless_agents_with_http_info(
        &self,
        params: ListAllCSMServerlessAgentsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmAgentsResponse>,
        datadog::Error<ListAllCSMServerlessAgentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_all_csm_serverless_agents";

        // unbox and build optional parameters
        let page = params.page;
        let size = params.size;
        let query = params.query;
        let order_direction = params.order_direction;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/onboarding/serverless/agents",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = size {
            local_req_builder =
                local_req_builder.query(&[("size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = order_direction {
            local_req_builder =
                local_req_builder.query(&[("order_direction", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CsmAgentsResponse>(&local_content)
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
            let local_entity: Option<ListAllCSMServerlessAgentsError> =
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
