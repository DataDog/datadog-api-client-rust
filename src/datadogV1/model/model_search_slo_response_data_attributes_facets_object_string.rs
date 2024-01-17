// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseDataAttributesFacetsObjectString {
    /// Count
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Facet
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl SearchSLOResponseDataAttributesFacetsObjectString {
    pub fn new() -> SearchSLOResponseDataAttributesFacetsObjectString {
        SearchSLOResponseDataAttributesFacetsObjectString {
            count: None,
            name: None,
        }
    }
}
impl Default for SearchSLOResponseDataAttributesFacetsObjectString {
    fn default() -> Self {
        Self::new()
    }
}
