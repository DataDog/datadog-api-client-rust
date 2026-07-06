// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListUserAuthorizedClientsOptionalParams is a struct for passing parameters to the method [`UserAuthorizedClientsAPI::list_user_authorized_clients`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListUserAuthorizedClientsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Filter results by client name, app title, or app description.
    pub filter: Option<String>,
    /// Filter results by the user-level disabled status.
    pub filter_disabled: Option<String>,
    /// Comma-separated list of related resources to include. Options: `oauth2_client`, `oauth2_client.app`.
    pub include: Option<String>,
}

impl ListUserAuthorizedClientsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter results by client name, app title, or app description.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Filter results by the user-level disabled status.
    pub fn filter_disabled(mut self, value: String) -> Self {
        self.filter_disabled = Some(value);
        self
    }
    /// Comma-separated list of related resources to include. Options: `oauth2_client`, `oauth2_client.app`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// DeleteUserAuthorizedClientError is a struct for typed errors of method [`UserAuthorizedClientsAPI::delete_user_authorized_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserAuthorizedClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteUserAuthorizedClientsByClientError is a struct for typed errors of method [`UserAuthorizedClientsAPI::delete_user_authorized_clients_by_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserAuthorizedClientsByClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetUserAuthorizedClientError is a struct for typed errors of method [`UserAuthorizedClientsAPI::get_user_authorized_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserAuthorizedClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListUserAuthorizedClientsError is a struct for typed errors of method [`UserAuthorizedClientsAPI::list_user_authorized_clients`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserAuthorizedClientsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage OAuth2 client authorizations at the user level.
#[derive(Debug, Clone)]
pub struct UserAuthorizedClientsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for UserAuthorizedClientsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl UserAuthorizedClientsAPI {
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

    /// Disable the current user's authorization for the specified OAuth2 client.
    pub async fn delete_user_authorized_client(
        &self,
        user_authorized_client_id: String,
    ) -> Result<(), datadog::Error<DeleteUserAuthorizedClientError>> {
        match self
            .delete_user_authorized_client_with_http_info(user_authorized_client_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable the current user's authorization for the specified OAuth2 client.
    pub async fn delete_user_authorized_client_with_http_info(
        &self,
        user_authorized_client_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteUserAuthorizedClientError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_user_authorized_client";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_authorized_clients/{user_authorized_client_id}",
            local_configuration.get_operation_host(operation_id),
            user_authorized_client_id = datadog::urlencode(user_authorized_client_id)
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
            let local_entity: Option<DeleteUserAuthorizedClientError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Disable all authorizations the current user has granted to the specified OAuth2 client.
    pub async fn delete_user_authorized_clients_by_client(
        &self,
        client_id: String,
    ) -> Result<(), datadog::Error<DeleteUserAuthorizedClientsByClientError>> {
        match self
            .delete_user_authorized_clients_by_client_with_http_info(client_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable all authorizations the current user has granted to the specified OAuth2 client.
    pub async fn delete_user_authorized_clients_by_client_with_http_info(
        &self,
        client_id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteUserAuthorizedClientsByClientError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_user_authorized_clients_by_client";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_authorized_clients/client/{client_id}",
            local_configuration.get_operation_host(operation_id),
            client_id = datadog::urlencode(client_id)
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
            let local_entity: Option<DeleteUserAuthorizedClientsByClientError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single OAuth2 client authorization for the current user.
    pub async fn get_user_authorized_client(
        &self,
        user_authorized_client_id: String,
    ) -> Result<
        crate::datadogV2::model::UserAuthorizedClientResponse,
        datadog::Error<GetUserAuthorizedClientError>,
    > {
        match self
            .get_user_authorized_client_with_http_info(user_authorized_client_id)
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

    /// Get a single OAuth2 client authorization for the current user.
    pub async fn get_user_authorized_client_with_http_info(
        &self,
        user_authorized_client_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UserAuthorizedClientResponse>,
        datadog::Error<GetUserAuthorizedClientError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_user_authorized_client";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_authorized_clients/{user_authorized_client_id}",
            local_configuration.get_operation_host(operation_id),
            user_authorized_client_id = datadog::urlencode(user_authorized_client_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UserAuthorizedClientResponse>(
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
            let local_entity: Option<GetUserAuthorizedClientError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of all OAuth2 clients authorized by the current user.
    pub async fn list_user_authorized_clients(
        &self,
        params: ListUserAuthorizedClientsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UserAuthorizedClientsResponse,
        datadog::Error<ListUserAuthorizedClientsError>,
    > {
        match self
            .list_user_authorized_clients_with_http_info(params)
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

    pub fn list_user_authorized_clients_with_pagination(
        &self,
        mut params: ListUserAuthorizedClientsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::UserAuthorizedClientData,
            datadog::Error<ListUserAuthorizedClientsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            if params.page_number.is_none() {
                params.page_number = Some(0);
            }
            loop {
                let resp = self.list_user_authorized_clients(params.clone()).await?;

                let r = resp.data;
                let count = r.len();
                for team in r {
                    yield team;
                }
                if count < page_size as usize {
                    break;
                }
                params.page_number = Some(params.page_number.unwrap() + 1);
            }
        }
    }

    /// Get a list of all OAuth2 clients authorized by the current user.
    pub async fn list_user_authorized_clients_with_http_info(
        &self,
        params: ListUserAuthorizedClientsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UserAuthorizedClientsResponse>,
        datadog::Error<ListUserAuthorizedClientsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_user_authorized_clients";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let filter = params.filter;
        let filter_disabled = params.filter_disabled;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_authorized_clients",
            local_configuration.get_operation_host(operation_id)
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
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_disabled {
            local_req_builder =
                local_req_builder.query(&[("filter[disabled]", &local_query_param.to_string())]);
        };
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
            match serde_json::from_str::<crate::datadogV2::model::UserAuthorizedClientsResponse>(
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
            let local_entity: Option<ListUserAuthorizedClientsError> =
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
