// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateDashboardParams is a struct for passing parameters to the method [`CreateDashboard`]
#[derive(Clone, Debug)]
pub struct CreateDashboardParams {
    /* Create a dashboard request body. */
    pub body: Dashboard,
}

// CreatePublicDashboardParams is a struct for passing parameters to the method [`CreatePublicDashboard`]
#[derive(Clone, Debug)]
pub struct CreatePublicDashboardParams {
    /* Create a shared dashboard request body. */
    pub body: SharedDashboard,
}

// DeleteDashboardParams is a struct for passing parameters to the method [`DeleteDashboard`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardParams {
    /* The ID of the dashboard. */
    pub dashboard_id: String,
}

// DeleteDashboardsParams is a struct for passing parameters to the method [`DeleteDashboards`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardsParams {
    /* Delete dashboards request body. */
    pub body: DashboardBulkDeleteRequest,
}

// DeletePublicDashboardParams is a struct for passing parameters to the method [`DeletePublicDashboard`]
#[derive(Clone, Debug)]
pub struct DeletePublicDashboardParams {
    /* The token of the shared dashboard. */
    pub token: String,
}

// DeletePublicDashboardInvitationParams is a struct for passing parameters to the method [`DeletePublicDashboardInvitation`]
#[derive(Clone, Debug)]
pub struct DeletePublicDashboardInvitationParams {
    /* The token of the shared dashboard. */
    pub token: String,
    /* Shared Dashboard Invitation deletion request body. */
    pub body: SharedDashboardInvites,
}

// GetDashboardParams is a struct for passing parameters to the method [`GetDashboard`]
#[derive(Clone, Debug)]
pub struct GetDashboardParams {
    /* The ID of the dashboard. */
    pub dashboard_id: String,
}

// GetPublicDashboardParams is a struct for passing parameters to the method [`GetPublicDashboard`]
#[derive(Clone, Debug)]
pub struct GetPublicDashboardParams {
    /* The token of the shared dashboard. Generated when a dashboard is shared. */
    pub token: String,
}

// GetPublicDashboardInvitationsParams is a struct for passing parameters to the method [`GetPublicDashboardInvitations`]
#[derive(Clone, Debug)]
pub struct GetPublicDashboardInvitationsParams {
    /* Token of the shared dashboard for which to fetch invitations. */
    pub token: String,
    /* The number of records to return in a single request. */
    pub page_size: i64,
    /* The page to access (base 0). */
    pub page_number: i64,
}

// ListDashboardsParams is a struct for passing parameters to the method [`ListDashboards`]
#[derive(Clone, Debug)]
pub struct ListDashboardsParams {
    /* When `true`, this query only returns shared custom created
or cloned dashboards. */
    pub filter_shared: bool,
    /* When `true`, this query returns only deleted custom-created
or cloned dashboards. This parameter is incompatible with `filter[shared]`. */
    pub filter_deleted: bool,
}

// RestoreDashboardsParams is a struct for passing parameters to the method [`RestoreDashboards`]
#[derive(Clone, Debug)]
pub struct RestoreDashboardsParams {
    /* Restore dashboards request body. */
    pub body: DashboardRestoreRequest,
}

// SendPublicDashboardInvitationParams is a struct for passing parameters to the method [`SendPublicDashboardInvitation`]
#[derive(Clone, Debug)]
pub struct SendPublicDashboardInvitationParams {
    /* The token of the shared dashboard. */
    pub token: String,
    /* Shared Dashboard Invitation request body. */
    pub body: SharedDashboardInvites,
}

// UpdateDashboardParams is a struct for passing parameters to the method [`UpdateDashboard`]
#[derive(Clone, Debug)]
pub struct UpdateDashboardParams {
    /* The ID of the dashboard. */
    pub dashboard_id: String,
    /* Update Dashboard request body. */
    pub body: Dashboard,
}

// UpdatePublicDashboardParams is a struct for passing parameters to the method [`UpdatePublicDashboard`]
#[derive(Clone, Debug)]
pub struct UpdatePublicDashboardParams {
    /* The token of the shared dashboard. */
    pub token: String,
    /* Update Dashboard request body. */
    pub body: SharedDashboardUpdateRequest,
}




/// CreateDashboardError is a struct for typed errors of method [`CreateDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreatePublicDashboardError is a struct for typed errors of method [`CreatePublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePublicDashboardError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardError is a struct for typed errors of method [`DeleteDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardsError is a struct for typed errors of method [`DeleteDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardError is a struct for typed errors of method [`DeletePublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeletePublicDashboardInvitationError is a struct for typed errors of method [`DeletePublicDashboardInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePublicDashboardInvitationError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDashboardError is a struct for typed errors of method [`GetDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardError is a struct for typed errors of method [`GetPublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetPublicDashboardInvitationsError is a struct for typed errors of method [`GetPublicDashboardInvitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublicDashboardInvitationsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListDashboardsError is a struct for typed errors of method [`ListDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RestoreDashboardsError is a struct for typed errors of method [`RestoreDashboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreDashboardsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SendPublicDashboardInvitationError is a struct for typed errors of method [`SendPublicDashboardInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPublicDashboardInvitationError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardError is a struct for typed errors of method [`UpdateDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdatePublicDashboardError is a struct for typed errors of method [`UpdatePublicDashboard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePublicDashboardError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}