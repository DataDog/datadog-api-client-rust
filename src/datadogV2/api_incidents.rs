// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateIncidentParams is a struct for passing parameters to the method [`CreateIncident`]
#[derive(Clone, Debug)]
pub struct CreateIncidentParams {
    /* Incident payload. */
    pub body: IncidentCreateRequest,
}

// CreateIncidentIntegrationParams is a struct for passing parameters to the method [`CreateIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct CreateIncidentIntegrationParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Incident integration metadata payload. */
    pub body: IncidentIntegrationMetadataCreateRequest,
}

// CreateIncidentTodoParams is a struct for passing parameters to the method [`CreateIncidentTodo`]
#[derive(Clone, Debug)]
pub struct CreateIncidentTodoParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Incident todo payload. */
    pub body: IncidentTodoCreateRequest,
}

// DeleteIncidentParams is a struct for passing parameters to the method [`DeleteIncident`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentParams {
    /* The UUID of the incident. */
    pub incident_id: String,
}

// DeleteIncidentIntegrationParams is a struct for passing parameters to the method [`DeleteIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentIntegrationParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident integration metadata. */
    pub integration_metadata_id: String,
}

// DeleteIncidentTodoParams is a struct for passing parameters to the method [`DeleteIncidentTodo`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentTodoParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident todo. */
    pub todo_id: String,
}

// GetIncidentParams is a struct for passing parameters to the method [`GetIncident`]
#[derive(Clone, Debug)]
pub struct GetIncidentParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Specifies which types of related objects should be included in the response. */
    pub include: Vec<IncidentRelatedObject>,
}

// GetIncidentIntegrationParams is a struct for passing parameters to the method [`GetIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct GetIncidentIntegrationParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident integration metadata. */
    pub integration_metadata_id: String,
}

// GetIncidentTodoParams is a struct for passing parameters to the method [`GetIncidentTodo`]
#[derive(Clone, Debug)]
pub struct GetIncidentTodoParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident todo. */
    pub todo_id: String,
}

// ListIncidentAttachmentsParams is a struct for passing parameters to the method [`ListIncidentAttachments`]
#[derive(Clone, Debug)]
pub struct ListIncidentAttachmentsParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Specifies which types of related objects are included in the response. */
    pub include: Vec<IncidentAttachmentRelatedObject>,
    /* Specifies which types of attachments are included in the response. */
    pub filter_attachment_type: Vec<IncidentAttachmentAttachmentType>,
}

// ListIncidentIntegrationsParams is a struct for passing parameters to the method [`ListIncidentIntegrations`]
#[derive(Clone, Debug)]
pub struct ListIncidentIntegrationsParams {
    /* The UUID of the incident. */
    pub incident_id: String,
}

// ListIncidentTodosParams is a struct for passing parameters to the method [`ListIncidentTodos`]
#[derive(Clone, Debug)]
pub struct ListIncidentTodosParams {
    /* The UUID of the incident. */
    pub incident_id: String,
}

// ListIncidentsParams is a struct for passing parameters to the method [`ListIncidents`]
#[derive(Clone, Debug)]
pub struct ListIncidentsParams {
    /* Specifies which types of related objects should be included in the response. */
    pub include: Vec<IncidentRelatedObject>,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific offset to use as the beginning of the returned page. */
    pub page_offset: i64,
}

// SearchIncidentsParams is a struct for passing parameters to the method [`SearchIncidents`]
#[derive(Clone, Debug)]
pub struct SearchIncidentsParams {
    /* Specifies which incidents should be returned. After entering a search query in your [Incidents page][1],
use the query parameter value in the URL of the page as the value for this parameter.

The query can contain any number of incident facets joined by `ANDs`, along with multiple values for each of
those facets joined by `OR`s, for instance: `query="state:active AND severity:(SEV-2 OR SEV-1)"`.

[1]: https://app.datadoghq.com/incidents */
    pub query: String,
    /* Specifies which types of related objects should be included in the response. */
    pub include: IncidentRelatedObject,
    /* Specifies the order of returned incidents. */
    pub sort: IncidentSearchSortOrder,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific offset to use as the beginning of the returned page. */
    pub page_offset: i64,
}

// UpdateIncidentParams is a struct for passing parameters to the method [`UpdateIncident`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Incident Payload. */
    pub body: IncidentUpdateRequest,
    /* Specifies which types of related objects should be included in the response. */
    pub include: Vec<IncidentRelatedObject>,
}

// UpdateIncidentAttachmentsParams is a struct for passing parameters to the method [`UpdateIncidentAttachments`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentAttachmentsParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* Incident Attachment Payload. */
    pub body: IncidentAttachmentUpdateRequest,
    /* Specifies which types of related objects are included in the response. */
    pub include: Vec<IncidentAttachmentRelatedObject>,
}

// UpdateIncidentIntegrationParams is a struct for passing parameters to the method [`UpdateIncidentIntegration`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentIntegrationParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident integration metadata. */
    pub integration_metadata_id: String,
    /* Incident integration metadata payload. */
    pub body: IncidentIntegrationMetadataPatchRequest,
}

// UpdateIncidentTodoParams is a struct for passing parameters to the method [`UpdateIncidentTodo`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentTodoParams {
    /* The UUID of the incident. */
    pub incident_id: String,
    /* The UUID of the incident todo. */
    pub todo_id: String,
    /* Incident todo payload. */
    pub body: IncidentTodoPatchRequest,
}




/// CreateIncidentError is a struct for typed errors of method [`CreateIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateIncidentIntegrationError is a struct for typed errors of method [`CreateIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentIntegrationError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateIncidentTodoError is a struct for typed errors of method [`CreateIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentTodoError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentError is a struct for typed errors of method [`DeleteIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentIntegrationError is a struct for typed errors of method [`DeleteIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentIntegrationError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentTodoError is a struct for typed errors of method [`DeleteIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentTodoError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentError is a struct for typed errors of method [`GetIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentIntegrationError is a struct for typed errors of method [`GetIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentIntegrationError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentTodoError is a struct for typed errors of method [`GetIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentTodoError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentAttachmentsError is a struct for typed errors of method [`ListIncidentAttachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentAttachmentsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentIntegrationsError is a struct for typed errors of method [`ListIncidentIntegrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentIntegrationsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentTodosError is a struct for typed errors of method [`ListIncidentTodos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentTodosError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentsError is a struct for typed errors of method [`ListIncidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchIncidentsError is a struct for typed errors of method [`SearchIncidents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchIncidentsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentError is a struct for typed errors of method [`UpdateIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentAttachmentsError is a struct for typed errors of method [`UpdateIncidentAttachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentAttachmentsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentIntegrationError is a struct for typed errors of method [`UpdateIncidentIntegration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentIntegrationError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentTodoError is a struct for typed errors of method [`UpdateIncidentTodo`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentTodoError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}