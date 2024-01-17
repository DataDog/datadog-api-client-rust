// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateIncidentParams is a struct for passing parameters to the method [`CreateIncident`]
#[derive(Clone, Debug)]
pub struct CreateIncidentParams {
    /// Incident payload.
    pub body: crate::datadogV2::model::IncidentCreateRequest,
}

/// CreateIncidentIntegrationParams is a struct for passing parameters to the method [`CreateIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct CreateIncidentIntegrationParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Incident integration metadata payload.
    pub body: crate::datadogV2::model::IncidentIntegrationMetadataCreateRequest,
}

/// CreateIncidentTodoParams is a struct for passing parameters to the method [`CreateIncidentTodo`]
#[derive(Clone, Debug)]
pub struct CreateIncidentTodoParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Incident todo payload.
    pub body: crate::datadogV2::model::IncidentTodoCreateRequest,
}

/// DeleteIncidentParams is a struct for passing parameters to the method [`DeleteIncident`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentParams {
    /// The UUID of the incident.
    pub incident_id: String,
}

/// DeleteIncidentIntegrationParams is a struct for passing parameters to the method [`DeleteIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentIntegrationParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident integration metadata.
    pub integration_metadata_id: String,
}

/// DeleteIncidentTodoParams is a struct for passing parameters to the method [`DeleteIncidentTodo`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentTodoParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident todo.
    pub todo_id: String,
}

/// GetIncidentParams is a struct for passing parameters to the method [`GetIncident`]
#[derive(Clone, Debug)]
pub struct GetIncidentParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
}

/// GetIncidentIntegrationParams is a struct for passing parameters to the method [`GetIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct GetIncidentIntegrationParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident integration metadata.
    pub integration_metadata_id: String,
}

/// GetIncidentTodoParams is a struct for passing parameters to the method [`GetIncidentTodo`]
#[derive(Clone, Debug)]
pub struct GetIncidentTodoParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident todo.
    pub todo_id: String,
}

/// ListIncidentAttachmentsParams is a struct for passing parameters to the method [`ListIncidentAttachments`]
#[derive(Clone, Debug)]
pub struct ListIncidentAttachmentsParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Specifies which types of related objects are included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>>,
    /// Specifies which types of attachments are included in the response.
    pub filter_attachment_type:
        Option<Vec<crate::datadogV2::model::IncidentAttachmentAttachmentType>>,
}

/// ListIncidentIntegrationsParams is a struct for passing parameters to the method [`ListIncidentIntegrations`]
#[derive(Clone, Debug)]
pub struct ListIncidentIntegrationsParams {
    /// The UUID of the incident.
    pub incident_id: String,
}

/// ListIncidentTodosParams is a struct for passing parameters to the method [`ListIncidentTodos`]
#[derive(Clone, Debug)]
pub struct ListIncidentTodosParams {
    /// The UUID of the incident.
    pub incident_id: String,
}

/// ListIncidentsParams is a struct for passing parameters to the method [`ListIncidents`]
#[derive(Clone, Debug)]
pub struct ListIncidentsParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

/// SearchIncidentsParams is a struct for passing parameters to the method [`SearchIncidents`]
#[derive(Clone, Debug)]
pub struct SearchIncidentsParams {
    /// Specifies which incidents should be returned. The query can contain any number of incident facets
    /// joined by `ANDs`, along with multiple values for each of those facets joined by `OR`s. For
    /// example: `state:active AND severity:(SEV-2 OR SEV-1)`.
    pub query: String,
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
    /// Specifies the order of returned incidents.
    pub sort: Option<crate::datadogV2::model::IncidentSearchSortOrder>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

/// UpdateIncidentParams is a struct for passing parameters to the method [`UpdateIncident`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Incident Payload.
    pub body: crate::datadogV2::model::IncidentUpdateRequest,
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
}

/// UpdateIncidentAttachmentsParams is a struct for passing parameters to the method [`UpdateIncidentAttachments`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentAttachmentsParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// Incident Attachment Payload.
    pub body: crate::datadogV2::model::IncidentAttachmentUpdateRequest,
    /// Specifies which types of related objects are included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>>,
}

/// UpdateIncidentIntegrationParams is a struct for passing parameters to the method [`UpdateIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentIntegrationParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident integration metadata.
    pub integration_metadata_id: String,
    /// Incident integration metadata payload.
    pub body: crate::datadogV2::model::IncidentIntegrationMetadataPatchRequest,
}

/// UpdateIncidentTodoParams is a struct for passing parameters to the method [`UpdateIncidentTodo`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentTodoParams {
    /// The UUID of the incident.
    pub incident_id: String,
    /// The UUID of the incident todo.
    pub todo_id: String,
    /// Incident todo payload.
    pub body: crate::datadogV2::model::IncidentTodoPatchRequest,
}

