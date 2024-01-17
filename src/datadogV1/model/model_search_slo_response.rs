// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A search SLO response containing results from the search query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponse {
    /// Data from search SLO response.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SearchSLOResponseData>>,
    /// Pagination links.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV1::model::SearchSLOResponseLinks>>,
    /// Searches metadata returned by the API.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV1::model::SearchSLOResponseMeta>>,
}

impl SearchSLOResponse {
    pub fn new() -> SearchSLOResponse {
        SearchSLOResponse {
            data: None,
            links: None,
            meta: None,
        }
    }
}
