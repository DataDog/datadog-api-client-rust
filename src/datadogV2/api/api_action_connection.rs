// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

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

/// ListAppKeyRegistrationsError is a struct for typed errors of method [`ActionConnectionAPI::list_app_key_registrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAppKeyRegistrationsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
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

    /// Delete an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
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

    /// Delete an existing Action Connection. This API requires a [registered application key](<https://docs.datadoghq.com/api/latest/action-connection/#register-a-new-app-key>).
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
}
