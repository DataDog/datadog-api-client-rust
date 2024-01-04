// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseDataAttributes {
    /// Facets
    #[serde(rename = "facets")]
    pub facets: Option<Box<crate::datadogV1::model::SearchSLOResponseDataAttributesFacets>>,
    /// SLOs
    #[serde(rename = "slos")]
    pub slos: Option<Vec<crate::datadogV1::model::SearchServiceLevelObjective>>,
}

impl SearchSLOResponseDataAttributes {
    pub fn new() -> SearchSLOResponseDataAttributes {
        SearchSLOResponseDataAttributes {
            facets: None,
            slos: None,
        }
    }
}
