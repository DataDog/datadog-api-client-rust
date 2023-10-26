// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateOpsgenieServiceParams is a struct for passing parameters to the method [`CreateOpsgenieService`]
#[derive(Clone, Debug, Default)]
pub struct CreateOpsgenieServiceParams {
    /// Opsgenie service payload
    pub body: crate::datadogV2::model::OpsgenieServiceCreateRequest,
}

/// DeleteOpsgenieServiceParams is a struct for passing parameters to the method [`DeleteOpsgenieService`]
#[derive(Clone, Debug, Default)]
pub struct DeleteOpsgenieServiceParams {
    /// The UUID of the service.
    pub integration_service_id: String,
}

/// GetOpsgenieServiceParams is a struct for passing parameters to the method [`GetOpsgenieService`]
#[derive(Clone, Debug, Default)]
pub struct GetOpsgenieServiceParams {
    /// The UUID of the service.
    pub integration_service_id: String,
}

/// UpdateOpsgenieServiceParams is a struct for passing parameters to the method [`UpdateOpsgenieService`]
#[derive(Clone, Debug, Default)]
pub struct UpdateOpsgenieServiceParams {
    /// The UUID of the service.
    pub integration_service_id: String,
    /// Opsgenie service payload.
    pub body: crate::datadogV2::model::OpsgenieServiceUpdateRequest,
}

/// CreateOpsgenieServiceError is a struct for typed errors of method [`CreateOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteOpsgenieServiceError is a struct for typed errors of method [`DeleteOpsgenieService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOpsgenieServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetOpsgenieServiceError is a struct for typed errors of method [`GetOpsgenieService`]
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

/// ListOpsgenieServicesError is a struct for typed errors of method [`ListOpsgenieServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOpsgenieServicesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateOpsgenieServiceError is a struct for typed errors of method [`UpdateOpsgenieService`]
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
}

impl Default for OpsgenieIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl OpsgenieIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a new service object in the Opsgenie integration.
    pub async fn create_opsgenie_service(
        &self,
        params: CreateOpsgenieServiceParams,
    ) -> Result<Option<crate::datadogV2::model::OpsgenieServiceResponse>, Error<CreateOpsgenieServiceError>> {
        match self.create_opsgenie_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new service object in the Opsgenie integration.
    pub async fn create_opsgenie_service_with_http_info(
        &self,
        params: CreateOpsgenieServiceParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>, Error<CreateOpsgenieServiceError>>
    {
        let local_configuration = &self.config;

        // unbox the parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/integration/opsgenie/services", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::OpsgenieServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateOpsgenieServiceError> = serde_json::from_str(&local_content).ok();
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
        params: DeleteOpsgenieServiceParams,
    ) -> Result<Option<()>, Error<DeleteOpsgenieServiceError>> {
        match self.delete_opsgenie_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a single service object in the Datadog Opsgenie integration.
    pub async fn delete_opsgenie_service_with_http_info(
        &self,
        params: DeleteOpsgenieServiceParams,
    ) -> Result<ResponseContent<()>, Error<DeleteOpsgenieServiceError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let integration_service_id = params.integration_service_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.base_path,
            integration_service_id = urlencode(integration_service_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<DeleteOpsgenieServiceError> = serde_json::from_str(&local_content).ok();
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
        params: GetOpsgenieServiceParams,
    ) -> Result<Option<crate::datadogV2::model::OpsgenieServiceResponse>, Error<GetOpsgenieServiceError>> {
        match self.get_opsgenie_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a single service from the Datadog Opsgenie integration.
    pub async fn get_opsgenie_service_with_http_info(
        &self,
        params: GetOpsgenieServiceParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>, Error<GetOpsgenieServiceError>> {
        let local_configuration = &self.config;

        // unbox the parameters
        let integration_service_id = params.integration_service_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.base_path,
            integration_service_id = urlencode(integration_service_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::OpsgenieServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetOpsgenieServiceError> = serde_json::from_str(&local_content).ok();
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
    ) -> Result<Option<crate::datadogV2::model::OpsgenieServicesResponse>, Error<ListOpsgenieServicesError>> {
        match self.list_opsgenie_services_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a list of all services from the Datadog Opsgenie integration.
    pub async fn list_opsgenie_services_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV2::model::OpsgenieServicesResponse>, Error<ListOpsgenieServicesError>>
    {
        let local_configuration = &self.config;

        // unbox the parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/integration/opsgenie/services", local_configuration.base_path);
        let mut local_req_builder = local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::OpsgenieServicesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListOpsgenieServicesError> = serde_json::from_str(&local_content).ok();
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
        params: UpdateOpsgenieServiceParams,
    ) -> Result<Option<crate::datadogV2::model::OpsgenieServiceResponse>, Error<UpdateOpsgenieServiceError>> {
        match self.update_opsgenie_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a single service object in the Datadog Opsgenie integration.
    pub async fn update_opsgenie_service_with_http_info(
        &self,
        params: UpdateOpsgenieServiceParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::OpsgenieServiceResponse>, Error<UpdateOpsgenieServiceError>>
    {
        let local_configuration = &self.config;

        // unbox the parameters
        let integration_service_id = params.integration_service_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/integration/opsgenie/services/{integration_service_id}",
            local_configuration.base_path,
            integration_service_id = urlencode(integration_service_id)
        );
        let mut local_req_builder = local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder = local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        // body params
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::OpsgenieServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateOpsgenieServiceError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
