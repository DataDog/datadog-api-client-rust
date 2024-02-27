// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateDORADeploymentError is a struct for typed errors of method [`DORAMetricsAPI::create_dora_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDORADeploymentError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateDORAIncidentError is a struct for typed errors of method [`DORAMetricsAPI::create_dora_incident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDORAIncidentError {
    Status400(Option<crate::datadogV2::model::JSONAPIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DORAMetricsAPI {
    config: configuration::Configuration,
}

impl Default for DORAMetricsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DORAMetricsAPI {
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
        body: crate::datadogV2::model::DORADeploymentRequest,
    ) -> Result<crate::datadogV2::model::DORADeploymentResponse, Error<CreateDORADeploymentError>>
    {
        match self.create_dora_deployment_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
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
        body: crate::datadogV2::model::DORADeploymentRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DORADeploymentResponse>,
        Error<CreateDORADeploymentError>,
    > {
        let operation_id = "v2.create_dora_deployment".to_string();
        if self.config.is_unstable_operation_enabled(&operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_dora_deployment' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_configuration = &self.config;

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
            match serde_json::from_str::<crate::datadogV2::model::DORADeploymentResponse>(
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
        body: crate::datadogV2::model::DORAIncidentRequest,
    ) -> Result<crate::datadogV2::model::DORAIncidentResponse, Error<CreateDORAIncidentError>> {
        match self.create_dora_incident_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
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
        body: crate::datadogV2::model::DORAIncidentRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::DORAIncidentResponse>,
        Error<CreateDORAIncidentError>,
    > {
        let operation_id = "v2.create_dora_incident".to_string();
        if self.config.is_unstable_operation_enabled(&operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_dora_incident' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_configuration = &self.config;

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
            match serde_json::from_str::<crate::datadogV2::model::DORAIncidentResponse>(
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
