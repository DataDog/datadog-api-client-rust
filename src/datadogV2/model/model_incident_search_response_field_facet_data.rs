// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet value and number of occurrences for a property field of an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseFieldFacetData {
    /// Count of the facet value appearing in search results.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// The facet value appearing in search results.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl IncidentSearchResponseFieldFacetData {
    pub fn new() -> IncidentSearchResponseFieldFacetData {
        IncidentSearchResponseFieldFacetData {
            count: None,
            name: None,
        }
    }

    pub fn count(&mut self, value: i32) -> &mut Self {
        self.count = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for IncidentSearchResponseFieldFacetData {
    fn default() -> Self {
        Self::new()
    }
}
