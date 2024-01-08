// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A list of all Datadog-AWS logs integrations available in your Datadog organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsAsyncResponse {
    /// List of errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::AWSLogsAsyncError>>,
    /// Status of the properties.
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl AWSLogsAsyncResponse {
    pub fn new() -> AWSLogsAsyncResponse {
        AWSLogsAsyncResponse {
            errors: None,
            status: None,
        }
    }
}
impl Default for AWSLogsAsyncResponse {
    fn default() -> Self {
        Self::new()
    }
}
