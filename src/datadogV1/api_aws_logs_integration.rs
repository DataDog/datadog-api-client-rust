// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CheckAWSLogsLambdaAsyncParams is a struct for passing parameters to the method [`CheckAWSLogsLambdaAsync`]
#[derive(Clone, Debug)]
pub struct CheckAWSLogsLambdaAsyncParams {
    /* Check AWS Log Lambda Async request body. */
    pub body: AWSAccountAndLambdaRequest,
}

// CheckAWSLogsServicesAsyncParams is a struct for passing parameters to the method [`CheckAWSLogsServicesAsync`]
#[derive(Clone, Debug)]
pub struct CheckAWSLogsServicesAsyncParams {
    /* Check AWS Logs Async Services request body. */
    pub body: AWSLogsServicesRequest,
}

// CreateAWSLambdaARNParams is a struct for passing parameters to the method [`CreateAWSLambdaARN`]
#[derive(Clone, Debug)]
pub struct CreateAWSLambdaARNParams {
    /* AWS Log Lambda Async request body. */
    pub body: AWSAccountAndLambdaRequest,
}

// DeleteAWSLambdaARNParams is a struct for passing parameters to the method [`DeleteAWSLambdaARN`]
#[derive(Clone, Debug)]
pub struct DeleteAWSLambdaARNParams {
    /* Delete AWS Lambda ARN request body. */
    pub body: AWSAccountAndLambdaRequest,
}

// EnableAWSLogServicesParams is a struct for passing parameters to the method [`EnableAWSLogServices`]
#[derive(Clone, Debug)]
pub struct EnableAWSLogServicesParams {
    /* Enable AWS Log Services request body. */
    pub body: AWSLogsServicesRequest,
}




/// CheckAWSLogsLambdaAsyncError is a struct for typed errors of method [`CheckAWSLogsLambdaAsync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckAWSLogsLambdaAsyncError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CheckAWSLogsServicesAsyncError is a struct for typed errors of method [`CheckAWSLogsServicesAsync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckAWSLogsServicesAsyncError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateAWSLambdaARNError is a struct for typed errors of method [`CreateAWSLambdaARN`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAWSLambdaARNError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAWSLambdaARNError is a struct for typed errors of method [`DeleteAWSLambdaARN`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAWSLambdaARNError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EnableAWSLogServicesError is a struct for typed errors of method [`EnableAWSLogServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnableAWSLogServicesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAWSLogsIntegrationsError is a struct for typed errors of method [`ListAWSLogsIntegrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSLogsIntegrationsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAWSLogsServicesError is a struct for typed errors of method [`ListAWSLogsServices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAWSLogsServicesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}