// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateConfluentAccountError is a struct for typed errors of method [`ConfluentCloudAPI::create_confluent_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateConfluentResourceError is a struct for typed errors of method [`ConfluentCloudAPI::create_confluent_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentAccountError is a struct for typed errors of method [`ConfluentCloudAPI::delete_confluent_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteConfluentResourceError is a struct for typed errors of method [`ConfluentCloudAPI::delete_confluent_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetConfluentAccountError is a struct for typed errors of method [`ConfluentCloudAPI::get_confluent_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetConfluentResourceError is a struct for typed errors of method [`ConfluentCloudAPI::get_confluent_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListConfluentAccountError is a struct for typed errors of method [`ConfluentCloudAPI::list_confluent_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListConfluentResourceError is a struct for typed errors of method [`ConfluentCloudAPI::list_confluent_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentAccountError is a struct for typed errors of method [`ConfluentCloudAPI::update_confluent_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentAccountError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateConfluentResourceError is a struct for typed errors of method [`ConfluentCloudAPI::update_confluent_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateConfluentResourceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ConfluentCloudAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ConfluentCloudAPI {
    fn default() -> Self {
        Self::with_config(configuration::Configuration::default())
    }
}

impl ConfluentCloudAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
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
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a Confluent account.
    pub async fn create_confluent_account(
        &self,
        body: crate::datadogV2::model::ConfluentAccountCreateRequest,
    ) -> Result<crate::datadogV2::model::ConfluentAccountResponse, Error<CreateConfluentAccountError>>
    {
        match self.create_confluent_account_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a Confluent account.
    pub async fn create_confluent_account_with_http_info(
        &self,
        body: crate::datadogV2::model::ConfluentAccountCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<CreateConfluentAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_confluent_account";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentAccountResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Confluent resource for the account associated with the provided ID.
    pub async fn create_confluent_resource(
        &self,
        account_id: String,
        body: crate::datadogV2::model::ConfluentResourceRequest,
    ) -> Result<
        crate::datadogV2::model::ConfluentResourceResponse,
        Error<CreateConfluentResourceError>,
    > {
        match self
            .create_confluent_resource_with_http_info(account_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a Confluent resource for the account associated with the provided ID.
    pub async fn create_confluent_resource_with_http_info(
        &self,
        account_id: String,
        body: crate::datadogV2::model::ConfluentResourceRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<CreateConfluentResourceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_confluent_resource";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentResourceResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Confluent account with the provided account ID.
    pub async fn delete_confluent_account(
        &self,
        account_id: String,
    ) -> Result<(), Error<DeleteConfluentAccountError>> {
        match self
            .delete_confluent_account_with_http_info(account_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Confluent account with the provided account ID.
    pub async fn delete_confluent_account_with_http_info(
        &self,
        account_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteConfluentAccountError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_confluent_account";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn delete_confluent_resource(
        &self,
        account_id: String,
        resource_id: String,
    ) -> Result<(), Error<DeleteConfluentResourceError>> {
        match self
            .delete_confluent_resource_with_http_info(account_id, resource_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn delete_confluent_resource_with_http_info(
        &self,
        account_id: String,
        resource_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteConfluentResourceError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_confluent_resource";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the Confluent account with the provided account ID.
    pub async fn get_confluent_account(
        &self,
        account_id: String,
    ) -> Result<crate::datadogV2::model::ConfluentAccountResponse, Error<GetConfluentAccountError>>
    {
        match self.get_confluent_account_with_http_info(account_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the Confluent account with the provided account ID.
    pub async fn get_confluent_account_with_http_info(
        &self,
        account_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<GetConfluentAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_confluent_account";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentAccountResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn get_confluent_resource(
        &self,
        account_id: String,
        resource_id: String,
    ) -> Result<crate::datadogV2::model::ConfluentResourceResponse, Error<GetConfluentResourceError>>
    {
        match self
            .get_confluent_resource_with_http_info(account_id, resource_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn get_confluent_resource_with_http_info(
        &self,
        account_id: String,
        resource_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<GetConfluentResourceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_confluent_resource";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentResourceResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// List Confluent accounts.
    pub async fn list_confluent_account(
        &self,
    ) -> Result<crate::datadogV2::model::ConfluentAccountsResponse, Error<ListConfluentAccountError>>
    {
        match self.list_confluent_account_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// List Confluent accounts.
    pub async fn list_confluent_account_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountsResponse>,
        Error<ListConfluentAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_confluent_account";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentAccountsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Confluent resource for the account associated with the provided ID.
    pub async fn list_confluent_resource(
        &self,
        account_id: String,
    ) -> Result<
        crate::datadogV2::model::ConfluentResourcesResponse,
        Error<ListConfluentResourceError>,
    > {
        match self
            .list_confluent_resource_with_http_info(account_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a Confluent resource for the account associated with the provided ID.
    pub async fn list_confluent_resource_with_http_info(
        &self,
        account_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourcesResponse>,
        Error<ListConfluentResourceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_confluent_resource";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentResourcesResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the Confluent account with the provided account ID.
    pub async fn update_confluent_account(
        &self,
        account_id: String,
        body: crate::datadogV2::model::ConfluentAccountUpdateRequest,
    ) -> Result<crate::datadogV2::model::ConfluentAccountResponse, Error<UpdateConfluentAccountError>>
    {
        match self
            .update_confluent_account_with_http_info(account_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update the Confluent account with the provided account ID.
    pub async fn update_confluent_account_with_http_info(
        &self,
        account_id: String,
        body: crate::datadogV2::model::ConfluentAccountUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentAccountResponse>,
        Error<UpdateConfluentAccountError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_confluent_account";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentAccountResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateConfluentAccountError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn update_confluent_resource(
        &self,
        account_id: String,
        resource_id: String,
        body: crate::datadogV2::model::ConfluentResourceRequest,
    ) -> Result<
        crate::datadogV2::model::ConfluentResourceResponse,
        Error<UpdateConfluentResourceError>,
    > {
        match self
            .update_confluent_resource_with_http_info(account_id, resource_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update a Confluent resource with the provided resource id for the account associated with the provided account ID.
    pub async fn update_confluent_resource_with_http_info(
        &self,
        account_id: String,
        resource_id: String,
        body: crate::datadogV2::model::ConfluentResourceRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::ConfluentResourceResponse>,
        Error<UpdateConfluentResourceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_confluent_resource";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integrations/confluent-cloud/accounts/{account_id}/resources/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            account_id = urlencode(account_id),
            resource_id = urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ConfluentResourceResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateConfluentResourceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
