// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Notebooks get all response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebooksResponse {
    /// List of notebook definitions.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::NotebooksResponseData>>,
    /// Searches metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV1::model::NotebooksResponseMeta>>,
}

impl NotebooksResponse {
    pub fn new() -> NotebooksResponse {
        NotebooksResponse {
            data: None,
            meta: None,
        }
    }
}
