// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDORADeploymentParams is a struct for passing parameters to the method [`CreateDORADeployment`]
#[derive(Clone, Debug)]
pub struct CreateDORADeploymentParams {
    pub body: crate::datadogV2::model::DORADeploymentRequest,
}

/// CreateDORAIncidentParams is a struct for passing parameters to the method [`CreateDORAIncident`]
#[derive(Clone, Debug)]
pub struct CreateDORAIncidentParams {
    pub body: crate::datadogV2::model::DORAIncidentRequest,
}

/// CreateDORADeploymentError is a struct for typed errors of method [`CreateDORADeployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDORADeploymentError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateDORAIncidentError is a struct for typed errors of method [`CreateDORAIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDORAIncidentError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DoraMetricsAPI {
    config: configuration::Configuration,
}

impl Default for DoraMetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DoraMetricsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Use this API endpoint to provide data about deployments for DORA metrics.
    ///
    /// This is necessary for:
    /// - Deployment Frequency
    /// - Change Lead Time
    /// - Change Failure Rate
    pub async fn create_dora_deployment(
        &self,
        params: CreateDORADeploymentParams,
    ) -> Result<
        Option<crate::datadogV2::model::DORADeploymentResponse>,
        Error<CreateDORADeploymentError>,
    > {
        match self.create_dora_deployment_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Use this API endpoint to provide data about deployments for DORA metrics.
    ///
    /// This is necessary for:
    /// - Deployment Frequency
    /// - Change Lead Time
    /// - Change Failure Rate
    pub async fn create_dora_deployment_with_http_info(
        &self,
        params: CreateDORADeploymentParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DORADeploymentResponse>,
        Error<CreateDORADeploymentError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/dora/deployment", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::DORADeploymentResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDORADeploymentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Use this API endpoint to provide data about incidents for DORA metrics.
    ///
    /// This is necessary for:
    /// - Change Failure Rate
    /// - Time to Restore
    pub async fn create_dora_incident(
        &self,
        params: CreateDORAIncidentParams,
    ) -> Result<Option<crate::datadogV2::model::DORAIncidentResponse>, Error<CreateDORAIncidentError>>
    {
        match self.create_dora_incident_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Use this API endpoint to provide data about incidents for DORA metrics.
    ///
    /// This is necessary for:
    /// - Change Failure Rate
    /// - Time to Restore
    pub async fn create_dora_incident_with_http_info(
        &self,
        params: CreateDORAIncidentParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DORAIncidentResponse>,
        Error<CreateDORAIncidentError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/dora/incident", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::DORAIncidentResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDORAIncidentError> =
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