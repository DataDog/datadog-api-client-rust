// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Facet data numeric attributes of an incident.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseNumericFacetData {
    /// Aggregate information for numeric incident data.
    #[serde(rename = "aggregates")]
    pub aggregates: Box<crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates>,
    /// Name of the incident property field.
    #[serde(rename = "name")]
    pub name: String,
}

impl IncidentSearchResponseNumericFacetData {
    pub fn new(
        aggregates: Box<crate::datadogV2::model::IncidentSearchResponseNumericFacetDataAggregates>,
        name: String,
    ) -> IncidentSearchResponseNumericFacetData {
        IncidentSearchResponseNumericFacetData { aggregates, name }
    }
}
