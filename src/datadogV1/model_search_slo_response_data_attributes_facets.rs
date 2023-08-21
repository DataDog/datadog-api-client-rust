// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseDataAttributesFacets {
    /// All tags associated with an SLO.
    #[serde(rename = "all_tags", skip_serializing_if = "Option::is_none")]
    pub all_tags: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
    /// Creator of an SLO.
    #[serde(rename = "creator_name", skip_serializing_if = "Option::is_none")]
    pub creator_name: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
    /// Tags with the `env` tag key.
    #[serde(rename = "env_tags", skip_serializing_if = "Option::is_none")]
    pub env_tags: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
    /// Tags with the `service` tag key.
    #[serde(rename = "service_tags", skip_serializing_if = "Option::is_none")]
    pub service_tags: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
    /// Type of SLO.
    #[serde(rename = "slo_type", skip_serializing_if = "Option::is_none")]
    pub slo_type: Vec<SearchSLOResponseDataAttributesFacetsObjectInt>,
    /// SLO Target
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Vec<SearchSLOResponseDataAttributesFacetsObjectInt>,
    /// Tags with the `team` tag key.
    #[serde(rename = "team_tags", skip_serializing_if = "Option::is_none")]
    pub team_tags: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
    /// Timeframes of SLOs.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: Vec<SearchSLOResponseDataAttributesFacetsObjectString>,
}

