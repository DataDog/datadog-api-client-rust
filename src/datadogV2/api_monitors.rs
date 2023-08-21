// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateMonitorConfigPolicyParams is a struct for passing parameters to the method [`CreateMonitorConfigPolicy`]
#[derive(Clone, Debug)]
pub struct CreateMonitorConfigPolicyParams {
    /* Create a monitor configuration policy request body. */
    pub body: MonitorConfigPolicyCreateRequest,
}

// DeleteMonitorConfigPolicyParams is a struct for passing parameters to the method [`DeleteMonitorConfigPolicy`]
#[derive(Clone, Debug)]
pub struct DeleteMonitorConfigPolicyParams {
    /* ID of the monitor configuration policy. */
    pub policy_id: String,
}

// GetMonitorConfigPolicyParams is a struct for passing parameters to the method [`GetMonitorConfigPolicy`]
#[derive(Clone, Debug)]
pub struct GetMonitorConfigPolicyParams {
    /* ID of the monitor configuration policy. */
    pub policy_id: String,
}

// UpdateMonitorConfigPolicyParams is a struct for passing parameters to the method [`UpdateMonitorConfigPolicy`]
#[derive(Clone, Debug)]
pub struct UpdateMonitorConfigPolicyParams {
    /* ID of the monitor configuration policy. */
    pub policy_id: String,
    /* Description of the update. */
    pub body: MonitorConfigPolicyEditRequest,
}




/// CreateMonitorConfigPolicyError is a struct for typed errors of method [`CreateMonitorConfigPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMonitorConfigPolicyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteMonitorConfigPolicyError is a struct for typed errors of method [`DeleteMonitorConfigPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMonitorConfigPolicyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonitorConfigPolicyError is a struct for typed errors of method [`GetMonitorConfigPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonitorConfigPolicyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMonitorConfigPoliciesError is a struct for typed errors of method [`ListMonitorConfigPolicies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorConfigPoliciesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMonitorConfigPolicyError is a struct for typed errors of method [`UpdateMonitorConfigPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMonitorConfigPolicyError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}