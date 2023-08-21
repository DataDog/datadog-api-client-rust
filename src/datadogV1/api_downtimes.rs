// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CancelDowntimeParams is a struct for passing parameters to the method [`CancelDowntime`]
#[derive(Clone, Debug)]
pub struct CancelDowntimeParams {
    /* ID of the downtime to cancel. */
    pub downtime_id: i64,
}

// CancelDowntimesByScopeParams is a struct for passing parameters to the method [`CancelDowntimesByScope`]
#[derive(Clone, Debug)]
pub struct CancelDowntimesByScopeParams {
    /* Scope to cancel downtimes for. */
    pub body: CancelDowntimesByScopeRequest,
}

// CreateDowntimeParams is a struct for passing parameters to the method [`CreateDowntime`]
#[derive(Clone, Debug)]
pub struct CreateDowntimeParams {
    /* Schedule a downtime request body. */
    pub body: Downtime,
}

// GetDowntimeParams is a struct for passing parameters to the method [`GetDowntime`]
#[derive(Clone, Debug)]
pub struct GetDowntimeParams {
    /* ID of the downtime to fetch. */
    pub downtime_id: i64,
}

// ListDowntimesParams is a struct for passing parameters to the method [`ListDowntimes`]
#[derive(Clone, Debug)]
pub struct ListDowntimesParams {
    /* Only return downtimes that are active when the request is made. */
    pub current_only: bool,
    /* Return creator information. */
    pub with_creator: bool,
}

// ListMonitorDowntimesParams is a struct for passing parameters to the method [`ListMonitorDowntimes`]
#[derive(Clone, Debug)]
pub struct ListMonitorDowntimesParams {
    /* The id of the monitor */
    pub monitor_id: i64,
}

// UpdateDowntimeParams is a struct for passing parameters to the method [`UpdateDowntime`]
#[derive(Clone, Debug)]
pub struct UpdateDowntimeParams {
    /* ID of the downtime to update. */
    pub downtime_id: i64,
    /* Update a downtime request body. */
    pub body: Downtime,
}




/// CancelDowntimeError is a struct for typed errors of method [`CancelDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimeError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CancelDowntimesByScopeError is a struct for typed errors of method [`CancelDowntimesByScope`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimesByScopeError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateDowntimeError is a struct for typed errors of method [`CreateDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDowntimeError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetDowntimeError is a struct for typed errors of method [`GetDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDowntimeError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListDowntimesError is a struct for typed errors of method [`ListDowntimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDowntimesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMonitorDowntimesError is a struct for typed errors of method [`ListMonitorDowntimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorDowntimesError {
    Status400(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateDowntimeError is a struct for typed errors of method [`UpdateDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDowntimeError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}