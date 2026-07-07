// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// GetOrgAuthorizedClientOptionalParams is a struct for passing parameters to the method [`OrgAuthorizedClientsAPI::get_org_authorized_client`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetOrgAuthorizedClientOptionalParams {
    /// Comma-separated list of related resources to include.
    /// Options: `oauth2_client`, `oauth2_client.app`, `oauth2_client.scopes`, `user_authorized_clients.user`.
    pub include: Option<String>,
    /// Filter included user authorized clients by disabled status.
    pub filter_user_authorized_clients_disabled: Option<String>,
    /// Filter included user authorized clients by user disabled status.
    pub filter_user_authorized_clients_user_disabled: Option<String>,
}

impl GetOrgAuthorizedClientOptionalParams {
    /// Comma-separated list of related resources to include.
    /// Options: `oauth2_client`, `oauth2_client.app`, `oauth2_client.scopes`, `user_authorized_clients.user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
    /// Filter included user authorized clients by disabled status.
    pub fn filter_user_authorized_clients_disabled(mut self, value: String) -> Self {
        self.filter_user_authorized_clients_disabled = Some(value);
        self
    }
    /// Filter included user authorized clients by user disabled status.
    pub fn filter_user_authorized_clients_user_disabled(mut self, value: String) -> Self {
        self.filter_user_authorized_clients_user_disabled = Some(value);
        self
    }
}

/// ListOrgAuthorizedClientUserAuthorizationsOptionalParams is a struct for passing parameters to the method [`OrgAuthorizedClientsAPI::list_org_authorized_client_user_authorizations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListOrgAuthorizedClientUserAuthorizationsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Field to sort results by. Options: `user.name`, `user.email`, `oauth2_client.name`.
    pub sort: Option<crate::datadogV2::model::OrgAuthorizedClientUserAuthorizationsSort>,
    /// Filter results by the user authorization disabled status.
    pub filter_disabled: Option<String>,
    /// Filter results by user name.
    pub filter_user_name: Option<String>,
    /// Filter results by user email.
    pub filter_user_email: Option<String>,
    /// Filter results by whether the user is disabled.
    pub filter_user_disabled: Option<String>,
}

impl ListOrgAuthorizedClientUserAuthorizationsOptionalParams {
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
    /// Field to sort results by. Options: `user.name`, `user.email`, `oauth2_client.name`.
    pub fn sort(
        mut self,
        value: crate::datadogV2::model::OrgAuthorizedClientUserAuthorizationsSort,
    ) -> Self {
        self.sort = Some(value);
        self
    }
    /// Filter results by the user authorization disabled status.
    pub fn filter_disabled(mut self, value: String) -> Self {
        self.filter_disabled = Some(value);
        self
    }
    /// Filter results by user name.
    pub fn filter_user_name(mut self, value: String) -> Self {
        self.filter_user_name = Some(value);
        self
    }
    /// Filter results by user email.
    pub fn filter_user_email(mut self, value: String) -> Self {
        self.filter_user_email = Some(value);
        self
    }
    /// Filter results by whether the user is disabled.
    pub fn filter_user_disabled(mut self, value: String) -> Self {
        self.filter_user_disabled = Some(value);
        self
    }
}

/// ListOrgAuthorizedClientsOptionalParams is a struct for passing parameters to the method [`OrgAuthorizedClientsAPI::list_org_authorized_clients`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListOrgAuthorizedClientsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Field to sort results by. Options include `oauth2_client.name`.
    pub sort: Option<String>,
    /// Filter results by client name, app title, or app description.
    pub filter: Option<String>,
    /// Filter results by the OAuth2 client name.
    pub filter_oauth2_client_name: Option<String>,
    /// Filter results by the org-level disabled status.
    pub filter_disabled: Option<String>,
    /// Comma-separated list of related resources to include.
    /// Options: `oauth2_client`, `oauth2_client.app`, `user_authorized_clients.user`.
    pub include: Option<String>,
}

impl ListOrgAuthorizedClientsOptionalParams {
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
    /// Field to sort results by. Options include `oauth2_client.name`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Filter results by client name, app title, or app description.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Filter results by the OAuth2 client name.
    pub fn filter_oauth2_client_name(mut self, value: String) -> Self {
        self.filter_oauth2_client_name = Some(value);
        self
    }
    /// Filter results by the org-level disabled status.
    pub fn filter_disabled(mut self, value: String) -> Self {
        self.filter_disabled = Some(value);
        self
    }
    /// Comma-separated list of related resources to include.
    /// Options: `oauth2_client`, `oauth2_client.app`, `user_authorized_clients.user`.
    pub fn include(mut self, value: String) -> Self {
        self.include = Some(value);
        self
    }
}

