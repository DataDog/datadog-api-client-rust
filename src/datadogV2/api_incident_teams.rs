// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateIncidentTeamParams is a struct for passing parameters to the method [`CreateIncidentTeam`]
#[derive(Clone, Debug)]
pub struct CreateIncidentTeamParams {
    /* Incident Team Payload. */
    pub body: IncidentTeamCreateRequest,
}

// DeleteIncidentTeamParams is a struct for passing parameters to the method [`DeleteIncidentTeam`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentTeamParams {
    /* The ID of the incident team. */
    pub team_id: String,
}

// GetIncidentTeamParams is a struct for passing parameters to the method [`GetIncidentTeam`]
#[derive(Clone, Debug)]
pub struct GetIncidentTeamParams {
    /* The ID of the incident team. */
    pub team_id: String,
    /* Specifies which types of related objects should be included in the response. */
    pub include: IncidentRelatedObject,
}

// ListIncidentTeamsParams is a struct for passing parameters to the method [`ListIncidentTeams`]
#[derive(Clone, Debug)]
pub struct ListIncidentTeamsParams {
    /* Specifies which types of related objects should be included in the response. */
    pub include: IncidentRelatedObject,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific offset to use as the beginning of the returned page. */
    pub page_offset: i64,
    /* A search query that filters teams by name. */
    pub filter: String,
}

// UpdateIncidentTeamParams is a struct for passing parameters to the method [`UpdateIncidentTeam`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentTeamParams {
    /* The ID of the incident team. */
    pub team_id: String,
    /* Incident Team Payload. */
    pub body: IncidentTeamUpdateRequest,
}




/// CreateIncidentTeamError is a struct for typed errors of method [`CreateIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentTeamError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentTeamError is a struct for typed errors of method [`DeleteIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentTeamError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetIncidentTeamError is a struct for typed errors of method [`GetIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentTeamError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListIncidentTeamsError is a struct for typed errors of method [`ListIncidentTeams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentTeamsError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentTeamError is a struct for typed errors of method [`UpdateIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentTeamError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}