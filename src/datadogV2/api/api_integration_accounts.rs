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

/// CreateIntegrationAccountError is a struct for typed errors of method [`IntegrationAccountsAPI::create_integration_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIntegrationAccountError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIntegrationAccountError is a struct for typed errors of method [`IntegrationAccountsAPI::delete_integration_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIntegrationAccountError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIntegrationAccountError is a struct for typed errors of method [`IntegrationAccountsAPI::get_integration_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIntegrationAccountError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIntegrationAccountsError is a struct for typed errors of method [`IntegrationAccountsAPI::list_integration_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIntegrationAccountsError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIntegrationAccountError is a struct for typed errors of method [`IntegrationAccountsAPI::update_integration_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIntegrationAccountError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Manage accounts for Datadog integrations served by the Account Management Service (AMS). The account payload is strongly typed per integration and interface.
#[derive(Debug, Clone)]
pub struct IntegrationAccountsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for IntegrationAccountsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl IntegrationAccountsAPI {
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

    /// Create an integration account for a given integration/interface.
    pub async fn create_integration_account(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        body: crate::datadogV2::model::IntegrationAccountRequest,
    ) -> Result<
        crate::datadogV2::model::IntegrationAccountResponse,
        datadog::Error<CreateIntegrationAccountError>,
    > {
        match self
            .create_integration_account_with_http_info(integration_id, interface_id, body)
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

    /// Create an integration account for a given integration/interface.
    pub async fn create_integration_account_with_http_info(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        body: crate::datadogV2::model::IntegrationAccountRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::IntegrationAccountResponse>,
        datadog::Error<CreateIntegrationAccountError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.create_integration_account";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.create_integration_account' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/{integration_id}/interfaces/{interface_id}/accounts",
            local_configuration.get_operation_host(local_operation_id),
            integration_id = datadog::urlencode(integration_id.to_string()),
            interface_id = datadog::urlencode(interface_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::IntegrationAccountResponse>(
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
            let local_entity: Option<CreateIntegrationAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Delete an integration account.
    pub async fn delete_integration_account(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
    ) -> Result<(), datadog::Error<DeleteIntegrationAccountError>> {
        match self
            .delete_integration_account_with_http_info(integration_id, interface_id, account_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an integration account.
    pub async fn delete_integration_account_with_http_info(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteIntegrationAccountError>> {
        let local_configuration = &self.config;
        let local_operation_id = "v2.delete_integration_account";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_integration_account' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/{integration_id}/interfaces/{interface_id}/accounts/{account_id}",
            local_configuration.get_operation_host(local_operation_id), integration_id=
            datadog::urlencode(integration_id.to_string())
            , interface_id=
            datadog::urlencode(interface_id.to_string())
            , account_id=
            datadog::urlencode(account_id)
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
            let local_entity: Option<DeleteIntegrationAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a single integration account.
    pub async fn get_integration_account(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
    ) -> Result<
        crate::datadogV2::model::IntegrationAccountResponse,
        datadog::Error<GetIntegrationAccountError>,
    > {
        match self
            .get_integration_account_with_http_info(integration_id, interface_id, account_id)
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

    /// Get a single integration account.
    pub async fn get_integration_account_with_http_info(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::IntegrationAccountResponse>,
        datadog::Error<GetIntegrationAccountError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.get_integration_account";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.get_integration_account' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/{integration_id}/interfaces/{interface_id}/accounts/{account_id}",
            local_configuration.get_operation_host(local_operation_id), integration_id=
            datadog::urlencode(integration_id.to_string())
            , interface_id=
            datadog::urlencode(interface_id.to_string())
            , account_id=
            datadog::urlencode(account_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IntegrationAccountResponse>(
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
            let local_entity: Option<GetIntegrationAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// List the integration accounts for a given integration/interface.
    pub async fn list_integration_accounts(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
    ) -> Result<
        crate::datadogV2::model::IntegrationAccountsResponse,
        datadog::Error<ListIntegrationAccountsError>,
    > {
        match self
            .list_integration_accounts_with_http_info(integration_id, interface_id)
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

    /// List the integration accounts for a given integration/interface.
    pub async fn list_integration_accounts_with_http_info(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::IntegrationAccountsResponse>,
        datadog::Error<ListIntegrationAccountsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_integration_accounts";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_integration_accounts' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/{integration_id}/interfaces/{interface_id}/accounts",
            local_configuration.get_operation_host(local_operation_id),
            integration_id = datadog::urlencode(integration_id.to_string()),
            interface_id = datadog::urlencode(interface_id.to_string())
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
            match serde_json::from_str::<crate::datadogV2::model::IntegrationAccountsResponse>(
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
            let local_entity: Option<ListIntegrationAccountsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update an integration account. The update is a partial merge: only the fields provided are changed, so a name-only or settings-only update does not need to resend the full integration payload or write-only credentials. When present, `type` selects the integration/interface variant. Top-level attributes and the contents of `authentication` and `settings` are merged one level deep; the `dataflows` array is merged by `id`.
    pub async fn update_integration_account(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
        body: crate::datadogV2::model::IntegrationAccountUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::IntegrationAccountResponse,
        datadog::Error<UpdateIntegrationAccountError>,
    > {
        match self
            .update_integration_account_with_http_info(
                integration_id,
                interface_id,
                account_id,
                body,
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

    /// Update an integration account. The update is a partial merge: only the fields provided are changed, so a name-only or settings-only update does not need to resend the full integration payload or write-only credentials. When present, `type` selects the integration/interface variant. Top-level attributes and the contents of `authentication` and `settings` are merged one level deep; the `dataflows` array is merged by `id`.
    pub async fn update_integration_account_with_http_info(
        &self,
        integration_id: crate::datadogV2::model::IntegrationAccountIntegrationId,
        interface_id: crate::datadogV2::model::IntegrationAccountInterfaceId,
        account_id: String,
        body: crate::datadogV2::model::IntegrationAccountUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::IntegrationAccountResponse>,
        datadog::Error<UpdateIntegrationAccountError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.update_integration_account";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.update_integration_account' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/{integration_id}/interfaces/{interface_id}/accounts/{account_id}",
            local_configuration.get_operation_host(local_operation_id), integration_id=
            datadog::urlencode(integration_id.to_string())
            , interface_id=
            datadog::urlencode(interface_id.to_string())
            , account_id=
            datadog::urlencode(account_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IntegrationAccountResponse>(
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
            let local_entity: Option<UpdateIntegrationAccountError> =
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
