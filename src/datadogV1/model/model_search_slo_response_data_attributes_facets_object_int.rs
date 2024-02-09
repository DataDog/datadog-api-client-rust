// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseDataAttributesFacetsObjectInt {
    /// Count
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// Facet
    #[serde(rename = "name")]
    pub name: Option<f64>,
}

impl SearchSLOResponseDataAttributesFacetsObjectInt {
    pub fn new() -> SearchSLOResponseDataAttributesFacetsObjectInt {
        SearchSLOResponseDataAttributesFacetsObjectInt {
            count: None,
            name: None,
        }
    }

    pub fn count(&mut self, value: i64) -> &mut Self {
        self.count = Some(value);
        self
    }

    pub fn name(&mut self, value: f64) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for SearchSLOResponseDataAttributesFacetsObjectInt {
    fn default() -> Self {
        Self::new()
    }
}
