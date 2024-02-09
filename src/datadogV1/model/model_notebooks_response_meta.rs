// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Searches metadata returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponseMeta {
    /// Pagination metadata returned by the API.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV1::model::NotebooksResponsePage>,
}

impl NotebooksResponseMeta {
    pub fn new() -> NotebooksResponseMeta {
        NotebooksResponseMeta { page: None }
    }

    pub fn page(&mut self, value: crate::datadogV1::model::NotebooksResponsePage) -> &mut Self {
        self.page = Some(value);
        self
    }
}

impl Default for NotebooksResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
