// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseAttributes {
    /// Facet data for incidents returned by a search query.
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: IncidentSearchResponseFacetsData,
    /// Incidents returned by the search.
    #[serde(rename = "incidents", skip_serializing_if = "Option::is_none")]
    pub incidents: Vec<IncidentSearchResponseIncidentsData>,
    /// Number of incidents returned by the search.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: i32,
}

