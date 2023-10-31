// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facets
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseDataAttributesFacets {
    /// All tags associated with an SLO.
    #[serde(rename = "all_tags")]
    pub all_tags: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Creator of an SLO.
    #[serde(rename = "creator_name")]
    pub creator_name: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Tags with the `env` tag key.
    #[serde(rename = "env_tags")]
    pub env_tags: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Tags with the `service` tag key.
    #[serde(rename = "service_tags")]
    pub service_tags: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Type of SLO.
    #[serde(rename = "slo_type")]
    pub slo_type: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>>,
    /// SLO Target
    #[serde(rename = "target")]
    pub target: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectInt>>,
    /// Tags with the `team` tag key.
    #[serde(rename = "team_tags")]
    pub team_tags: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
    /// Timeframes of SLOs.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<Vec<crate::datadogV1::model::SearchSLOResponseDataAttributesFacetsObjectString>>,
}

impl SearchSLOResponseDataAttributesFacets {
    pub fn new() -> SearchSLOResponseDataAttributesFacets {
        SearchSLOResponseDataAttributesFacets {
            all_tags: None,
            creator_name: None,
            env_tags: None,
            service_tags: None,
            slo_type: None,
            target: None,
            team_tags: None,
            timeframe: None,
        }
    }
}
