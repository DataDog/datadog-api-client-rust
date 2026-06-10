// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// GetCSMAgentlessHostFacetInfoOptionalParams is a struct for passing parameters to the method [`CSMSettingsAPI::get_csm_agentless_host_facet_info`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCSMAgentlessHostFacetInfoOptionalParams {
    /// A search string to filter the facet values.
    pub search: Option<String>,
    /// A filter query to scope the facet value counts.
    pub query: Option<String>,
}

impl GetCSMAgentlessHostFacetInfoOptionalParams {
    /// A search string to filter the facet values.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// A filter query to scope the facet value counts.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// GetCSMUnifiedHostFacetInfoOptionalParams is a struct for passing parameters to the method [`CSMSettingsAPI::get_csm_unified_host_facet_info`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetCSMUnifiedHostFacetInfoOptionalParams {
    /// A search string to filter the facet values.
    pub search: Option<String>,
    /// A filter query to scope the facet value counts.
    pub query: Option<String>,
}

impl GetCSMUnifiedHostFacetInfoOptionalParams {
    /// A search string to filter the facet values.
    pub fn search(mut self, value: String) -> Self {
        self.search = Some(value);
        self
    }
    /// A filter query to scope the facet value counts.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// ListCSMAgentlessHostsOptionalParams is a struct for passing parameters to the method [`CSMSettingsAPI::list_csm_agentless_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCSMAgentlessHostsOptionalParams {
    /// The page index for pagination (zero-based).
    pub page: Option<i32>,
    /// The number of agentless hosts to return per page.
    pub size: Option<i32>,
    /// A search query string to filter agentless hosts.
    pub query: Option<String>,
}

impl ListCSMAgentlessHostsOptionalParams {
    /// The page index for pagination (zero-based).
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }
    /// The number of agentless hosts to return per page.
    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
        self
    }
    /// A search query string to filter agentless hosts.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// ListCSMUnifiedHostsOptionalParams is a struct for passing parameters to the method [`CSMSettingsAPI::list_csm_unified_hosts`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCSMUnifiedHostsOptionalParams {
    /// The page index for pagination (zero-based).
    pub page: Option<i32>,
    /// The number of hosts to return per page.
    pub size: Option<i32>,
    /// A search query string to filter unified hosts.
    pub query: Option<String>,
}

impl ListCSMUnifiedHostsOptionalParams {
    /// The page index for pagination (zero-based).
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }
    /// The number of hosts to return per page.
    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
        self
    }
    /// A search query string to filter unified hosts.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
}

/// GetCSMAgentlessHostFacetInfoError is a struct for typed errors of method [`CSMSettingsAPI::get_csm_agentless_host_facet_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCSMAgentlessHostFacetInfoError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCSMUnifiedHostFacetInfoError is a struct for typed errors of method [`CSMSettingsAPI::get_csm_unified_host_facet_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCSMUnifiedHostFacetInfoError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMAgentlessHostFacetsError is a struct for typed errors of method [`CSMSettingsAPI::list_csm_agentless_host_facets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMAgentlessHostFacetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMAgentlessHostsError is a struct for typed errors of method [`CSMSettingsAPI::list_csm_agentless_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMAgentlessHostsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMUnifiedHostFacetsError is a struct for typed errors of method [`CSMSettingsAPI::list_csm_unified_host_facets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMUnifiedHostFacetsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCSMUnifiedHostsError is a struct for typed errors of method [`CSMSettingsAPI::list_csm_unified_hosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCSMUnifiedHostsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Datadog Cloud Security Management (CSM) Settings APIs allow you to list and filter
/// your cloud hosts monitored by CSM, covering both agentless and agent-based discovery.
/// For more information, see [Cloud Security Management](<https://docs.datadoghq.com/security/cloud_security_management>).
#[derive(Debug, Clone)]
pub struct CSMSettingsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for CSMSettingsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl CSMSettingsAPI {
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

