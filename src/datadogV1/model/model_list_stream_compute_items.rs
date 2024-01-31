// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of facets and aggregations which to compute.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamComputeItems {
    /// Aggregation value.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::ListStreamComputeAggregation,
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
}

impl ListStreamComputeItems {
    pub fn new(
        aggregation: crate::datadogV1::model::ListStreamComputeAggregation,
    ) -> ListStreamComputeItems {
        ListStreamComputeItems {
            aggregation,
            facet: None,
        }
    }

    pub fn with_facet(&mut self, value: String) -> &mut Self {
        self.facet = Some(value);
        self
    }
}
