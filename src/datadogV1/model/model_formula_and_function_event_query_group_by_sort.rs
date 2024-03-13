// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options for sorting group by results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionEventQueryGroupBySort {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// Metric used for sorting group by results.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::QuerySortOrder>,
}

impl FormulaAndFunctionEventQueryGroupBySort {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> FormulaAndFunctionEventQueryGroupBySort {
        FormulaAndFunctionEventQueryGroupBySort {
            aggregation,
            metric: None,
            order: None,
        }
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV1::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }
}
