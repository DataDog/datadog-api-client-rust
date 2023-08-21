// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseFacetsData {
    /// Facet data for incident commander users.
    #[serde(rename = "commander", skip_serializing_if = "Option::is_none")]
    pub commander: Vec<IncidentSearchResponseUserFacetData>,
    /// Facet data for incident creator users.
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Vec<IncidentSearchResponseUserFacetData>,
    /// Facet data for incident property fields.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Vec<IncidentSearchResponsePropertyFieldFacetData>,
    /// Facet data for incident impact attributes.
    #[serde(rename = "impact", skip_serializing_if = "Option::is_none")]
    pub impact: Vec<IncidentSearchResponseFieldFacetData>,
    /// Facet data for incident last modified by users.
    #[serde(rename = "last_modified_by", skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Vec<IncidentSearchResponseUserFacetData>,
    /// Facet data for incident postmortem existence.
    #[serde(rename = "postmortem", skip_serializing_if = "Option::is_none")]
    pub postmortem: Vec<IncidentSearchResponseFieldFacetData>,
    /// Facet data for incident responder users.
    #[serde(rename = "responder", skip_serializing_if = "Option::is_none")]
    pub responder: Vec<IncidentSearchResponseUserFacetData>,
    /// Facet data for incident severity attributes.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Vec<IncidentSearchResponseFieldFacetData>,
    /// Facet data for incident state attributes.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Vec<IncidentSearchResponseFieldFacetData>,
    /// Facet data for incident time to repair metrics.
    #[serde(rename = "time_to_repair", skip_serializing_if = "Option::is_none")]
    pub time_to_repair: Vec<IncidentSearchResponseNumericFacetData>,
    /// Facet data for incident time to resolve metrics.
    #[serde(rename = "time_to_resolve", skip_serializing_if = "Option::is_none")]
    pub time_to_resolve: Vec<IncidentSearchResponseNumericFacetData>,
}

