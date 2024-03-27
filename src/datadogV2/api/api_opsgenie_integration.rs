// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateOpsgenieServiceError is a struct for typed errors of method [`OpsgenieIntegrationAPI::create_opsgenie_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteOpsgenieServiceError is a struct for typed errors of method [`OpsgenieIntegrationAPI::delete_opsgenie_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOpsgenieServiceError is a struct for typed errors of method [`OpsgenieIntegrationAPI::get_opsgenie_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListOpsgenieServicesError is a struct for typed errors of method [`OpsgenieIntegrationAPI::list_opsgenie_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOpsgenieServicesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOpsgenieServiceError is a struct for typed errors of method [`OpsgenieIntegrationAPI::update_opsgenie_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct OpsgenieIntegrationAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for OpsgenieIntegrationAPI {
    fn default() -> Self {
        Self::with_config(configuration::Configuration::default())
    }
}

impl OpsgenieIntegrationAPI {
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

    /// Create a new service object in the Opsgenie integration.
    pub async fn create_opsgenie_service(
        &self,
        body: crate::datadogV2::model::OpsgenieServiceCreateRequest,
    ) -> Result<crate::datadogV2::model::OpsgenieServiceResponse, Error<CreateOpsgenieServiceError>>
    {
        match self.create_opsgenie_service_with_http_info(body).await {
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

    /// Create a new service object in the Opsgenie integration.
    pub async fn create_opsgenie_service_with_http_info(
        &self,
        body: crate::datadogV2::model::OpsgenieServiceCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>,
        Error<CreateOpsgenieServiceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_opsgenie_service";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services",
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
            match serde_json::from_str::<crate::datadogV2::model::OpsgenieServiceResponse>(
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
            let local_entity: Option<CreateOpsgenieServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a single service object in the Datadog Opsgenie integration.
    pub async fn delete_opsgenie_service(
        &self,
        integration_service_id: String,
    ) -> Result<(), Error<DeleteOpsgenieServiceError>> {
        match self
            .delete_opsgenie_service_with_http_info(integration_service_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a single service object in the Datadog Opsgenie integration.
    pub async fn delete_opsgenie_service_with_http_info(
        &self,
        integration_service_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteOpsgenieServiceError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_opsgenie_service";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.get_operation_host(operation_id),
            integration_service_id = urlencode(integration_service_id)
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
            let local_entity: Option<DeleteOpsgenieServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a single service from the Datadog Opsgenie integration.
    pub async fn get_opsgenie_service(
        &self,
        integration_service_id: String,
    ) -> Result<crate::datadogV2::model::OpsgenieServiceResponse, Error<GetOpsgenieServiceError>>
    {
        match self
            .get_opsgenie_service_with_http_info(integration_service_id)
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

    /// Get a single service from the Datadog Opsgenie integration.
    pub async fn get_opsgenie_service_with_http_info(
        &self,
        integration_service_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>,
        Error<GetOpsgenieServiceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_opsgenie_service";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.get_operation_host(operation_id),
            integration_service_id = urlencode(integration_service_id)
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
            match serde_json::from_str::<crate::datadogV2::model::OpsgenieServiceResponse>(
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
            let local_entity: Option<GetOpsgenieServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of all services from the Datadog Opsgenie integration.
    pub async fn list_opsgenie_services(
        &self,
    ) -> Result<crate::datadogV2::model::OpsgenieServicesResponse, Error<ListOpsgenieServicesError>>
    {
        match self.list_opsgenie_services_with_http_info().await {
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

    /// Get a list of all services from the Datadog Opsgenie integration.
    pub async fn list_opsgenie_services_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OpsgenieServicesResponse>,
        Error<ListOpsgenieServicesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_opsgenie_services";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services",
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
            match serde_json::from_str::<crate::datadogV2::model::OpsgenieServicesResponse>(
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
            let local_entity: Option<ListOpsgenieServicesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a single service object in the Datadog Opsgenie integration.
    pub async fn update_opsgenie_service(
        &self,
        integration_service_id: String,
        body: crate::datadogV2::model::OpsgenieServiceUpdateRequest,
    ) -> Result<crate::datadogV2::model::OpsgenieServiceResponse, Error<UpdateOpsgenieServiceError>>
    {
        match self
            .update_opsgenie_service_with_http_info(integration_service_id, body)
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

    /// Update a single service object in the Datadog Opsgenie integration.
    pub async fn update_opsgenie_service_with_http_info(
        &self,
        integration_service_id: String,
        body: crate::datadogV2::model::OpsgenieServiceUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>,
        Error<UpdateOpsgenieServiceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_opsgenie_service";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.get_operation_host(operation_id),
            integration_service_id = urlencode(integration_service_id)
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
            match serde_json::from_str::<crate::datadogV2::model::OpsgenieServiceResponse>(
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
            let local_entity: Option<UpdateOpsgenieServiceError> =
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
