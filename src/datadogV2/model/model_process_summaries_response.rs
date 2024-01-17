// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of process summaries.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesResponse {
    /// Array of process summary objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ProcessSummary>>,
    /// Response metadata object.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::ProcessSummariesMeta>>,
}

impl ProcessSummariesResponse {
    pub fn new() -> ProcessSummariesResponse {
        ProcessSummariesResponse {
            data: None,
            meta: None,
        }
    }
}
impl Default for ProcessSummariesResponse {
    fn default() -> Self {
        Self::new()
    }
}
