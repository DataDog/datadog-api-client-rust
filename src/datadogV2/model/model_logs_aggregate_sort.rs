// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A sort rule
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsAggregateSort {
    /// An aggregation function
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::LogsAggregationFunction>,
    /// The metric to sort by (only used for `type=measure`)
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The order to use, ascending or descending
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::LogsSortOrder>,
    /// The type of sorting algorithm
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::LogsAggregateSortType>,
}

impl LogsAggregateSort {
    pub fn new() -> LogsAggregateSort {
        LogsAggregateSort {
            aggregation: None,
            metric: None,
            order: None,
            type_: None,
        }
    }

    pub fn aggregation(
        &mut self,
        value: crate::datadogV2::model::LogsAggregationFunction,
    ) -> &mut Self {
        self.aggregation = Some(value);
        self
    }

    pub fn metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    pub fn order(&mut self, value: crate::datadogV2::model::LogsSortOrder) -> &mut Self {
        self.order = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::LogsAggregateSortType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for LogsAggregateSort {
    fn default() -> Self {
        Self::new()
    }
}
