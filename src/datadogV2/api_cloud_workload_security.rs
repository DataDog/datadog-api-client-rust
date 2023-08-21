// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateCloudWorkloadSecurityAgentRuleParams is a struct for passing parameters to the method [`CreateCloudWorkloadSecurityAgentRule`]
#[derive(Clone, Debug)]
pub struct CreateCloudWorkloadSecurityAgentRuleParams {
    /* The definition of the new Agent rule. */
    pub body: CloudWorkloadSecurityAgentRuleCreateRequest,
}

// DeleteCloudWorkloadSecurityAgentRuleParams is a struct for passing parameters to the method [`DeleteCloudWorkloadSecurityAgentRule`]
#[derive(Clone, Debug)]
pub struct DeleteCloudWorkloadSecurityAgentRuleParams {
    /* The ID of the Agent rule. */
    pub agent_rule_id: String,
}

// GetCloudWorkloadSecurityAgentRuleParams is a struct for passing parameters to the method [`GetCloudWorkloadSecurityAgentRule`]
#[derive(Clone, Debug)]
pub struct GetCloudWorkloadSecurityAgentRuleParams {
    /* The ID of the Agent rule. */
    pub agent_rule_id: String,
}

// UpdateCloudWorkloadSecurityAgentRuleParams is a struct for passing parameters to the method [`UpdateCloudWorkloadSecurityAgentRule`]
#[derive(Clone, Debug)]
pub struct UpdateCloudWorkloadSecurityAgentRuleParams {
    /* The ID of the Agent rule. */
    pub agent_rule_id: String,
    /* New definition of the Agent rule. */
    pub body: CloudWorkloadSecurityAgentRuleUpdateRequest,
}




/// CreateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`CreateCloudWorkloadSecurityAgentRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudWorkloadSecurityAgentRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`DeleteCloudWorkloadSecurityAgentRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudWorkloadSecurityAgentRuleError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DownloadCloudWorkloadPolicyFileError is a struct for typed errors of method [`DownloadCloudWorkloadPolicyFile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCloudWorkloadPolicyFileError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`GetCloudWorkloadSecurityAgentRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudWorkloadSecurityAgentRuleError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCloudWorkloadSecurityAgentRulesError is a struct for typed errors of method [`ListCloudWorkloadSecurityAgentRules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudWorkloadSecurityAgentRulesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateCloudWorkloadSecurityAgentRuleError is a struct for typed errors of method [`UpdateCloudWorkloadSecurityAgentRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCloudWorkloadSecurityAgentRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}