/// CreateIncidentError is a struct for typed errors of method [`CreateIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateIncidentIntegrationError is a struct for typed errors of method [`CreateIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentIntegrationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateIncidentTodoError is a struct for typed errors of method [`CreateIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentTodoError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentError is a struct for typed errors of method [`DeleteIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentIntegrationError is a struct for typed errors of method [`DeleteIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentIntegrationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentTodoError is a struct for typed errors of method [`DeleteIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentTodoError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentError is a struct for typed errors of method [`GetIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentIntegrationError is a struct for typed errors of method [`GetIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentIntegrationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentTodoError is a struct for typed errors of method [`GetIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentTodoError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentAttachmentsError is a struct for typed errors of method [`ListIncidentAttachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentAttachmentsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentIntegrationsError is a struct for typed errors of method [`ListIncidentIntegrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentIntegrationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentTodosError is a struct for typed errors of method [`ListIncidentTodos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentTodosError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentsError is a struct for typed errors of method [`ListIncidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchIncidentsError is a struct for typed errors of method [`SearchIncidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchIncidentsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentError is a struct for typed errors of method [`UpdateIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentAttachmentsError is a struct for typed errors of method [`UpdateIncidentAttachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentAttachmentsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentIntegrationError is a struct for typed errors of method [`UpdateIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentIntegrationError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentTodoError is a struct for typed errors of method [`UpdateIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentTodoError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct IncidentsAPI {
    config: configuration::Configuration,
}

