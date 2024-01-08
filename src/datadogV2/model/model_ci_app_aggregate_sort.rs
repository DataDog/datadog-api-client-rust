// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A sort rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppAggregateSort {
    /// An aggregation function.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::CIAppAggregationFunction>,
    /// The metric to sort by (only used for `type=measure`).
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// The order to use, ascending or descending.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::CIAppSortOrder>,
    /// The type of sorting algorithm.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CIAppAggregateSortType>,
}

impl CIAppAggregateSort {
    pub fn new() -> CIAppAggregateSort {
        CIAppAggregateSort {
            aggregation: None,
            metric: None,
            order: None,
            type_: None,
        }
    }
}
impl Default for CIAppAggregateSort {
    fn default() -> Self {
        Self::new()
    }
}
