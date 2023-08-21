// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// UpdateIPAllowlistParams is a struct for passing parameters to the method [`UpdateIPAllowlist`]
#[derive(Clone, Debug)]
pub struct UpdateIPAllowlistParams {
    pub body: IPAllowlistUpdateRequest,
}




/// GetIPAllowlistError is a struct for typed errors of method [`GetIPAllowlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIPAllowlistError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateIPAllowlistError is a struct for typed errors of method [`UpdateIPAllowlist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIPAllowlistError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}