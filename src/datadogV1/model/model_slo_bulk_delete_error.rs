// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOBulkDeleteError {
    /// The ID of the service level objective object associated with
    /// this error.
    #[serde(rename = "id")]
    pub id: String,
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
    /// The timeframe of the threshold associated with this error
    /// or "all" if all thresholds are affected.
    #[serde(rename = "timeframe")]
    pub timeframe: crate::datadogV1::model::SLOErrorTimeframe,
}

impl SLOBulkDeleteError {
    pub fn new(
        id: String,
        message: String,
        timeframe: crate::datadogV1::model::SLOErrorTimeframe,
    ) -> SLOBulkDeleteError {
        SLOBulkDeleteError {
            id,
            message,
            timeframe,
        }
    }
}
