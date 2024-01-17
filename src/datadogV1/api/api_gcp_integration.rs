// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateGCPIntegrationParams is a struct for passing parameters to the method [`CreateGCPIntegration`]
#[derive(Clone, Debug)]
pub struct CreateGCPIntegrationParams {
    /// Create a Datadog-GCP integration.
    pub body: crate::datadogV1::model::GCPAccount,
}

/// DeleteGCPIntegrationParams is a struct for passing parameters to the method [`DeleteGCPIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteGCPIntegrationParams {
    /// Delete a given Datadog-GCP integration.
    pub body: crate::datadogV1::model::GCPAccount,
}

/// UpdateGCPIntegrationParams is a struct for passing parameters to the method [`UpdateGCPIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateGCPIntegrationParams {
    /// Update a Datadog-GCP integration.
    pub body: crate::datadogV1::model::GCPAccount,
}

/// CreateGCPIntegrationError is a struct for typed errors of method [`CreateGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGCPIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteGCPIntegrationError is a struct for typed errors of method [`DeleteGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGCPIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListGCPIntegrationError is a struct for typed errors of method [`ListGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGCPIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateGCPIntegrationError is a struct for typed errors of method [`UpdateGCPIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGCPIntegrationError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct GcpIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for GcpIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl GcpIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Create a Datadog-GCP integration.
    pub async fn create_gcp_integration(
        &self,
        params: CreateGCPIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateGCPIntegrationError>,
    > {
        match self.create_gcp_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Create a Datadog-GCP integration.
    pub async fn create_gcp_integration_with_http_info(
        &self,
        params: CreateGCPIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<CreateGCPIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/gcp", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateGCPIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Delete a given Datadog-GCP integration.
    pub async fn delete_gcp_integration(
        &self,
        params: DeleteGCPIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteGCPIntegrationError>,
    > {
        match self.delete_gcp_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Delete a given Datadog-GCP integration.
    pub async fn delete_gcp_integration_with_http_info(
        &self,
        params: DeleteGCPIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<DeleteGCPIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/gcp", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<DeleteGCPIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. List all Datadog-GCP integrations configured in your Datadog account.
    pub async fn list_gcp_integration(
        &self,
    ) -> Result<Option<Vec<crate::datadogV1::model::GCPAccount>>, Error<ListGCPIntegrationError>>
    {
        match self.list_gcp_integration_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. List all Datadog-GCP integrations configured in your Datadog account.
    pub async fn list_gcp_integration_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::GCPAccount>>,
        Error<ListGCPIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/gcp", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
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
            let local_entity: Option<Vec<crate::datadogV1::model::GCPAccount>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListGCPIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Update a Datadog-GCP integrations host_filters and/or auto-mute.
    /// Requires a `project_id` and `client_email`, however these fields cannot be updated.
    /// If you need to update these fields, delete and use the create (`POST`) endpoint.
    /// The unspecified fields will keep their original values.
    pub async fn update_gcp_integration(
        &self,
        params: UpdateGCPIntegrationParams,
    ) -> Result<
        Option<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateGCPIntegrationError>,
    > {
        match self.update_gcp_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// This endpoint is deprecated – use the V2 endpoints instead. Update a Datadog-GCP integrations host_filters and/or auto-mute.
    /// Requires a `project_id` and `client_email`, however these fields cannot be updated.
    /// If you need to update these fields, delete and use the create (`POST`) endpoint.
    /// The unspecified fields will keep their original values.
    pub async fn update_gcp_integration_with_http_info(
        &self,
        params: UpdateGCPIntegrationParams,
    ) -> Result<
        ResponseContent<std::collections::HashMap<String, serde_json::Value>>,
        Error<UpdateGCPIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/integration/gcp", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<std::collections::HashMap<String, serde_json::Value>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateGCPIntegrationError> =
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
