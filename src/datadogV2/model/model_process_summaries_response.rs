// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesResponse {
    /// Array of process summary objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::datadogV2::model::ProcessSummary>>,
    /// Response metadata object.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::datadogV2::model::ProcessSummariesMeta>>,
}

impl ProcessSummariesResponse {
    /// List of process summaries.
    pub fn new() -> ProcessSummariesResponse {
        ProcessSummariesResponse { data: None, meta: None }
    }
}
