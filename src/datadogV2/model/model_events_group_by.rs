// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A dimension on which to split a query's results.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsGroupBy {
    /// The facet by which to split groups.
    #[serde(rename = "facet")]
    pub facet: String,
    /// The maximum number of groups to return.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The dimension by which to sort a query's results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::EventsGroupBySort>,
}

impl EventsGroupBy {
    pub fn new(facet: String) -> EventsGroupBy {
        EventsGroupBy {
            facet,
            limit: None,
            sort: None,
        }
    }

    pub fn limit(&mut self, value: i32) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(&mut self, value: crate::datadogV2::model::EventsGroupBySort) -> &mut Self {
        self.sort = Some(value);
        self
    }
}
