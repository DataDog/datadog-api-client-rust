// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AddReadRoleToArchiveParams is a struct for passing parameters to the method [`AddReadRoleToArchive`]
#[derive(Clone, Debug)]
pub struct AddReadRoleToArchiveParams {
    /* The ID of the archive. */
    pub archive_id: String,
    pub body: RelationshipToRole,
}

// CreateLogsArchiveParams is a struct for passing parameters to the method [`CreateLogsArchive`]
#[derive(Clone, Debug)]
pub struct CreateLogsArchiveParams {
    /* The definition of the new archive. */
    pub body: LogsArchiveCreateRequest,
}

// DeleteLogsArchiveParams is a struct for passing parameters to the method [`DeleteLogsArchive`]
#[derive(Clone, Debug)]
pub struct DeleteLogsArchiveParams {
    /* The ID of the archive. */
    pub archive_id: String,
}

// GetLogsArchiveParams is a struct for passing parameters to the method [`GetLogsArchive`]
#[derive(Clone, Debug)]
pub struct GetLogsArchiveParams {
    /* The ID of the archive. */
    pub archive_id: String,
}

// ListArchiveReadRolesParams is a struct for passing parameters to the method [`ListArchiveReadRoles`]
#[derive(Clone, Debug)]
pub struct ListArchiveReadRolesParams {
    /* The ID of the archive. */
    pub archive_id: String,
}

// RemoveRoleFromArchiveParams is a struct for passing parameters to the method [`RemoveRoleFromArchive`]
#[derive(Clone, Debug)]
pub struct RemoveRoleFromArchiveParams {
    /* The ID of the archive. */
    pub archive_id: String,
    pub body: RelationshipToRole,
}

// UpdateLogsArchiveParams is a struct for passing parameters to the method [`UpdateLogsArchive`]
#[derive(Clone, Debug)]
pub struct UpdateLogsArchiveParams {
    /* The ID of the archive. */
    pub archive_id: String,
    /* New definition of the archive. */
    pub body: LogsArchiveCreateRequest,
}

// UpdateLogsArchiveOrderParams is a struct for passing parameters to the method [`UpdateLogsArchiveOrder`]
#[derive(Clone, Debug)]
pub struct UpdateLogsArchiveOrderParams {
    /* An object containing the new ordered list of archive IDs. */
    pub body: LogsArchiveOrder,
}




/// AddReadRoleToArchiveError is a struct for typed errors of method [`AddReadRoleToArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddReadRoleToArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateLogsArchiveError is a struct for typed errors of method [`CreateLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsArchiveError is a struct for typed errors of method [`DeleteLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveError is a struct for typed errors of method [`GetLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveOrderError is a struct for typed errors of method [`GetLogsArchiveOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveOrderError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListArchiveReadRolesError is a struct for typed errors of method [`ListArchiveReadRoles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchiveReadRolesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogsArchivesError is a struct for typed errors of method [`ListLogsArchives`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsArchivesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RemoveRoleFromArchiveError is a struct for typed errors of method [`RemoveRoleFromArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRoleFromArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveError is a struct for typed errors of method [`UpdateLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsArchiveError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveOrderError is a struct for typed errors of method [`UpdateLogsArchiveOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsArchiveOrderError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}