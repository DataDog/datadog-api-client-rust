// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes returned by an incident search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseAttributes {
    /// Facet data for incidents returned by a search query.
    #[serde(rename = "facets")]
    pub facets: crate::datadogV2::model::IncidentSearchResponseFacetsData,
    /// Incidents returned by the search.
    #[serde(rename = "incidents")]
    pub incidents: Vec<crate::datadogV2::model::IncidentSearchResponseIncidentsData>,
    /// Number of incidents returned by the search.
    #[serde(rename = "total")]
    pub total: i32,
}

impl IncidentSearchResponseAttributes {
    pub fn new(
        facets: crate::datadogV2::model::IncidentSearchResponseFacetsData,
        incidents: Vec<crate::datadogV2::model::IncidentSearchResponseIncidentsData>,
        total: i32,
    ) -> IncidentSearchResponseAttributes {
        IncidentSearchResponseAttributes {
            facets,
            incidents,
            total,
        }
    }
}
