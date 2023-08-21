// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateTeamParams is a struct for passing parameters to the method [`CreateTeam`]
#[derive(Clone, Debug)]
pub struct CreateTeamParams {
    pub body: TeamCreateRequest,
}

// CreateTeamLinkParams is a struct for passing parameters to the method [`CreateTeamLink`]
#[derive(Clone, Debug)]
pub struct CreateTeamLinkParams {
    /* None */
    pub team_id: String,
    pub body: TeamLinkCreateRequest,
}

// CreateTeamMembershipParams is a struct for passing parameters to the method [`CreateTeamMembership`]
#[derive(Clone, Debug)]
pub struct CreateTeamMembershipParams {
    /* None */
    pub team_id: String,
    pub body: UserTeamRequest,
}

// DeleteTeamParams is a struct for passing parameters to the method [`DeleteTeam`]
#[derive(Clone, Debug)]
pub struct DeleteTeamParams {
    /* None */
    pub team_id: String,
}

// DeleteTeamLinkParams is a struct for passing parameters to the method [`DeleteTeamLink`]
#[derive(Clone, Debug)]
pub struct DeleteTeamLinkParams {
    /* None */
    pub team_id: String,
    /* None */
    pub link_id: String,
}

// DeleteTeamMembershipParams is a struct for passing parameters to the method [`DeleteTeamMembership`]
#[derive(Clone, Debug)]
pub struct DeleteTeamMembershipParams {
    /* None */
    pub team_id: String,
    /* None */
    pub user_id: String,
}

// GetTeamParams is a struct for passing parameters to the method [`GetTeam`]
#[derive(Clone, Debug)]
pub struct GetTeamParams {
    /* None */
    pub team_id: String,
}

// GetTeamLinkParams is a struct for passing parameters to the method [`GetTeamLink`]
#[derive(Clone, Debug)]
pub struct GetTeamLinkParams {
    /* None */
    pub team_id: String,
    /* None */
    pub link_id: String,
}

// GetTeamLinksParams is a struct for passing parameters to the method [`GetTeamLinks`]
#[derive(Clone, Debug)]
pub struct GetTeamLinksParams {
    /* None */
    pub team_id: String,
}

// GetTeamMembershipsParams is a struct for passing parameters to the method [`GetTeamMemberships`]
#[derive(Clone, Debug)]
pub struct GetTeamMembershipsParams {
    /* None */
    pub team_id: String,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* Specifies the order of returned team memberships */
    pub sort: GetTeamMembershipsSort,
    /* Search query, can be user email or name */
    pub filter_keyword: String,
}

// GetTeamPermissionSettingsParams is a struct for passing parameters to the method [`GetTeamPermissionSettings`]
#[derive(Clone, Debug)]
pub struct GetTeamPermissionSettingsParams {
    /* None */
    pub team_id: String,
}

// ListTeamsParams is a struct for passing parameters to the method [`ListTeams`]
#[derive(Clone, Debug)]
pub struct ListTeamsParams {
    /* Specific page number to return. */
    pub page_number: i64,
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specifies the order of the returned teams */
    pub sort: ListTeamsSort,
    /* Included related resources optionally requested. Allowed enum values: `team_links, user_team_permissions` */
    pub include: Vec<ListTeamsInclude>,
    /* Search query. Can be team name, team handle, or email of team member */
    pub filter_keyword: String,
    /* When true, only returns teams the current user belongs to */
    pub filter_me: bool,
}

// UpdateTeamParams is a struct for passing parameters to the method [`UpdateTeam`]
#[derive(Clone, Debug)]
pub struct UpdateTeamParams {
    /* None */
    pub team_id: String,
    pub body: TeamUpdateRequest,
}

// UpdateTeamLinkParams is a struct for passing parameters to the method [`UpdateTeamLink`]
#[derive(Clone, Debug)]
pub struct UpdateTeamLinkParams {
    /* None */
    pub team_id: String,
    /* None */
    pub link_id: String,
    pub body: TeamLinkCreateRequest,
}

// UpdateTeamMembershipParams is a struct for passing parameters to the method [`UpdateTeamMembership`]
#[derive(Clone, Debug)]
pub struct UpdateTeamMembershipParams {
    /* None */
    pub team_id: String,
    /* None */
    pub user_id: String,
    pub body: UserTeamUpdateRequest,
}

// UpdateTeamPermissionSettingParams is a struct for passing parameters to the method [`UpdateTeamPermissionSetting`]
#[derive(Clone, Debug)]
pub struct UpdateTeamPermissionSettingParams {
    /* None */
    pub team_id: String,
    /* None */
    pub action: String,
    pub body: TeamPermissionSettingUpdateRequest,
}




/// CreateTeamError is a struct for typed errors of method [`CreateTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamError {
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateTeamLinkError is a struct for typed errors of method [`CreateTeamLink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamLinkError {
    Status404(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateTeamMembershipError is a struct for typed errors of method [`CreateTeamMembership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamMembershipError {
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamError is a struct for typed errors of method [`DeleteTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamLinkError is a struct for typed errors of method [`DeleteTeamLink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamLinkError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamMembershipError is a struct for typed errors of method [`DeleteTeamMembership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamMembershipError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamError is a struct for typed errors of method [`GetTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamLinkError is a struct for typed errors of method [`GetTeamLink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamLinkError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamLinksError is a struct for typed errors of method [`GetTeamLinks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamLinksError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamMembershipsError is a struct for typed errors of method [`GetTeamMemberships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamMembershipsError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTeamPermissionSettingsError is a struct for typed errors of method [`GetTeamPermissionSettings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamPermissionSettingsError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTeamsError is a struct for typed errors of method [`ListTeams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsError {
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamError is a struct for typed errors of method [`UpdateTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamError {
    Status400(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamLinkError is a struct for typed errors of method [`UpdateTeamLink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamLinkError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamMembershipError is a struct for typed errors of method [`UpdateTeamMembership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamMembershipError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamPermissionSettingError is a struct for typed errors of method [`UpdateTeamPermissionSetting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamPermissionSettingError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}