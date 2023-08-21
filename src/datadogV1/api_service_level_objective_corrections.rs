// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateSLOCorrectionParams is a struct for passing parameters to the method [`CreateSLOCorrection`]
#[derive(Clone, Debug)]
pub struct CreateSLOCorrectionParams {
    /* Create an SLO Correction */
    pub body: SLOCorrectionCreateRequest,
}

// DeleteSLOCorrectionParams is a struct for passing parameters to the method [`DeleteSLOCorrection`]
#[derive(Clone, Debug)]
pub struct DeleteSLOCorrectionParams {
    /* The ID of the SLO correction object. */
    pub slo_correction_id: String,
}

// GetSLOCorrectionParams is a struct for passing parameters to the method [`GetSLOCorrection`]
#[derive(Clone, Debug)]
pub struct GetSLOCorrectionParams {
    /* The ID of the SLO correction object. */
    pub slo_correction_id: String,
}

// ListSLOCorrectionParams is a struct for passing parameters to the method [`ListSLOCorrection`]
#[derive(Clone, Debug)]
pub struct ListSLOCorrectionParams {
    /* The specific offset to use as the beginning of the returned response. */
    pub offset: i64,
    /* The number of SLO corrections to return in the response. Default is 25. */
    pub limit: i64,
}

// UpdateSLOCorrectionParams is a struct for passing parameters to the method [`UpdateSLOCorrection`]
#[derive(Clone, Debug)]
pub struct UpdateSLOCorrectionParams {
    /* The ID of the SLO correction object. */
    pub slo_correction_id: String,
    /* The edited SLO correction object. */
    pub body: SLOCorrectionUpdateRequest,
}




/// CreateSLOCorrectionError is a struct for typed errors of method [`CreateSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOCorrectionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOCorrectionError is a struct for typed errors of method [`DeleteSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOCorrectionError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionError is a struct for typed errors of method [`GetSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSLOCorrectionError is a struct for typed errors of method [`ListSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOCorrectionError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOCorrectionError is a struct for typed errors of method [`UpdateSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSLOCorrectionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}