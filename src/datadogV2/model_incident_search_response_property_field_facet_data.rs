// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponsePropertyFieldFacetData {
    /// Aggregate information for numeric incident data.
    #[serde(rename = "aggregates", skip_serializing_if = "Option::is_none")]
    pub aggregates: IncidentSearchResponseNumericFacetDataAggregates,
    /// Facet data for the property field of an incident.
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: Vec<IncidentSearchResponseFieldFacetData>,
    /// Name of the incident property field.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
}

