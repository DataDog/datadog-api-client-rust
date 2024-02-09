// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Aggregate information for numeric incident data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseNumericFacetDataAggregates {
    /// Maximum value of the numeric aggregates.
    #[serde(rename = "max", default, with = "::serde_with::rust::double_option")]
    pub max: Option<Option<f64>>,
    /// Minimum value of the numeric aggregates.
    #[serde(rename = "min", default, with = "::serde_with::rust::double_option")]
    pub min: Option<Option<f64>>,
}

impl IncidentSearchResponseNumericFacetDataAggregates {
    pub fn new() -> IncidentSearchResponseNumericFacetDataAggregates {
        IncidentSearchResponseNumericFacetDataAggregates {
            max: None,
            min: None,
        }
    }

    pub fn max(&mut self, value: Option<f64>) -> &mut Self {
        self.max = Some(value);
        self
    }

    pub fn min(&mut self, value: Option<f64>) -> &mut Self {
        self.min = Some(value);
        self
    }
}

impl Default for IncidentSearchResponseNumericFacetDataAggregates {
    fn default() -> Self {
        Self::new()
    }
}
