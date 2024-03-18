// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;

use futures_core::stream::Stream;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetIncidentOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::get_incident`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetIncidentOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
}

impl GetIncidentOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(mut self, value: Vec<crate::datadogV2::model::IncidentRelatedObject>) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListIncidentAttachmentsOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::list_incident_attachments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListIncidentAttachmentsOptionalParams {
    /// Specifies which types of related objects are included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>>,
    /// Specifies which types of attachments are included in the response.
    pub filter_attachment_type:
        Option<Vec<crate::datadogV2::model::IncidentAttachmentAttachmentType>>,
}

impl ListIncidentAttachmentsOptionalParams {
    /// Specifies which types of related objects are included in the response.
    pub fn include(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>,
    ) -> Self {
        self.include = Some(value);
        self
    }
    /// Specifies which types of attachments are included in the response.
    pub fn filter_attachment_type(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentAttachmentAttachmentType>,
    ) -> Self {
        self.filter_attachment_type = Some(value);
        self
    }
}

/// ListIncidentsOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::list_incidents`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListIncidentsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

impl ListIncidentsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(mut self, value: Vec<crate::datadogV2::model::IncidentRelatedObject>) -> Self {
        self.include = Some(value);
        self
    }
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
}

/// SearchIncidentsOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::search_incidents`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchIncidentsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
    /// Specifies the order of returned incidents.
    pub sort: Option<crate::datadogV2::model::IncidentSearchSortOrder>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
}

impl SearchIncidentsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(mut self, value: crate::datadogV2::model::IncidentRelatedObject) -> Self {
        self.include = Some(value);
        self
    }
    /// Specifies the order of returned incidents.
    pub fn sort(mut self, value: crate::datadogV2::model::IncidentSearchSortOrder) -> Self {
        self.sort = Some(value);
        self
    }
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
}

/// UpdateIncidentOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::update_incident`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateIncidentOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentRelatedObject>>,
}

impl UpdateIncidentOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(mut self, value: Vec<crate::datadogV2::model::IncidentRelatedObject>) -> Self {
        self.include = Some(value);
        self
    }
}

/// UpdateIncidentAttachmentsOptionalParams is a struct for passing parameters to the method [`IncidentsAPI::update_incident_attachments`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateIncidentAttachmentsOptionalParams {
    /// Specifies which types of related objects are included in the response.
    pub include: Option<Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>>,
}

impl UpdateIncidentAttachmentsOptionalParams {
    /// Specifies which types of related objects are included in the response.
    pub fn include(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentAttachmentRelatedObject>,
    ) -> Self {
        self.include = Some(value);
        self
    }
}

/// CreateIncidentError is a struct for typed errors of method [`IncidentsAPI::create_incident`]
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

/// CreateIncidentIntegrationError is a struct for typed errors of method [`IncidentsAPI::create_incident_integration`]
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

/// CreateIncidentTodoError is a struct for typed errors of method [`IncidentsAPI::create_incident_todo`]
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

/// DeleteIncidentError is a struct for typed errors of method [`IncidentsAPI::delete_incident`]
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

/// DeleteIncidentIntegrationError is a struct for typed errors of method [`IncidentsAPI::delete_incident_integration`]
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

/// DeleteIncidentTodoError is a struct for typed errors of method [`IncidentsAPI::delete_incident_todo`]
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

/// GetIncidentError is a struct for typed errors of method [`IncidentsAPI::get_incident`]
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

/// GetIncidentIntegrationError is a struct for typed errors of method [`IncidentsAPI::get_incident_integration`]
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

/// GetIncidentTodoError is a struct for typed errors of method [`IncidentsAPI::get_incident_todo`]
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

/// ListIncidentAttachmentsError is a struct for typed errors of method [`IncidentsAPI::list_incident_attachments`]
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

/// ListIncidentIntegrationsError is a struct for typed errors of method [`IncidentsAPI::list_incident_integrations`]
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

/// ListIncidentTodosError is a struct for typed errors of method [`IncidentsAPI::list_incident_todos`]
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

