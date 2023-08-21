// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// AddSecurityMonitoringSignalToIncidentParams is a struct for passing parameters to the method [`AddSecurityMonitoringSignalToIncident`]
#[derive(Clone, Debug)]
pub struct AddSecurityMonitoringSignalToIncidentParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: AddSignalToIncidentRequest,
}

// EditSecurityMonitoringSignalAssigneeParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalAssigneeParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: SignalAssigneeUpdateRequest,
}

// EditSecurityMonitoringSignalStateParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalState`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalStateParams {
    /* The ID of the signal. */
    pub signal_id: String,
    /* Attributes describing the signal update. */
    pub body: SignalStateUpdateRequest,
}




/// AddSecurityMonitoringSignalToIncidentError is a struct for typed errors of method [`AddSecurityMonitoringSignalToIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSecurityMonitoringSignalToIncidentError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`EditSecurityMonitoringSignalState`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalStateError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}