/// DeleteOrgAuthorizedClientError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::delete_org_authorized_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrgAuthorizedClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteOrgAuthorizedClientAllUserAuthorizationsError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::delete_org_authorized_client_all_user_authorizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrgAuthorizedClientAllUserAuthorizationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteOrgAuthorizedClientUserAuthorizationError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::delete_org_authorized_client_user_authorization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrgAuthorizedClientUserAuthorizationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOrgAuthorizedClientError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::get_org_authorized_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrgAuthorizedClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOrgAuthorizedClientUserAuthorizationsError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::list_org_authorized_client_user_authorizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgAuthorizedClientUserAuthorizationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOrgAuthorizedClientsError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::list_org_authorized_clients`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgAuthorizedClientsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateOrgAuthorizedClientError is a struct for typed errors of method [`OrgAuthorizedClientsAPI::update_org_authorized_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgAuthorizedClientError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage OAuth2 client authorizations at the organization level.
#[derive(Debug, Clone)]
pub struct OrgAuthorizedClientsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OrgAuthorizedClientsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl OrgAuthorizedClientsAPI {
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

    /// Disable an OAuth2 client authorization for the current organization, revoking access for all users.
    pub async fn delete_org_authorized_client(
        &self,
        org_authorized_client_id: String,
    ) -> Result<(), datadog::Error<DeleteOrgAuthorizedClientError>> {
        match self
            .delete_org_authorized_client_with_http_info(org_authorized_client_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable an OAuth2 client authorization for the current organization, revoking access for all users.
    pub async fn delete_org_authorized_client_with_http_info(
        &self,
        org_authorized_client_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteOrgAuthorizedClientError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_org_authorized_client";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}",
            local_configuration.get_operation_host(operation_id),
            org_authorized_client_id = datadog::urlencode(org_authorized_client_id)
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
            let local_entity: Option<DeleteOrgAuthorizedClientError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Disable all authorizations for a specific user for the specified OAuth2 client in the current organization.
    pub async fn delete_org_authorized_client_all_user_authorizations(
        &self,
        org_authorized_client_id: String,
        user_id: String,
    ) -> Result<(), datadog::Error<DeleteOrgAuthorizedClientAllUserAuthorizationsError>> {
        match self
            .delete_org_authorized_client_all_user_authorizations_with_http_info(
                org_authorized_client_id,
                user_id,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable all authorizations for a specific user for the specified OAuth2 client in the current organization.
    pub async fn delete_org_authorized_client_all_user_authorizations_with_http_info(
        &self,
        org_authorized_client_id: String,
        user_id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteOrgAuthorizedClientAllUserAuthorizationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_org_authorized_client_all_user_authorizations";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}/user/{user_id}",
            local_configuration.get_operation_host(operation_id),
            org_authorized_client_id = datadog::urlencode(org_authorized_client_id),
            user_id = datadog::urlencode(user_id)
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
            let local_entity: Option<DeleteOrgAuthorizedClientAllUserAuthorizationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Disable a specific user authorization for the specified OAuth2 client in the current organization.
    pub async fn delete_org_authorized_client_user_authorization(
        &self,
        org_authorized_client_id: String,
        user_authorized_client_id: String,
    ) -> Result<(), datadog::Error<DeleteOrgAuthorizedClientUserAuthorizationError>> {
        match self
            .delete_org_authorized_client_user_authorization_with_http_info(
                org_authorized_client_id,
                user_authorized_client_id,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable a specific user authorization for the specified OAuth2 client in the current organization.
    pub async fn delete_org_authorized_client_user_authorization_with_http_info(
        &self,
        org_authorized_client_id: String,
        user_authorized_client_id: String,
    ) -> Result<
        datadog::ResponseContent<()>,
        datadog::Error<DeleteOrgAuthorizedClientUserAuthorizationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_org_authorized_client_user_authorization";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}/user_authorized_clients/{user_authorized_client_id}",
            local_configuration.get_operation_host(operation_id), org_authorized_client_id=
            datadog::urlencode(org_authorized_client_id)
            , user_authorized_client_id=
            datadog::urlencode(user_authorized_client_id)
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
            let local_entity: Option<DeleteOrgAuthorizedClientUserAuthorizationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single OAuth2 client authorized for the current organization.
    pub async fn get_org_authorized_client(
        &self,
        org_authorized_client_id: String,
        params: GetOrgAuthorizedClientOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OrgAuthorizedClientResponse,
        datadog::Error<GetOrgAuthorizedClientError>,
    > {
        match self
            .get_org_authorized_client_with_http_info(org_authorized_client_id, params)
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

    /// Get a single OAuth2 client authorized for the current organization.
    pub async fn get_org_authorized_client_with_http_info(
        &self,
        org_authorized_client_id: String,
        params: GetOrgAuthorizedClientOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OrgAuthorizedClientResponse>,
        datadog::Error<GetOrgAuthorizedClientError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_org_authorized_client";

        // unbox and build optional parameters
        let include = params.include;
        let filter_user_authorized_clients_disabled =
            params.filter_user_authorized_clients_disabled;
        let filter_user_authorized_clients_user_disabled =
            params.filter_user_authorized_clients_user_disabled;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}",
            local_configuration.get_operation_host(operation_id),
            org_authorized_client_id = datadog::urlencode(org_authorized_client_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_authorized_clients_disabled {
            local_req_builder = local_req_builder.query(&[(
                "filter[user_authorized_clients][disabled]",
                &local_query_param.to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_user_authorized_clients_user_disabled {
            local_req_builder = local_req_builder.query(&[(
                "filter[user_authorized_clients][user][disabled]",
                &local_query_param.to_string(),
            )]);
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
            match serde_json::from_str::<crate::datadogV2::model::OrgAuthorizedClientResponse>(
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
            let local_entity: Option<GetOrgAuthorizedClientError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of user authorizations for the specified OAuth2 client in the current organization.
    pub async fn list_org_authorized_client_user_authorizations(
        &self,
        org_authorized_client_id: String,
        params: ListOrgAuthorizedClientUserAuthorizationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::UserAuthorizedClientsResponse,
        datadog::Error<ListOrgAuthorizedClientUserAuthorizationsError>,
    > {
        match self
            .list_org_authorized_client_user_authorizations_with_http_info(
                org_authorized_client_id,
                params,
            )
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

    pub fn list_org_authorized_client_user_authorizations_with_pagination(
        &self,
        org_authorized_client_id: String,
        mut params: ListOrgAuthorizedClientUserAuthorizationsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::UserAuthorizedClientData,
            datadog::Error<ListOrgAuthorizedClientUserAuthorizationsError>,
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
                let resp = self.list_org_authorized_client_user_authorizations( org_authorized_client_id.clone(),params.clone()).await?;

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

    /// Get a list of user authorizations for the specified OAuth2 client in the current organization.
    pub async fn list_org_authorized_client_user_authorizations_with_http_info(
        &self,
        org_authorized_client_id: String,
        params: ListOrgAuthorizedClientUserAuthorizationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UserAuthorizedClientsResponse>,
        datadog::Error<ListOrgAuthorizedClientUserAuthorizationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_org_authorized_client_user_authorizations";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter_disabled = params.filter_disabled;
        let filter_user_name = params.filter_user_name;
        let filter_user_email = params.filter_user_email;
        let filter_user_disabled = params.filter_user_disabled;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}/user_authorized_clients",
            local_configuration.get_operation_host(operation_id),
            org_authorized_client_id = datadog::urlencode(org_authorized_client_id)
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
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_disabled {
            local_req_builder =
                local_req_builder.query(&[("filter[disabled]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_name {
            local_req_builder =
                local_req_builder.query(&[("filter[user][name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_email {
            local_req_builder =
                local_req_builder.query(&[("filter[user][email]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_user_disabled {
            local_req_builder = local_req_builder
                .query(&[("filter[user][disabled]", &local_query_param.to_string())]);
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
            let local_entity: Option<ListOrgAuthorizedClientUserAuthorizationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of all OAuth2 clients authorized for the current organization.
    pub async fn list_org_authorized_clients(
        &self,
        params: ListOrgAuthorizedClientsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::OrgAuthorizedClientsResponse,
        datadog::Error<ListOrgAuthorizedClientsError>,
    > {
        match self
            .list_org_authorized_clients_with_http_info(params)
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

    pub fn list_org_authorized_clients_with_pagination(
        &self,
        mut params: ListOrgAuthorizedClientsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::OrgAuthorizedClientData,
            datadog::Error<ListOrgAuthorizedClientsError>,
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
                let resp = self.list_org_authorized_clients(params.clone()).await?;

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

    /// Get a list of all OAuth2 clients authorized for the current organization.
    pub async fn list_org_authorized_clients_with_http_info(
        &self,
        params: ListOrgAuthorizedClientsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OrgAuthorizedClientsResponse>,
        datadog::Error<ListOrgAuthorizedClientsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_org_authorized_clients";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_oauth2_client_name = params.filter_oauth2_client_name;
        let filter_disabled = params.filter_disabled;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients",
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
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_oauth2_client_name {
            local_req_builder = local_req_builder.query(&[(
                "filter[oauth2_client][name]",
                &local_query_param.to_string(),
            )]);
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
            match serde_json::from_str::<crate::datadogV2::model::OrgAuthorizedClientsResponse>(
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
            let local_entity: Option<ListOrgAuthorizedClientsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Enable or disable an OAuth2 client authorization for the current organization.
    pub async fn update_org_authorized_client(
        &self,
        org_authorized_client_id: String,
        body: crate::datadogV2::model::OrgAuthorizedClientUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::OrgAuthorizedClientResponse,
        datadog::Error<UpdateOrgAuthorizedClientError>,
    > {
        match self
            .update_org_authorized_client_with_http_info(org_authorized_client_id, body)
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

    /// Enable or disable an OAuth2 client authorization for the current organization.
    pub async fn update_org_authorized_client_with_http_info(
        &self,
        org_authorized_client_id: String,
        body: crate::datadogV2::model::OrgAuthorizedClientUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::OrgAuthorizedClientResponse>,
        datadog::Error<UpdateOrgAuthorizedClientError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_org_authorized_client";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/org_authorized_clients/{org_authorized_client_id}",
            local_configuration.get_operation_host(operation_id),
            org_authorized_client_id = datadog::urlencode(org_authorized_client_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    #[cfg(feature = "zstd")]
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::OrgAuthorizedClientResponse>(
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
            let local_entity: Option<UpdateOrgAuthorizedClientError> =
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
