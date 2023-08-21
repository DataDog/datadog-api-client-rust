// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateDashboardListItemsParams is a struct for passing parameters to the method [`CreateDashboardListItems`]
#[derive(Clone, Debug)]
pub struct CreateDashboardListItemsParams {
    /* ID of the dashboard list to add items to. */
    pub dashboard_list_id: i64,
    /* Dashboards to add to the dashboard list. */
    pub body: DashboardListAddItemsRequest,
}

// DeleteDashboardListItemsParams is a struct for passing parameters to the method [`DeleteDashboardListItems`]
#[derive(Clone, Debug)]
pub struct DeleteDashboardListItemsParams {
    /* ID of the dashboard list to delete items from. */
    pub dashboard_list_id: i64,
    /* Dashboards to delete from the dashboard list. */
    pub body: DashboardListDeleteItemsRequest,
}

// GetDashboardListItemsParams is a struct for passing parameters to the method [`GetDashboardListItems`]
#[derive(Clone, Debug)]
pub struct GetDashboardListItemsParams {
    /* ID of the dashboard list to get items from. */
    pub dashboard_list_id: i64,
}

// UpdateDashboardListItemsParams is a struct for passing parameters to the method [`UpdateDashboardListItems`]
#[derive(Clone, Debug)]
pub struct UpdateDashboardListItemsParams {
    /* ID of the dashboard list to update items from. */
    pub dashboard_list_id: i64,
    /* New dashboards of the dashboard list. */
    pub body: DashboardListUpdateItemsRequest,
}




/// CreateDashboardListItemsError is a struct for typed errors of method [`CreateDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDashboardListItemsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteDashboardListItemsError is a struct for typed errors of method [`DeleteDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDashboardListItemsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDashboardListItemsError is a struct for typed errors of method [`GetDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDashboardListItemsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateDashboardListItemsError is a struct for typed errors of method [`UpdateDashboardListItems`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDashboardListItemsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}