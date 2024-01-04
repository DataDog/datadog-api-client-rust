// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateIncidentServiceParams is a struct for passing parameters to the method [`CreateIncidentService`]
#[derive(Clone, Debug)]
pub struct CreateIncidentServiceParams {
    /// Incident Service Payload.
    pub body: crate::datadogV2::model::IncidentServiceCreateRequest,
}

/// DeleteIncidentServiceParams is a struct for passing parameters to the method [`DeleteIncidentService`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentServiceParams {
    /// The ID of the incident service.
    pub service_id: String,
}

/// GetIncidentServiceParams is a struct for passing parameters to the method [`GetIncidentService`]
#[derive(Clone, Debug)]
pub struct GetIncidentServiceParams {
    /// The ID of the incident service.
    pub service_id: String,
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
}

/// ListIncidentServicesParams is a struct for passing parameters to the method [`ListIncidentServices`]
#[derive(Clone, Debug)]
pub struct ListIncidentServicesParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// A search query that filters services by name.
    pub filter: Option<String>,
}

/// UpdateIncidentServiceParams is a struct for passing parameters to the method [`UpdateIncidentService`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentServiceParams {
    /// The ID of the incident service.
    pub service_id: String,
    /// Incident Service Payload.
    pub body: crate::datadogV2::model::IncidentServiceUpdateRequest,
}

/// CreateIncidentServiceError is a struct for typed errors of method [`CreateIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentServiceError is a struct for typed errors of method [`DeleteIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentServiceError is a struct for typed errors of method [`GetIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentServicesError is a struct for typed errors of method [`ListIncidentServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentServicesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentServiceError is a struct for typed errors of method [`UpdateIncidentService`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentServiceError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct IncidentServicesAPI {
    config: configuration::Configuration,
}

impl Default for IncidentServicesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl IncidentServicesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates a new incident service.
    pub async fn create_incident_service(
        &self,
        params: CreateIncidentServiceParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentServiceResponse>,
        Error<CreateIncidentServiceError>,
    > {
        match self.create_incident_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates a new incident service.
    pub async fn create_incident_service_with_http_info(
        &self,
        params: CreateIncidentServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentServiceResponse>,
        Error<CreateIncidentServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/services", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::IncidentServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateIncidentServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes an existing incident service.
    pub async fn delete_incident_service(
        &self,
        params: DeleteIncidentServiceParams,
    ) -> Result<Option<()>, Error<DeleteIncidentServiceError>> {
        match self.delete_incident_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing incident service.
    pub async fn delete_incident_service_with_http_info(
        &self,
        params: DeleteIncidentServiceParams,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentServiceError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_id = params.service_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/{service_id}",
            local_configuration.base_path,
            service_id = urlencode(service_id)
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
            let local_entity: Option<DeleteIncidentServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get details of an incident service. If the `include[users]` query parameter is provided,
    /// the included attribute will contain the users related to these incident services.
    pub async fn get_incident_service(
        &self,
        params: GetIncidentServiceParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentServiceResponse>,
        Error<GetIncidentServiceError>,
    > {
        match self.get_incident_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get details of an incident service. If the `include[users]` query parameter is provided,
    /// the included attribute will contain the users related to these incident services.
    pub async fn get_incident_service_with_http_info(
        &self,
        params: GetIncidentServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentServiceResponse>,
        Error<GetIncidentServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_id = params.service_id;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/{service_id}",
            local_configuration.base_path,
            service_id = urlencode(service_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = include {
            local_req_builder = local_req_builder.query(&[("include", &local_str.to_string())]);
        };

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
            let local_entity: Option<crate::datadogV2::model::IncidentServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentServiceError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all incident services uploaded for the requesting user's organization. If the `include[users]` query parameter is provided, the included attribute will contain the users related to these incident services.
    pub async fn list_incident_services(
        &self,
        params: ListIncidentServicesParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentServicesResponse>,
        Error<ListIncidentServicesError>,
    > {
        match self.list_incident_services_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all incident services uploaded for the requesting user's organization. If the `include[users]` query parameter is provided, the included attribute will contain the users related to these incident services.
    pub async fn list_incident_services_with_http_info(
        &self,
        params: ListIncidentServicesParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentServicesResponse>,
        Error<ListIncidentServicesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let include = params.include;
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let filter = params.filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/services", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = include {
            local_req_builder = local_req_builder.query(&[("include", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter {
            local_req_builder = local_req_builder.query(&[("filter", &local_str.to_string())]);
        };

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
            let local_entity: Option<crate::datadogV2::model::IncidentServicesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentServicesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates an existing incident service. Only provide the attributes which should be updated as this request is a partial update.
    pub async fn update_incident_service(
        &self,
        params: UpdateIncidentServiceParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentServiceResponse>,
        Error<UpdateIncidentServiceError>,
    > {
        match self.update_incident_service_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates an existing incident service. Only provide the attributes which should be updated as this request is a partial update.
    pub async fn update_incident_service_with_http_info(
        &self,
        params: UpdateIncidentServiceParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentServiceResponse>,
        Error<UpdateIncidentServiceError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let service_id = params.service_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/services/{service_id}",
            local_configuration.base_path,
            service_id = urlencode(service_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::IncidentServiceResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentServiceError> =
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