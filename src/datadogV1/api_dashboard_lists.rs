// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateDashboardListParams is a struct for passing parameters to the method [`CreateDashboardList`]
#[derive(Clone, Debug)]
pub struct CreateDashboardListParams {
    /* Create a dashboard list request body. */
    pub body: DashboardList,
}

// DeleteDashboardListParams is a struct for passing parameters to the method [`DeleteDashboardList`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardListParams {
    /* ID of the dashboard list to delete. */
    pub list_id: i64,
}

// GetDashboardListParams is a struct for passing parameters to the method [`GetDashboardList`]
#[derive(Clone, Debug)]
pub struct GetDashboardListParams {
    /* ID of the dashboard list to fetch. */
    pub list_id: i64,
}

// UpdateDashboardListParams is a struct for passing parameters to the method [`UpdateDashboardList`]
#[derive(Clone, Debug)]
pub struct UpdateDashboardListParams {
    /* ID of the dashboard list to update. */
    pub list_id: i64,
    /* Update a dashboard list request body. */
    pub body: DashboardList,
}




/// CreateDashboardListError is a struct for typed errors of method [`CreateDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardListError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardListError is a struct for typed errors of method [`DeleteDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardListError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDashboardListError is a struct for typed errors of method [`GetDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardListError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListDashboardListsError is a struct for typed errors of method [`ListDashboardLists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDashboardListsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardListError is a struct for typed errors of method [`UpdateDashboardList`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardListError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}