// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The dimension by which to sort a query's results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsGroupBySort {
    /// The type of aggregation that can be performed on events-based queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV2::model::EventsAggregation,
    /// The metric's calculated value which should be used to define the sort order of a query's results.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    /// Direction of sort.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV2::model::QuerySortOrder>,
    /// The type of sort to use on the calculated value.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::EventsSortType>,
}

impl EventsGroupBySort {
    pub fn new(aggregation: crate::datadogV2::model::EventsAggregation) -> EventsGroupBySort {
        EventsGroupBySort {
            aggregation,
            metric: None,
            order: None,
            type_: None,
        }
    }

    pub fn metric(&mut self, value: String) -> &mut Self {
        self.metric = Some(value);
        self
    }

    pub fn order(&mut self, value: crate::datadogV2::model::QuerySortOrder) -> &mut Self {
        self.order = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::EventsSortType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