    /// Get the value distribution for a specific agentless host facet, with optional search and filtering.
    pub async fn get_csm_agentless_host_facet_info(
        &self,
        facet: String,
        params: GetCSMAgentlessHostFacetInfoOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CsmHostFacetInfoResponse,
        datadog::Error<GetCSMAgentlessHostFacetInfoError>,
    > {
        match self
            .get_csm_agentless_host_facet_info_with_http_info(facet, params)
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

    /// Get the value distribution for a specific agentless host facet, with optional search and filtering.
    pub async fn get_csm_agentless_host_facet_info_with_http_info(
        &self,
        facet: String,
        params: GetCSMAgentlessHostFacetInfoOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmHostFacetInfoResponse>,
        datadog::Error<GetCSMAgentlessHostFacetInfoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_csm_agentless_host_facet_info";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_csm_agentless_host_facet_info' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let search = params.search;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/agentless_hosts/facet_info",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("facet", &facet.to_string())]);
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CsmHostFacetInfoResponse>(
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
            let local_entity: Option<GetCSMAgentlessHostFacetInfoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the value distribution for a specific unified host facet, with optional search and filtering.
    pub async fn get_csm_unified_host_facet_info(
        &self,
        facet: String,
        params: GetCSMUnifiedHostFacetInfoOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CsmHostFacetInfoResponse,
        datadog::Error<GetCSMUnifiedHostFacetInfoError>,
    > {
        match self
            .get_csm_unified_host_facet_info_with_http_info(facet, params)
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

    /// Get the value distribution for a specific unified host facet, with optional search and filtering.
    pub async fn get_csm_unified_host_facet_info_with_http_info(
        &self,
        facet: String,
        params: GetCSMUnifiedHostFacetInfoOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmHostFacetInfoResponse>,
        datadog::Error<GetCSMUnifiedHostFacetInfoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_csm_unified_host_facet_info";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_csm_unified_host_facet_info' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let search = params.search;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/hosts/facet_info",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("facet", &facet.to_string())]);
        if let Some(ref local_query_param) = search {
            local_req_builder =
                local_req_builder.query(&[("search", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::CsmHostFacetInfoResponse>(
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
            let local_entity: Option<GetCSMUnifiedHostFacetInfoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of available facets for filtering agentless hosts.
    pub async fn list_csm_agentless_host_facets(
        &self,
    ) -> Result<
        crate::datadogV2::model::CsmAgentlessHostFacetsResponse,
        datadog::Error<ListCSMAgentlessHostFacetsError>,
    > {
        match self.list_csm_agentless_host_facets_with_http_info().await {
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

    /// Get the list of available facets for filtering agentless hosts.
    pub async fn list_csm_agentless_host_facets_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmAgentlessHostFacetsResponse>,
        datadog::Error<ListCSMAgentlessHostFacetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_agentless_host_facets";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_csm_agentless_host_facets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/agentless_hosts/facets",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::CsmAgentlessHostFacetsResponse>(
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
            let local_entity: Option<ListCSMAgentlessHostFacetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of agentless hosts for CSM, with optional pagination and filtering.
    pub async fn list_csm_agentless_hosts(
        &self,
        params: ListCSMAgentlessHostsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CsmAgentlessHostsResponse,
        datadog::Error<ListCSMAgentlessHostsError>,
    > {
        match self.list_csm_agentless_hosts_with_http_info(params).await {
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

    /// Get the list of agentless hosts for CSM, with optional pagination and filtering.
    pub async fn list_csm_agentless_hosts_with_http_info(
        &self,
        params: ListCSMAgentlessHostsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmAgentlessHostsResponse>,
        datadog::Error<ListCSMAgentlessHostsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_agentless_hosts";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_csm_agentless_hosts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page = params.page;
        let size = params.size;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/agentless_hosts",
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
            match serde_json::from_str::<crate::datadogV2::model::CsmAgentlessHostsResponse>(
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
            let local_entity: Option<ListCSMAgentlessHostsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of available facets for filtering unified hosts.
    pub async fn list_csm_unified_host_facets(
        &self,
    ) -> Result<
        crate::datadogV2::model::CsmUnifiedHostFacetsResponse,
        datadog::Error<ListCSMUnifiedHostFacetsError>,
    > {
        match self.list_csm_unified_host_facets_with_http_info().await {
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

    /// Get the list of available facets for filtering unified hosts.
    pub async fn list_csm_unified_host_facets_with_http_info(
        &self,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmUnifiedHostFacetsResponse>,
        datadog::Error<ListCSMUnifiedHostFacetsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_unified_host_facets";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_csm_unified_host_facets' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/hosts/facets",
            local_configuration.get_operation_host(operation_id)
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
            match serde_json::from_str::<crate::datadogV2::model::CsmUnifiedHostFacetsResponse>(
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
            let local_entity: Option<ListCSMUnifiedHostFacetsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get the list of unified hosts for CSM, combining agent and agentless host data, with optional pagination and filtering.
    pub async fn list_csm_unified_hosts(
        &self,
        params: ListCSMUnifiedHostsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::CsmUnifiedHostsResponse,
        datadog::Error<ListCSMUnifiedHostsError>,
    > {
        match self.list_csm_unified_hosts_with_http_info(params).await {
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

    /// Get the list of unified hosts for CSM, combining agent and agentless host data, with optional pagination and filtering.
    pub async fn list_csm_unified_hosts_with_http_info(
        &self,
        params: ListCSMUnifiedHostsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CsmUnifiedHostsResponse>,
        datadog::Error<ListCSMUnifiedHostsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_csm_unified_hosts";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_csm_unified_hosts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page = params.page;
        let size = params.size;
        let query = params.query;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/csm/settings/hosts",
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
            match serde_json::from_str::<crate::datadogV2::model::CsmUnifiedHostsResponse>(
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
            let local_entity: Option<ListCSMUnifiedHostsError> =
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
