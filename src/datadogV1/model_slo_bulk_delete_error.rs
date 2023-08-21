// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOBulkDeleteError {
    /// The ID of the service level objective object associated with
this error.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// The error message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: String,
    /// The timeframe of the threshold associated with this error
or "all" if all thresholds are affected.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: SLOErrorTimeframe,
}

