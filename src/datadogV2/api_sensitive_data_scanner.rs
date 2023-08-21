// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateScanningGroupParams is a struct for passing parameters to the method [`CreateScanningGroup`]
#[derive(Clone, Debug)]
pub struct CreateScanningGroupParams {
    pub body: SensitiveDataScannerGroupCreateRequest,
}

// CreateScanningRuleParams is a struct for passing parameters to the method [`CreateScanningRule`]
#[derive(Clone, Debug)]
pub struct CreateScanningRuleParams {
    pub body: SensitiveDataScannerRuleCreateRequest,
}

// DeleteScanningGroupParams is a struct for passing parameters to the method [`DeleteScanningGroup`]
#[derive(Clone, Debug)]
pub struct DeleteScanningGroupParams {
    /* The ID of a group of rules. */
    pub group_id: String,
    pub body: SensitiveDataScannerGroupDeleteRequest,
}

// DeleteScanningRuleParams is a struct for passing parameters to the method [`DeleteScanningRule`]
#[derive(Clone, Debug)]
pub struct DeleteScanningRuleParams {
    /* The ID of the rule. */
    pub rule_id: String,
    pub body: SensitiveDataScannerRuleDeleteRequest,
}

// ReorderScanningGroupsParams is a struct for passing parameters to the method [`ReorderScanningGroups`]
#[derive(Clone, Debug)]
pub struct ReorderScanningGroupsParams {
    pub body: SensitiveDataScannerConfigRequest,
}

// UpdateScanningGroupParams is a struct for passing parameters to the method [`UpdateScanningGroup`]
#[derive(Clone, Debug)]
pub struct UpdateScanningGroupParams {
    /* The ID of a group of rules. */
    pub group_id: String,
    pub body: SensitiveDataScannerGroupUpdateRequest,
}

// UpdateScanningRuleParams is a struct for passing parameters to the method [`UpdateScanningRule`]
#[derive(Clone, Debug)]
pub struct UpdateScanningRuleParams {
    /* The ID of the rule. */
    pub rule_id: String,
    pub body: SensitiveDataScannerRuleUpdateRequest,
}




/// CreateScanningGroupError is a struct for typed errors of method [`CreateScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningGroupError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateScanningRuleError is a struct for typed errors of method [`CreateScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateScanningRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningGroupError is a struct for typed errors of method [`DeleteScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningGroupError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteScanningRuleError is a struct for typed errors of method [`DeleteScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteScanningRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListScanningGroupsError is a struct for typed errors of method [`ListScanningGroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListScanningGroupsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListStandardPatternsError is a struct for typed errors of method [`ListStandardPatterns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListStandardPatternsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ReorderScanningGroupsError is a struct for typed errors of method [`ReorderScanningGroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderScanningGroupsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningGroupError is a struct for typed errors of method [`UpdateScanningGroup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScanningGroupError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateScanningRuleError is a struct for typed errors of method [`UpdateScanningRule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateScanningRuleError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}