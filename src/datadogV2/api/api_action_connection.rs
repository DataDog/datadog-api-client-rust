// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListActionConnectionsOptionalParams is a struct for passing parameters to the method [`ActionConnectionAPI::list_action_connections`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListActionConnectionsOptionalParams {
    /// The number of connections to return per page.
    pub page_size: Option<i64>,
    /// The page number to return.
    pub page_number: Option<i64>,
    /// Filter by integration type.
    pub filter_integration: Option<Vec<String>>,
    /// Filter by tags.
    pub filter_tags: Option<Vec<String>>,
    /// Filter by environment.
    pub filter_environment: Option<String>,
    /// Filter by connection IDs.
    pub filter_connection_ids: Option<Vec<String>>,
    /// Filter by creator IDs.
    pub filter_creator_ids: Option<Vec<String>>,
    /// Search string to filter connections.
    pub filter_search: Option<String>,
    /// Sort parameters.
    pub sort: Option<Vec<String>>,
}

impl ListActionConnectionsOptionalParams {
    /// The number of connections to return per page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by integration type.
    pub fn filter_integration(mut self, value: Vec<String>) -> Self {
        self.filter_integration = Some(value);
        self
    }
    /// Filter by tags.
    pub fn filter_tags(mut self, value: Vec<String>) -> Self {
        self.filter_tags = Some(value);
        self
    }
    /// Filter by environment.
    pub fn filter_environment(mut self, value: String) -> Self {
        self.filter_environment = Some(value);
        self
    }
    /// Filter by connection IDs.
    pub fn filter_connection_ids(mut self, value: Vec<String>) -> Self {
        self.filter_connection_ids = Some(value);
        self
    }
    /// Filter by creator IDs.
    pub fn filter_creator_ids(mut self, value: Vec<String>) -> Self {
        self.filter_creator_ids = Some(value);
        self
    }
    /// Search string to filter connections.
    pub fn filter_search(mut self, value: String) -> Self {
        self.filter_search = Some(value);
        self
    }
    /// Sort parameters.
    pub fn sort(mut self, value: Vec<String>) -> Self {
        self.sort = Some(value);
        self
    }
}

/// ListAppKeyRegistrationsOptionalParams is a struct for passing parameters to the method [`ActionConnectionAPI::list_app_key_registrations`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListAppKeyRegistrationsOptionalParams {
    /// The number of App Key Registrations to return per page.
    pub page_size: Option<i64>,
    /// The page number to return.
    pub page_number: Option<i64>,
}

impl ListAppKeyRegistrationsOptionalParams {
    /// The number of App Key Registrations to return per page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// ListConnectionGroupsOptionalParams is a struct for passing parameters to the method [`ActionConnectionAPI::list_connection_groups`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListConnectionGroupsOptionalParams {
    /// The number of connection groups to return per page.
    pub page_size: Option<i64>,
    /// The page number to return.
    pub page_number: Option<i64>,
    /// Filter by integration type.
    pub filter_integration: Option<Vec<String>>,
    /// Filter by environment.
    pub filter_environment: Option<String>,
    /// Filter by connection group IDs.
    pub filter_connection_group_ids: Option<Vec<String>>,
    /// Filter by creator ID.
    pub filter_creator_id: Option<String>,
    /// Filter by creator IDs.
    pub filter_creator_ids: Option<Vec<String>>,
    /// Search string to filter connection groups.
    pub filter_search: Option<String>,
    /// Sort parameters.
    pub sort: Option<Vec<String>>,
}

impl ListConnectionGroupsOptionalParams {
    /// The number of connection groups to return per page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// The page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// Filter by integration type.
    pub fn filter_integration(mut self, value: Vec<String>) -> Self {
        self.filter_integration = Some(value);
        self
    }
    /// Filter by environment.
    pub fn filter_environment(mut self, value: String) -> Self {
        self.filter_environment = Some(value);
        self
    }
    /// Filter by connection group IDs.
    pub fn filter_connection_group_ids(mut self, value: Vec<String>) -> Self {
        self.filter_connection_group_ids = Some(value);
        self
    }
    /// Filter by creator ID.
    pub fn filter_creator_id(mut self, value: String) -> Self {
        self.filter_creator_id = Some(value);
        self
    }
    /// Filter by creator IDs.
    pub fn filter_creator_ids(mut self, value: Vec<String>) -> Self {
        self.filter_creator_ids = Some(value);
        self
    }
    /// Search string to filter connection groups.
    pub fn filter_search(mut self, value: String) -> Self {
        self.filter_search = Some(value);
        self
    }
    /// Sort parameters.
    pub fn sort(mut self, value: Vec<String>) -> Self {
        self.sort = Some(value);
        self
    }
}

