// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateLogsPipelineParams is a struct for passing parameters to the method [`CreateLogsPipeline`]
#[derive(Clone, Debug)]
pub struct CreateLogsPipelineParams {
    /* Definition of the new pipeline. */
    pub body: LogsPipeline,
}

// DeleteLogsPipelineParams is a struct for passing parameters to the method [`DeleteLogsPipeline`]
#[derive(Clone, Debug)]
pub struct DeleteLogsPipelineParams {
    /* ID of the pipeline to delete. */
    pub pipeline_id: String,
}

// GetLogsPipelineParams is a struct for passing parameters to the method [`GetLogsPipeline`]
#[derive(Clone, Debug)]
pub struct GetLogsPipelineParams {
    /* ID of the pipeline to get. */
    pub pipeline_id: String,
}

// UpdateLogsPipelineParams is a struct for passing parameters to the method [`UpdateLogsPipeline`]
#[derive(Clone, Debug)]
pub struct UpdateLogsPipelineParams {
    /* ID of the pipeline to delete. */
    pub pipeline_id: String,
    /* New definition of the pipeline. */
    pub body: LogsPipeline,
}

// UpdateLogsPipelineOrderParams is a struct for passing parameters to the method [`UpdateLogsPipelineOrder`]
#[derive(Clone, Debug)]
pub struct UpdateLogsPipelineOrderParams {
    /* Object containing the new ordered list of pipeline IDs. */
    pub body: LogsPipelinesOrder,
}




/// CreateLogsPipelineError is a struct for typed errors of method [`CreateLogsPipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsPipelineError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsPipelineError is a struct for typed errors of method [`DeleteLogsPipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsPipelineError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsPipelineError is a struct for typed errors of method [`GetLogsPipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsPipelineError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetLogsPipelineOrderError is a struct for typed errors of method [`GetLogsPipelineOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsPipelineOrderError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLogsPipelinesError is a struct for typed errors of method [`ListLogsPipelines`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsPipelinesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsPipelineError is a struct for typed errors of method [`UpdateLogsPipeline`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsPipelineError {
    Status400(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsPipelineOrderError is a struct for typed errors of method [`UpdateLogsPipelineOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsPipelineOrderError {
    Status400(LogsAPIErrorResponse),
    Status422(LogsAPIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}