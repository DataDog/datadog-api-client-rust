// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object with all Index configurations for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexListResponse {
    /// Array of Log index configurations.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<crate::datadogV1::model::LogsIndex>>,
}

impl LogsIndexListResponse {
    pub fn new() -> LogsIndexListResponse {
        LogsIndexListResponse { indexes: None }
    }

    pub fn with_indexes(&mut self, value: Vec<crate::datadogV1::model::LogsIndex>) -> &mut Self {
        self.indexes = Some(value);
        self
    }
}
impl Default for LogsIndexListResponse {
    fn default() -> Self {
        Self::new()
    }
}