/// CreateActionConnectionError is a struct for typed errors of method [`ActionConnectionAPI::create_action_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateActionConnectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteActionConnectionError is a struct for typed errors of method [`ActionConnectionAPI::delete_action_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteActionConnectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetActionConnectionError is a struct for typed errors of method [`ActionConnectionAPI::get_action_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetActionConnectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAppKeyRegistrationError is a struct for typed errors of method [`ActionConnectionAPI::get_app_key_registration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppKeyRegistrationError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListActionConnectionsError is a struct for typed errors of method [`ActionConnectionAPI::list_action_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListActionConnectionsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAppKeyRegistrationsError is a struct for typed errors of method [`ActionConnectionAPI::list_app_key_registrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAppKeyRegistrationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListConnectionGroupsError is a struct for typed errors of method [`ActionConnectionAPI::list_connection_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConnectionGroupsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RegisterAppKeyError is a struct for typed errors of method [`ActionConnectionAPI::register_app_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterAppKeyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnregisterAppKeyError is a struct for typed errors of method [`ActionConnectionAPI::unregister_app_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnregisterAppKeyError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateActionConnectionError is a struct for typed errors of method [`ActionConnectionAPI::update_action_connection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateActionConnectionError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateConnectionGroupError is a struct for typed errors of method [`ActionConnectionAPI::update_connection_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConnectionGroupError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Action connections extend your installed integrations and allow you to take action in your third-party systems
/// (e.g. AWS, GitLab, and Statuspage) with Datadog’s Workflow Automation and App Builder products.
///
/// Datadog’s Integrations automatically provide authentication for Slack, Microsoft Teams, PagerDuty, Opsgenie,
/// JIRA, GitHub, and Statuspage. You do not need additional connections in order to access these tools within
/// Workflow Automation and App Builder.
///
/// We offer granular access control for editing and resolving connections.
#[derive(Debug, Clone)]
pub struct ActionConnectionAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ActionConnectionAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ActionConnectionAPI {
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

