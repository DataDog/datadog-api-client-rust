// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The description of a notebook update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookUpdateRequest {
    /// The data for a notebook update request.
    #[serde(rename = "data")]
    pub data: crate::datadogV1::model::NotebookUpdateData,
}

impl NotebookUpdateRequest {
    pub fn new(data: crate::datadogV1::model::NotebookUpdateData) -> NotebookUpdateRequest {
        NotebookUpdateRequest { data }
    }
}
