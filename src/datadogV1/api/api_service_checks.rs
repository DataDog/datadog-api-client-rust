// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// SubmitServiceCheckParams is a struct for passing parameters to the method [`SubmitServiceCheck`]
#[derive(Clone, Debug, Default)]
pub struct SubmitServiceCheckParams {
    /// Service Check request body.
    pub body: Vec<crate::datadogV1::model::ServiceCheck>,
}

/// SubmitServiceCheckError is a struct for typed errors of method [`SubmitServiceCheck`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitServiceCheckError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status408(Option<crate::datadogV1::model::APIErrorResponse>),
    Status413(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceChecksAPI {
    config: configuration::Configuration,
}

impl Default for ServiceChecksAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceChecksAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Submit a list of Service Checks.
    ///
    /// **Notes**:
    /// - A valid API key is required.
    /// - Service checks can be submitted up to 10 minutes in the past.
    pub async fn submit_service_check(
        &self,
        params: SubmitServiceCheckParams,
    ) -> Result<
        Option<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitServiceCheckError>,
    > {
        match self.submit_service_check_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Submit a list of Service Checks.
    ///
    /// **Notes**:
    /// - A valid API key is required.
    /// - Service checks can be submitted up to 10 minutes in the past.
    pub async fn submit_service_check_with_http_info(
        &self,
        params: SubmitServiceCheckParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::IntakePayloadAccepted>,
        Error<SubmitServiceCheckError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/check_run", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::IntakePayloadAccepted> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SubmitServiceCheckError> =
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