    /// Create a new Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn create_action_connection(
        &self,
        body: crate::datadogV2::model::CreateActionConnectionRequest,
    ) -> Result<
        crate::datadogV2::model::CreateActionConnectionResponse,
        datadog::Error<CreateActionConnectionError>,
    > {
        match self.create_action_connection_with_http_info(body).await {
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

    /// Create a new Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn create_action_connection_with_http_info(
        &self,
        body: crate::datadogV2::model::CreateActionConnectionRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::CreateActionConnectionResponse>,
        datadog::Error<CreateActionConnectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_action_connection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::CreateActionConnectionResponse>(
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
            let local_entity: Option<CreateActionConnectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_action_connection(
        &self,
        connection_id: String,
    ) -> Result<(), datadog::Error<DeleteActionConnectionError>> {
        match self
            .delete_action_connection_with_http_info(connection_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>). Alternatively, you can configure these permissions [in the UI](<https://docs.datadoghq.com/account_management/api-app-keys/#actions-api-access>).
    pub async fn delete_action_connection_with_http_info(
        &self,
        connection_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteActionConnectionError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_action_connection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections/{connection_id}",
            local_configuration.get_operation_host(operation_id),
            connection_id = datadog::urlencode(connection_id)
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
            let local_entity: Option<DeleteActionConnectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn get_action_connection(
        &self,
        connection_id: String,
    ) -> Result<
        crate::datadogV2::model::GetActionConnectionResponse,
        datadog::Error<GetActionConnectionError>,
    > {
        match self
            .get_action_connection_with_http_info(connection_id)
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

    /// Get an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn get_action_connection_with_http_info(
        &self,
        connection_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetActionConnectionResponse>,
        datadog::Error<GetActionConnectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_action_connection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections/{connection_id}",
            local_configuration.get_operation_host(operation_id),
            connection_id = datadog::urlencode(connection_id)
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
            match serde_json::from_str::<crate::datadogV2::model::GetActionConnectionResponse>(
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
            let local_entity: Option<GetActionConnectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an existing App Key Registration
    pub async fn get_app_key_registration(
        &self,
        app_key_id: String,
    ) -> Result<
        crate::datadogV2::model::GetAppKeyRegistrationResponse,
        datadog::Error<GetAppKeyRegistrationError>,
    > {
        match self
            .get_app_key_registration_with_http_info(app_key_id)
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

    /// Get an existing App Key Registration
    pub async fn get_app_key_registration_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::GetAppKeyRegistrationResponse>,
        datadog::Error<GetAppKeyRegistrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_app_key_registration";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/app_key_registrations/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = datadog::urlencode(app_key_id)
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
            match serde_json::from_str::<crate::datadogV2::model::GetAppKeyRegistrationResponse>(
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
            let local_entity: Option<GetAppKeyRegistrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all action connections for the organization. This endpoint supports filtering by integration type, tags, environment, and search strings. Pagination is supported using page size and page number parameters.
    pub async fn list_action_connections(
        &self,
        params: ListActionConnectionsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListActionConnectionsResponse,
        datadog::Error<ListActionConnectionsError>,
    > {
        match self.list_action_connections_with_http_info(params).await {
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

    /// List all action connections for the organization. This endpoint supports filtering by integration type, tags, environment, and search strings. Pagination is supported using page size and page number parameters.
    pub async fn list_action_connections_with_http_info(
        &self,
        params: ListActionConnectionsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListActionConnectionsResponse>,
        datadog::Error<ListActionConnectionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_action_connections";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_action_connections' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let filter_integration = params.filter_integration;
        let filter_tags = params.filter_tags;
        let filter_environment = params.filter_environment;
        let filter_connection_ids = params.filter_connection_ids;
        let filter_creator_ids = params.filter_creator_ids;
        let filter_search = params.filter_search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections",
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
        if let Some(ref local) = filter_integration {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[integration]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_tags {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[tags]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_environment {
            local_req_builder =
                local_req_builder.query(&[("filter[environment]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_connection_ids {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[connection_ids]", &param.to_string())]);
            }
        };
        if let Some(ref local) = filter_creator_ids {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[creator_ids]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_search {
            local_req_builder =
                local_req_builder.query(&[("filter[search]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = sort {
            for param in local {
                local_req_builder = local_req_builder.query(&[("sort", &param.to_string())]);
            }
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
            match serde_json::from_str::<crate::datadogV2::model::ListActionConnectionsResponse>(
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
            let local_entity: Option<ListActionConnectionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List App Key Registrations
    pub async fn list_app_key_registrations(
        &self,
        params: ListAppKeyRegistrationsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListAppKeyRegistrationsResponse,
        datadog::Error<ListAppKeyRegistrationsError>,
    > {
        match self.list_app_key_registrations_with_http_info(params).await {
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

    /// List App Key Registrations
    pub async fn list_app_key_registrations_with_http_info(
        &self,
        params: ListAppKeyRegistrationsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListAppKeyRegistrationsResponse>,
        datadog::Error<ListAppKeyRegistrationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_app_key_registrations";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/app_key_registrations",
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
            match serde_json::from_str::<crate::datadogV2::model::ListAppKeyRegistrationsResponse>(
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
            let local_entity: Option<ListAppKeyRegistrationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List all connection groups for the organization. This endpoint supports filtering by integration type, environment, connection group IDs, and search strings. Pagination is supported using page size and page number parameters.
    pub async fn list_connection_groups(
        &self,
        params: ListConnectionGroupsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListConnectionGroupsResponse,
        datadog::Error<ListConnectionGroupsError>,
    > {
        match self.list_connection_groups_with_http_info(params).await {
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

    /// List all connection groups for the organization. This endpoint supports filtering by integration type, environment, connection group IDs, and search strings. Pagination is supported using page size and page number parameters.
    pub async fn list_connection_groups_with_http_info(
        &self,
        params: ListConnectionGroupsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListConnectionGroupsResponse>,
        datadog::Error<ListConnectionGroupsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_connection_groups";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_connection_groups' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let filter_integration = params.filter_integration;
        let filter_environment = params.filter_environment;
        let filter_connection_group_ids = params.filter_connection_group_ids;
        let filter_creator_id = params.filter_creator_id;
        let filter_creator_ids = params.filter_creator_ids;
        let filter_search = params.filter_search;
        let sort = params.sort;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections/groups",
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
        if let Some(ref local) = filter_integration {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[integration]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_environment {
            local_req_builder =
                local_req_builder.query(&[("filter[environment]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_connection_group_ids {
            for param in local {
                local_req_builder = local_req_builder
                    .query(&[("filter[connection_group_ids]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_creator_id {
            local_req_builder =
                local_req_builder.query(&[("filter[creator_id]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = filter_creator_ids {
            for param in local {
                local_req_builder =
                    local_req_builder.query(&[("filter[creator_ids]", &param.to_string())]);
            }
        };
        if let Some(ref local_query_param) = filter_search {
            local_req_builder =
                local_req_builder.query(&[("filter[search]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = sort {
            for param in local {
                local_req_builder = local_req_builder.query(&[("sort", &param.to_string())]);
            }
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
            match serde_json::from_str::<crate::datadogV2::model::ListConnectionGroupsResponse>(
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
            let local_entity: Option<ListConnectionGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Register a new App Key
    pub async fn register_app_key(
        &self,
        app_key_id: String,
    ) -> Result<crate::datadogV2::model::RegisterAppKeyResponse, datadog::Error<RegisterAppKeyError>>
    {
        match self.register_app_key_with_http_info(app_key_id).await {
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

    /// Register a new App Key
    pub async fn register_app_key_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RegisterAppKeyResponse>,
        datadog::Error<RegisterAppKeyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.register_app_key";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/app_key_registrations/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = datadog::urlencode(app_key_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::RegisterAppKeyResponse>(
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
            let local_entity: Option<RegisterAppKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Unregister an App Key
    pub async fn unregister_app_key(
        &self,
        app_key_id: String,
    ) -> Result<(), datadog::Error<UnregisterAppKeyError>> {
        match self.unregister_app_key_with_http_info(app_key_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Unregister an App Key
    pub async fn unregister_app_key_with_http_info(
        &self,
        app_key_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<UnregisterAppKeyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.unregister_app_key";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/app_key_registrations/{app_key_id}",
            local_configuration.get_operation_host(operation_id),
            app_key_id = datadog::urlencode(app_key_id)
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
            let local_entity: Option<UnregisterAppKeyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn update_action_connection(
        &self,
        connection_id: String,
        body: crate::datadogV2::model::UpdateActionConnectionRequest,
    ) -> Result<
        crate::datadogV2::model::UpdateActionConnectionResponse,
        datadog::Error<UpdateActionConnectionError>,
    > {
        match self
            .update_action_connection_with_http_info(connection_id, body)
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

    /// Update an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
    pub async fn update_action_connection_with_http_info(
        &self,
        connection_id: String,
        body: crate::datadogV2::model::UpdateActionConnectionRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateActionConnectionResponse>,
        datadog::Error<UpdateActionConnectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_action_connection";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections/{connection_id}",
            local_configuration.get_operation_host(operation_id),
            connection_id = datadog::urlencode(connection_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UpdateActionConnectionResponse>(
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
            let local_entity: Option<UpdateActionConnectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an existing connection group by ID. This endpoint allows updating the name, description, tag keys, integration type, connections, and policy attributes of a connection group.
    pub async fn update_connection_group(
        &self,
        connection_group_id: String,
        body: crate::datadogV2::model::UpdateConnectionGroupRequest,
    ) -> Result<
        crate::datadogV2::model::UpdateConnectionGroupResponse,
        datadog::Error<UpdateConnectionGroupError>,
    > {
        match self
            .update_connection_group_with_http_info(connection_group_id, body)
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

    /// Update an existing connection group by ID. This endpoint allows updating the name, description, tag keys, integration type, connections, and policy attributes of a connection group.
    pub async fn update_connection_group_with_http_info(
        &self,
        connection_group_id: String,
        body: crate::datadogV2::model::UpdateConnectionGroupRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpdateConnectionGroupResponse>,
        datadog::Error<UpdateConnectionGroupError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_connection_group";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_connection_group' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/actions/connections/groups/{connection_group_id}",
            local_configuration.get_operation_host(operation_id),
            connection_group_id = datadog::urlencode(connection_group_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UpdateConnectionGroupResponse>(
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
            let local_entity: Option<UpdateConnectionGroupError> =
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
