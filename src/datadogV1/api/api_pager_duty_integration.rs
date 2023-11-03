// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreatePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`CreatePagerDutyIntegrationService`]
#[derive(Clone, Debug, Default)]
pub struct CreatePagerDutyIntegrationServiceParams {
    /// Create a new service object request body.
    pub body: crate::datadogV1::model::PagerDutyService,
}

/// DeletePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`DeletePagerDutyIntegrationService`]
#[derive(Clone, Debug, Default)]
pub struct DeletePagerDutyIntegrationServiceParams {
    /// The service name
    pub service_name: String,
}

/// GetPagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`GetPagerDutyIntegrationService`]
#[derive(Clone, Debug, Default)]
pub struct GetPagerDutyIntegrationServiceParams {
    /// The service name.
    pub service_name: String,
}

/// UpdatePagerDutyIntegrationServiceParams is a struct for passing parameters to the method [`UpdatePagerDutyIntegrationService`]
#[derive(Clone, Debug, Default)]
pub struct UpdatePagerDutyIntegrationServiceParams {
    /// The service name
    pub service_name: String,
    /// Update an existing service object request body.
    pub body: crate::datadogV1::model::PagerDutyServiceKey,
}

/// CreatePagerDutyIntegrationServiceError is a struct for typed errors of method [`CreatePagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePagerDutyIntegrationServiceError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePagerDutyIntegrationServiceError is a struct for typed errors of method [`DeletePagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePagerDutyIntegrationServiceError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPagerDutyIntegrationServiceError is a struct for typed errors of method [`GetPagerDutyIntegrationService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPagerDutyIntegrationServiceError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePagerDutyIntegrationServiceError is a struct for typed errors of method [`UpdatePagerDutyIntegrationService`]
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
        params: CreatePagerDutyIntegrationServiceParams,
    ) -> Result<
        Option<crate::datadogV1::model::PagerDutyServiceName>,
        Error<CreatePagerDutyIntegrationServiceError>,
    > {
        match self
            .create_pager_duty_integration_service_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new service object in the PagerDuty integration.
    pub async fn create_pager_duty_integration_service_with_http_info(
        &self,
        params: CreatePagerDutyIntegrationServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::PagerDutyServiceName>,
        Error<CreatePagerDutyIntegrationServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services",
            local_configuration.base_path
        );
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::PagerDutyServiceName> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeletePagerDutyIntegrationServiceParams,
    ) -> Result<Option<()>, Error<DeletePagerDutyIntegrationServiceError>> {
        match self
            .delete_pager_duty_integration_service_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a single service object in the Datadog-PagerDuty integration.
    pub async fn delete_pager_duty_integration_service_with_http_info(
        &self,
        params: DeletePagerDutyIntegrationServiceParams,
    ) -> Result<ResponseContent<()>, Error<DeletePagerDutyIntegrationServiceError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_name = params.service_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.base_path,
            service_name = urlencode(service_name)
        );
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
        params: GetPagerDutyIntegrationServiceParams,
    ) -> Result<
        Option<crate::datadogV1::model::PagerDutyServiceName>,
        Error<GetPagerDutyIntegrationServiceError>,
    > {
        match self
            .get_pager_duty_integration_service_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get service name in the Datadog-PagerDuty integration.
    pub async fn get_pager_duty_integration_service_with_http_info(
        &self,
        params: GetPagerDutyIntegrationServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::PagerDutyServiceName>,
        Error<GetPagerDutyIntegrationServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_name = params.service_name;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.base_path,
            service_name = urlencode(service_name)
        );
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
            let local_entity: Option<crate::datadogV1::model::PagerDutyServiceName> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdatePagerDutyIntegrationServiceParams,
    ) -> Result<Option<()>, Error<UpdatePagerDutyIntegrationServiceError>> {
        match self
            .update_pager_duty_integration_service_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a single service object in the Datadog-PagerDuty integration.
    pub async fn update_pager_duty_integration_service_with_http_info(
        &self,
        params: UpdatePagerDutyIntegrationServiceParams,
    ) -> Result<ResponseContent<()>, Error<UpdatePagerDutyIntegrationServiceError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_name = params.service_name;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/integration/pagerduty/configuration/services/{service_name}",
            local_configuration.base_path,
            service_name = urlencode(service_name)
        );
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
        local_req_builder = local_req_builder.json(&body);

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
