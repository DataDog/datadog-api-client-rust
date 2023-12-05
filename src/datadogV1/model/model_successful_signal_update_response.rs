// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated signal data following a successfully performed update.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulSignalUpdateResponse {
    /// Status of the response.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl SuccessfulSignalUpdateResponse {
    pub fn new() -> SuccessfulSignalUpdateResponse {
        SuccessfulSignalUpdateResponse { status: None }
    }
}