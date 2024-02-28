// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreatePagerDutyIntegrationServiceError is a struct for typed errors of method [`PagerDutyIntegrationAPI::create_pager_duty_integration_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePagerDutyIntegrationServiceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePagerDutyIntegrationServiceError is a struct for typed errors of method [`PagerDutyIntegrationAPI::delete_pager_duty_integration_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePagerDutyIntegrationServiceError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPagerDutyIntegrationServiceError is a struct for typed errors of method [`PagerDutyIntegrationAPI::get_pager_duty_integration_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPagerDutyIntegrationServiceError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePagerDutyIntegrationServiceError is a struct for typed errors of method [`PagerDutyIntegrationAPI::update_pager_duty_integration_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePagerDutyIntegrationServiceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct PagerDutyIntegrationAPI {
    config: configuration::Configuration,
}

impl Default for PagerDutyIntegrationAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl PagerDutyIntegrationAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a new service object in the PagerDuty integration.
    pub async fn create_pager_duty_integration_service(
        &self,
        body: crate::datadogV1::model::PagerDutyService,
    ) -> Result<
        crate::datadogV1::model::PagerDutyServiceName,
        Error<CreatePagerDutyIntegrationServiceError>,
    > {
        match self
            .create_pager_duty_integration_service_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Create a new service object in the PagerDuty integration.
    pub async fn create_pager_duty_integration_service_with_http_info(
        &self,
        body: crate::datadogV1::model::PagerDutyService,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::PagerDutyServiceName>,
        Error<CreatePagerDutyIntegrationServiceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_pager_duty_integration_service";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services",
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
            match serde_json::from_str::<crate::datadogV1::model::PagerDutyServiceName>(
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
            let local_entity: Option<CreatePagerDutyIntegrationServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a single service object in the Datadog-PagerDuty integration.
    pub async fn delete_pager_duty_integration_service(
        &self,
        service_name: String,
    ) -> Result<(), Error<DeletePagerDutyIntegrationServiceError>> {
        match self
            .delete_pager_duty_integration_service_with_http_info(service_name)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a single service object in the Datadog-PagerDuty integration.
    pub async fn delete_pager_duty_integration_service_with_http_info(
        &self,
        service_name: String,
    ) -> Result<ResponseContent<()>, Error<DeletePagerDutyIntegrationServiceError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_pager_duty_integration_service";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.get_operation_host(operation_id),
            service_name = urlencode(service_name)
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
            let local_entity: Option<DeletePagerDutyIntegrationServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get service name in the Datadog-PagerDuty integration.
    pub async fn get_pager_duty_integration_service(
        &self,
        service_name: String,
    ) -> Result<
        crate::datadogV1::model::PagerDutyServiceName,
        Error<GetPagerDutyIntegrationServiceError>,
    > {
        match self
            .get_pager_duty_integration_service_with_http_info(service_name)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get service name in the Datadog-PagerDuty integration.
    pub async fn get_pager_duty_integration_service_with_http_info(
        &self,
        service_name: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::PagerDutyServiceName>,
        Error<GetPagerDutyIntegrationServiceError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_pager_duty_integration_service";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.get_operation_host(operation_id),
            service_name = urlencode(service_name)
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
            match serde_json::from_str::<crate::datadogV1::model::PagerDutyServiceName>(
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
            let local_entity: Option<GetPagerDutyIntegrationServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a single service object in the Datadog-PagerDuty integration.
    pub async fn update_pager_duty_integration_service(
        &self,
        service_name: String,
        body: crate::datadogV1::model::PagerDutyServiceKey,
    ) -> Result<(), Error<UpdatePagerDutyIntegrationServiceError>> {
        match self
            .update_pager_duty_integration_service_with_http_info(service_name, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Update a single service object in the Datadog-PagerDuty integration.
    pub async fn update_pager_duty_integration_service_with_http_info(
        &self,
        service_name: String,
        body: crate::datadogV1::model::PagerDutyServiceKey,
    ) -> Result<ResponseContent<()>, Error<UpdatePagerDutyIntegrationServiceError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.update_pager_duty_integration_service";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.get_operation_host(operation_id),
            service_name = urlencode(service_name)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<UpdatePagerDutyIntegrationServiceError> =
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
