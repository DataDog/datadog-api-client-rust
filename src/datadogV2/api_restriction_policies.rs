// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// DeleteRestrictionPolicyParams is a struct for passing parameters to the method [`DeleteRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct DeleteRestrictionPolicyParams {
    /* Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`. */
    pub resource_id: String,
}

// GetRestrictionPolicyParams is a struct for passing parameters to the method [`GetRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct GetRestrictionPolicyParams {
    /* Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`. */
    pub resource_id: String,
}

// UpdateRestrictionPolicyParams is a struct for passing parameters to the method [`UpdateRestrictionPolicy`]
#[derive(Clone, Debug)]
pub struct UpdateRestrictionPolicyParams {
    /* Identifier, formatted as `type:id`. Supported types: `connection`, `dashboard`, `notebook`, `security-rule`, `slo`. */
    pub resource_id: String,
    /* Restriction policy payload */
    pub body: RestrictionPolicyUpdateRequest,
}




/// DeleteRestrictionPolicyError is a struct for typed errors of method [`DeleteRestrictionPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRestrictionPolicyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRestrictionPolicyError is a struct for typed errors of method [`GetRestrictionPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRestrictionPolicyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRestrictionPolicyError is a struct for typed errors of method [`UpdateRestrictionPolicy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRestrictionPolicyError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}