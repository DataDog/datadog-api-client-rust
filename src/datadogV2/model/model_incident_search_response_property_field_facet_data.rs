// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet data for the incident property fields.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponsePropertyFieldFacetData {
    /// Aggregate information for numeric incident data.
    #[serde(rename = "aggregates")]
    pub aggregates:
        Option<crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates>,
    /// Facet data for the property field of an incident.
    #[serde(rename = "facets")]
    pub facets: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
    /// Name of the incident property field.
    #[serde(rename = "name")]
    pub name: String,
}

impl IncidentSearchResponsePropertyFieldFacetData {
    pub fn new(
        facets: Vec<crate::datadogV2::model::IncidentSearchResponseFieldFacetData>,
        name: String,
    ) -> IncidentSearchResponsePropertyFieldFacetData {
        IncidentSearchResponsePropertyFieldFacetData {
            aggregates: None,
            facets,
            name,
        }
    }

    pub fn aggregates(
        &mut self,
        value: crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates,
    ) -> &mut Self {
        self.aggregates = Some(value);
        self
    }
}
