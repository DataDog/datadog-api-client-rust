// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The description of a notebook response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookResponse {
    /// The data for a notebook.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::NotebookResponseData>,
}

impl NotebookResponse {
    pub fn new() -> NotebookResponse {
        NotebookResponse { data: None }
    }

    pub fn data(&mut self, value: crate::datadogV1::model::NotebookResponseData) -> &mut Self {
        self.data = Some(value);
        self
    }
}

impl Default for NotebookResponse {
    fn default() -> Self {
        Self::new()
    }
}