impl Default for IncidentsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl IncidentsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create an incident.
    pub async fn create_incident(
        &self,
        params: CreateIncidentParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentResponse>, Error<CreateIncidentError>> {
        match self.create_incident_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an incident.
    pub async fn create_incident_with_http_info(
        &self,
        params: CreateIncidentParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentResponse>,
        Error<CreateIncidentError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/incidents", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::IncidentResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateIncidentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an incident integration metadata.
    pub async fn create_incident_integration(
        &self,
        params: CreateIncidentIntegrationParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<CreateIncidentIntegrationError>,
    > {
        match self
            .create_incident_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an incident integration metadata.
    pub async fn create_incident_integration_with_http_info(
        &self,
        params: CreateIncidentIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<CreateIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateIncidentIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an incident todo.
    pub async fn create_incident_todo(
        &self,
        params: CreateIncidentTodoParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTodoResponse>, Error<CreateIncidentTodoError>>
    {
        match self.create_incident_todo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an incident todo.
    pub async fn create_incident_todo_with_http_info(
        &self,
        params: CreateIncidentTodoParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<CreateIncidentTodoError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentTodoResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateIncidentTodoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes an existing incident from the users organization.
    pub async fn delete_incident(
        &self,
        params: DeleteIncidentParams,
    ) -> Result<Option<()>, Error<DeleteIncidentError>> {
        match self.delete_incident_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing incident from the users organization.
    pub async fn delete_incident_with_http_info(
        &self,
        params: DeleteIncidentParams,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
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
            let local_entity: Option<DeleteIncidentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an incident integration metadata.
    pub async fn delete_incident_integration(
        &self,
        params: DeleteIncidentIntegrationParams,
    ) -> Result<Option<()>, Error<DeleteIncidentIntegrationError>> {
        match self
            .delete_incident_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an incident integration metadata.
    pub async fn delete_incident_integration_with_http_info(
        &self,
        params: DeleteIncidentIntegrationParams,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentIntegrationError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let integration_metadata_id = params.integration_metadata_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}", 
            local_configuration.base_path, incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
            let local_entity: Option<DeleteIncidentIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete an incident todo.
    pub async fn delete_incident_todo(
        &self,
        params: DeleteIncidentTodoParams,
    ) -> Result<Option<()>, Error<DeleteIncidentTodoError>> {
        match self.delete_incident_todo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete an incident todo.
    pub async fn delete_incident_todo_with_http_info(
        &self,
        params: DeleteIncidentTodoParams,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentTodoError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let todo_id = params.todo_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
            let local_entity: Option<DeleteIncidentTodoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the details of an incident by `incident_id`.
    pub async fn get_incident(
        &self,
        params: GetIncidentParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentResponse>, Error<GetIncidentError>> {
        match self.get_incident_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the details of an incident by `incident_id`.
    pub async fn get_incident_with_http_info(
        &self,
        params: GetIncidentParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::IncidentResponse>, Error<GetIncidentError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get incident integration metadata details.
    pub async fn get_incident_integration(
        &self,
        params: GetIncidentIntegrationParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<GetIncidentIntegrationError>,
    > {
        match self.get_incident_integration_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get incident integration metadata details.
    pub async fn get_incident_integration_with_http_info(
        &self,
        params: GetIncidentIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<GetIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let integration_metadata_id = params.integration_metadata_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}", 
            local_configuration.base_path, incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get incident todo details.
    pub async fn get_incident_todo(
        &self,
        params: GetIncidentTodoParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTodoResponse>, Error<GetIncidentTodoError>>
    {
        match self.get_incident_todo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get incident todo details.
    pub async fn get_incident_todo_with_http_info(
        &self,
        params: GetIncidentTodoParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<GetIncidentTodoError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let todo_id = params.todo_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentTodoResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentTodoError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all attachments for a given incident.
    pub async fn list_incident_attachments(
        &self,
        params: ListIncidentAttachmentsParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentAttachmentsResponse>,
        Error<ListIncidentAttachmentsError>,
    > {
        match self.list_incident_attachments_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all attachments for a given incident.
    pub async fn list_incident_attachments_with_http_info(
        &self,
        params: ListIncidentAttachmentsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentAttachmentsResponse>,
        Error<ListIncidentAttachmentsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let include = params.include;
        let filter_attachment_type = params.filter_attachment_type;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/attachments",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local) = filter_attachment_type {
            local_req_builder = local_req_builder.query(&[(
                "filter[attachment_type]",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentAttachmentsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentAttachmentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all integration metadata for an incident.
    pub async fn list_incident_integrations(
        &self,
        params: ListIncidentIntegrationsParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentIntegrationMetadataListResponse>,
        Error<ListIncidentIntegrationsError>,
    > {
        match self.list_incident_integrations_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all integration metadata for an incident.
    pub async fn list_incident_integrations_with_http_info(
        &self,
        params: ListIncidentIntegrationsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataListResponse>,
        Error<ListIncidentIntegrationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
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
            let local_entity: Option<
                crate::datadogV2::model::IncidentIntegrationMetadataListResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentIntegrationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all todos for an incident.
    pub async fn list_incident_todos(
        &self,
        params: ListIncidentTodosParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentTodoListResponse>,
        Error<ListIncidentTodosError>,
    > {
        match self.list_incident_todos_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all todos for an incident.
    pub async fn list_incident_todos_with_http_info(
        &self,
        params: ListIncidentTodosParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoListResponse>,
        Error<ListIncidentTodosError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentTodoListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentTodosError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all incidents for the user's organization.
    pub async fn list_incidents(
        &self,
        params: ListIncidentsParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentsResponse>, Error<ListIncidentsError>> {
        match self.list_incidents_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all incidents for the user's organization.
    pub async fn list_incidents_with_http_info(
        &self,
        params: ListIncidentsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentsResponse>,
        Error<ListIncidentsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let include = params.include;
        let page_size = params.page_size;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/incidents", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Search for incidents matching a certain query.
    pub async fn search_incidents(
        &self,
        params: SearchIncidentsParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentSearchResponse>, Error<SearchIncidentsError>>
    {
        match self.search_incidents_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Search for incidents matching a certain query.
    pub async fn search_incidents_with_http_info(
        &self,
        params: SearchIncidentsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentSearchResponse>,
        Error<SearchIncidentsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let query = params.query;
        let include = params.include;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/incidents/search", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref local_str) = include {
            local_req_builder = local_req_builder.query(&[("include", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentSearchResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SearchIncidentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates an incident. Provide only the attributes that should be updated as this request is a partial update.
    pub async fn update_incident(
        &self,
        params: UpdateIncidentParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentResponse>, Error<UpdateIncidentError>> {
        match self.update_incident_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates an incident. Provide only the attributes that should be updated as this request is a partial update.
    pub async fn update_incident_with_http_info(
        &self,
        params: UpdateIncidentParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentResponse>,
        Error<UpdateIncidentError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let body = params.body;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The bulk update endpoint for creating, updating, and deleting attachments for a given incident.
    pub async fn update_incident_attachments(
        &self,
        params: UpdateIncidentAttachmentsParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentAttachmentUpdateResponse>,
        Error<UpdateIncidentAttachmentsError>,
    > {
        match self
            .update_incident_attachments_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// The bulk update endpoint for creating, updating, and deleting attachments for a given incident.
    pub async fn update_incident_attachments_with_http_info(
        &self,
        params: UpdateIncidentAttachmentsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentAttachmentUpdateResponse>,
        Error<UpdateIncidentAttachmentsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let body = params.body;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/attachments",
            local_configuration.base_path,
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentAttachmentUpdateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentAttachmentsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an existing incident integration metadata.
    pub async fn update_incident_integration(
        &self,
        params: UpdateIncidentIntegrationParams,
    ) -> Result<
        Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<UpdateIncidentIntegrationError>,
    > {
        match self
            .update_incident_integration_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an existing incident integration metadata.
    pub async fn update_incident_integration_with_http_info(
        &self,
        params: UpdateIncidentIntegrationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<UpdateIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let integration_metadata_id = params.integration_metadata_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}", 
            local_configuration.base_path, incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentIntegrationMetadataResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentIntegrationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update an incident todo.
    pub async fn update_incident_todo(
        &self,
        params: UpdateIncidentTodoParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTodoResponse>, Error<UpdateIncidentTodoError>>
    {
        match self.update_incident_todo_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update an incident todo.
    pub async fn update_incident_todo_with_http_info(
        &self,
        params: UpdateIncidentTodoParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<UpdateIncidentTodoError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let incident_id = params.incident_id;
        let todo_id = params.todo_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.base_path,
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentTodoResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentTodoError> =
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
