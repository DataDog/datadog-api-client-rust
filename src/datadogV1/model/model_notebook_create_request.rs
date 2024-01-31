// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The description of a notebook create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookCreateRequest {
    /// The data for a notebook create request.
    #[serde(rename = "data")]
    pub data: crate::datadogV1::model::NotebookCreateData,
}

impl NotebookCreateRequest {
    pub fn new(data: crate::datadogV1::model::NotebookCreateData) -> NotebookCreateRequest {
        NotebookCreateRequest { data }
    }
}