/// ListIncidentsError is a struct for typed errors of method [`IncidentsAPI::list_incidents`]
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

/// SearchIncidentsError is a struct for typed errors of method [`IncidentsAPI::search_incidents`]
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

/// UpdateIncidentError is a struct for typed errors of method [`IncidentsAPI::update_incident`]
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

/// UpdateIncidentAttachmentsError is a struct for typed errors of method [`IncidentsAPI::update_incident_attachments`]
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

/// UpdateIncidentIntegrationError is a struct for typed errors of method [`IncidentsAPI::update_incident_integration`]
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

/// UpdateIncidentTodoError is a struct for typed errors of method [`IncidentsAPI::update_incident_todo`]
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
        body: crate::datadogV2::model::IncidentCreateRequest,
    ) -> Result<crate::datadogV2::model::IncidentResponse, Error<CreateIncidentError>> {
        match self.create_incident_with_http_info(body).await {
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

    /// Create an incident.
    pub async fn create_incident_with_http_info(
        &self,
        body: crate::datadogV2::model::IncidentCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentResponse>,
        Error<CreateIncidentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_incident";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_incident' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents",
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentResponse>(&local_content)
            {
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
        incident_id: String,
        body: crate::datadogV2::model::IncidentIntegrationMetadataCreateRequest,
    ) -> Result<
        crate::datadogV2::model::IncidentIntegrationMetadataResponse,
        Error<CreateIncidentIntegrationError>,
    > {
        match self
            .create_incident_integration_with_http_info(incident_id, body)
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

    /// Create an incident integration metadata.
    pub async fn create_incident_integration_with_http_info(
        &self,
        incident_id: String,
        body: crate::datadogV2::model::IncidentIntegrationMetadataCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<CreateIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_incident_integration";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_incident_integration' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentIntegrationMetadataResponse>(
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
        incident_id: String,
        body: crate::datadogV2::model::IncidentTodoCreateRequest,
    ) -> Result<crate::datadogV2::model::IncidentTodoResponse, Error<CreateIncidentTodoError>> {
        match self
            .create_incident_todo_with_http_info(incident_id, body)
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

    /// Create an incident todo.
    pub async fn create_incident_todo_with_http_info(
        &self,
        incident_id: String,
        body: crate::datadogV2::model::IncidentTodoCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<CreateIncidentTodoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_incident_todo";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_incident_todo' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentTodoResponse>(
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
        incident_id: String,
    ) -> Result<(), Error<DeleteIncidentError>> {
        match self.delete_incident_with_http_info(incident_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing incident from the users organization.
    pub async fn delete_incident_with_http_info(
        &self,
        incident_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_incident";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_incident' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
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
        incident_id: String,
        integration_metadata_id: String,
    ) -> Result<(), Error<DeleteIncidentIntegrationError>> {
        match self
            .delete_incident_integration_with_http_info(incident_id, integration_metadata_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an incident integration metadata.
    pub async fn delete_incident_integration_with_http_info(
        &self,
        incident_id: String,
        integration_metadata_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentIntegrationError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_incident_integration";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_incident_integration' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}",
            local_configuration.get_operation_host(operation_id), incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
        incident_id: String,
        todo_id: String,
    ) -> Result<(), Error<DeleteIncidentTodoError>> {
        match self
            .delete_incident_todo_with_http_info(incident_id, todo_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete an incident todo.
    pub async fn delete_incident_todo_with_http_info(
        &self,
        incident_id: String,
        todo_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentTodoError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_incident_todo";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_incident_todo' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
        incident_id: String,
        params: GetIncidentOptionalParams,
    ) -> Result<crate::datadogV2::model::IncidentResponse, Error<GetIncidentError>> {
        match self.get_incident_with_http_info(incident_id, params).await {
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

    /// Get the details of an incident by `incident_id`.
    pub async fn get_incident_with_http_info(
        &self,
        incident_id: String,
        params: GetIncidentOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::IncidentResponse>, Error<GetIncidentError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_incident";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_incident' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentResponse>(&local_content)
            {
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
        incident_id: String,
        integration_metadata_id: String,
    ) -> Result<
        crate::datadogV2::model::IncidentIntegrationMetadataResponse,
        Error<GetIncidentIntegrationError>,
    > {
        match self
            .get_incident_integration_with_http_info(incident_id, integration_metadata_id)
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

    /// Get incident integration metadata details.
    pub async fn get_incident_integration_with_http_info(
        &self,
        incident_id: String,
        integration_metadata_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<GetIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_incident_integration";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_incident_integration' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}",
            local_configuration.get_operation_host(operation_id), incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentIntegrationMetadataResponse>(
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
        incident_id: String,
        todo_id: String,
    ) -> Result<crate::datadogV2::model::IncidentTodoResponse, Error<GetIncidentTodoError>> {
        match self
            .get_incident_todo_with_http_info(incident_id, todo_id)
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

    /// Get incident todo details.
    pub async fn get_incident_todo_with_http_info(
        &self,
        incident_id: String,
        todo_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<GetIncidentTodoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_incident_todo";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_incident_todo' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentTodoResponse>(
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
        incident_id: String,
        params: ListIncidentAttachmentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::IncidentAttachmentsResponse,
        Error<ListIncidentAttachmentsError>,
    > {
        match self
            .list_incident_attachments_with_http_info(incident_id, params)
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

    /// Get all attachments for a given incident.
    pub async fn list_incident_attachments_with_http_info(
        &self,
        incident_id: String,
        params: ListIncidentAttachmentsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentAttachmentsResponse>,
        Error<ListIncidentAttachmentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_incident_attachments";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_incident_attachments' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let filter_attachment_type = params.filter_attachment_type;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/attachments",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
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
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentAttachmentsResponse>(
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
        incident_id: String,
    ) -> Result<
        crate::datadogV2::model::IncidentIntegrationMetadataListResponse,
        Error<ListIncidentIntegrationsError>,
    > {
        match self
            .list_incident_integrations_with_http_info(incident_id)
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

    /// Get all integration metadata for an incident.
    pub async fn list_incident_integrations_with_http_info(
        &self,
        incident_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataListResponse>,
        Error<ListIncidentIntegrationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_incident_integrations";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_incident_integrations' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
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
            match serde_json::from_str::<
                crate::datadogV2::model::IncidentIntegrationMetadataListResponse,
            >(&local_content)
            {
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
        incident_id: String,
    ) -> Result<crate::datadogV2::model::IncidentTodoListResponse, Error<ListIncidentTodosError>>
    {
        match self.list_incident_todos_with_http_info(incident_id).await {
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

    /// Get all todos for an incident.
    pub async fn list_incident_todos_with_http_info(
        &self,
        incident_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoListResponse>,
        Error<ListIncidentTodosError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_incident_todos";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_incident_todos' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentTodoListResponse>(
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
        params: ListIncidentsOptionalParams,
    ) -> Result<crate::datadogV2::model::IncidentsResponse, Error<ListIncidentsError>> {
        match self.list_incidents_with_http_info(params).await {
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

    pub fn list_incidents_with_pagination(
        &self,
        mut params: ListIncidentsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::IncidentResponseData, Error<ListIncidentsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.list_incidents(params.clone()).await?;

                let r = resp.data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all incidents for the user's organization.
    pub async fn list_incidents_with_http_info(
        &self,
        params: ListIncidentsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentsResponse>,
        Error<ListIncidentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_incidents";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_incidents' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let page_size = params.page_size;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentsResponse>(&local_content)
            {
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
        query: String,
        params: SearchIncidentsOptionalParams,
    ) -> Result<crate::datadogV2::model::IncidentSearchResponse, Error<SearchIncidentsError>> {
        match self.search_incidents_with_http_info(query, params).await {
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

    pub fn search_incidents_with_pagination(
        &self,
        query: String,
        mut params: SearchIncidentsOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::IncidentSearchResponseIncidentsData,
            Error<SearchIncidentsError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            loop {
                let resp = self.search_incidents( query.clone(),params.clone()).await?;
                let Some(attributes) = resp.data.attributes else { break };

                let r = attributes.incidents;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Search for incidents matching a certain query.
    pub async fn search_incidents_with_http_info(
        &self,
        query: String,
        params: SearchIncidentsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentSearchResponse>,
        Error<SearchIncidentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.search_incidents";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.search_incidents' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let sort = params.sort;
        let page_size = params.page_size;
        let page_offset = params.page_offset;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("query", &query.to_string())]);
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentSearchResponse>(
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
        incident_id: String,
        body: crate::datadogV2::model::IncidentUpdateRequest,
        params: UpdateIncidentOptionalParams,
    ) -> Result<crate::datadogV2::model::IncidentResponse, Error<UpdateIncidentError>> {
        match self
            .update_incident_with_http_info(incident_id, body, params)
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

    /// Updates an incident. Provide only the attributes that should be updated as this request is a partial update.
    pub async fn update_incident_with_http_info(
        &self,
        incident_id: String,
        body: crate::datadogV2::model::IncidentUpdateRequest,
        params: UpdateIncidentOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentResponse>,
        Error<UpdateIncidentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_incident";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.update_incident' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentResponse>(&local_content)
            {
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
        incident_id: String,
        body: crate::datadogV2::model::IncidentAttachmentUpdateRequest,
        params: UpdateIncidentAttachmentsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::IncidentAttachmentUpdateResponse,
        Error<UpdateIncidentAttachmentsError>,
    > {
        match self
            .update_incident_attachments_with_http_info(incident_id, body, params)
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

    /// The bulk update endpoint for creating, updating, and deleting attachments for a given incident.
    pub async fn update_incident_attachments_with_http_info(
        &self,
        incident_id: String,
        body: crate::datadogV2::model::IncidentAttachmentUpdateRequest,
        params: UpdateIncidentAttachmentsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentAttachmentUpdateResponse>,
        Error<UpdateIncidentAttachmentsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_incident_attachments";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.update_incident_attachments' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/attachments",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::IncidentAttachmentUpdateResponse>(
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
        incident_id: String,
        integration_metadata_id: String,
        body: crate::datadogV2::model::IncidentIntegrationMetadataPatchRequest,
    ) -> Result<
        crate::datadogV2::model::IncidentIntegrationMetadataResponse,
        Error<UpdateIncidentIntegrationError>,
    > {
        match self
            .update_incident_integration_with_http_info(incident_id, integration_metadata_id, body)
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

    /// Update an existing incident integration metadata.
    pub async fn update_incident_integration_with_http_info(
        &self,
        incident_id: String,
        integration_metadata_id: String,
        body: crate::datadogV2::model::IncidentIntegrationMetadataPatchRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentIntegrationMetadataResponse>,
        Error<UpdateIncidentIntegrationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_incident_integration";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.update_incident_integration' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/integrations/{integration_metadata_id}",
            local_configuration.get_operation_host(operation_id), incident_id=
            urlencode(incident_id)
            , integration_metadata_id=
            urlencode(integration_metadata_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentIntegrationMetadataResponse>(
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
        incident_id: String,
        todo_id: String,
        body: crate::datadogV2::model::IncidentTodoPatchRequest,
    ) -> Result<crate::datadogV2::model::IncidentTodoResponse, Error<UpdateIncidentTodoError>> {
        match self
            .update_incident_todo_with_http_info(incident_id, todo_id, body)
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

    /// Update an incident todo.
    pub async fn update_incident_todo_with_http_info(
        &self,
        incident_id: String,
        todo_id: String,
        body: crate::datadogV2::model::IncidentTodoPatchRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTodoResponse>,
        Error<UpdateIncidentTodoError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_incident_todo";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {operation_id}");
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.update_incident_todo' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/incidents/{incident_id}/relationships/todos/{todo_id}",
            local_configuration.get_operation_host(operation_id),
            incident_id = urlencode(incident_id),
            todo_id = urlencode(todo_id)
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
            match serde_json::from_str::<crate::datadogV2::model::IncidentTodoResponse>(
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
