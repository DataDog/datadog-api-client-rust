// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Searches metadata returned by the API.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseMeta {
    /// Pagination metadata returned by the API.
    #[serde(rename = "pagination")]
    pub pagination: Option<Box<crate::datadogV1::model::SearchSLOResponseMetaPage>>,
}

impl SearchSLOResponseMeta {
    pub fn new() -> SearchSLOResponseMeta {
        SearchSLOResponseMeta { pagination: None }
    }
}
impl Default for SearchSLOResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
