// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An individual timeseries events query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsTimeseriesQuery {
    /// The instructions for what to compute for this query.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::EventsCompute,
    /// A data source that is powered by the Events Platform.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::EventsDataSource,
    /// The list of facets on which to split results.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::EventsGroupBy>>,
    /// The indexes in which to search.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Configuration of the search/filter for an events query.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::EventsSearch>,
}

impl EventsTimeseriesQuery {
    pub fn new(
        compute: crate::datadogV2::model::EventsCompute,
        data_source: crate::datadogV2::model::EventsDataSource,
    ) -> EventsTimeseriesQuery {
        EventsTimeseriesQuery {
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name: None,
            search: None,
        }
    }

    pub fn group_by(&mut self, value: Vec<crate::datadogV2::model::EventsGroupBy>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(&mut self, value: Vec<String>) -> &mut Self {
        self.indexes = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn search(&mut self, value: crate::datadogV2::model::EventsSearch) -> &mut Self {
        self.search = Some(value);
        self
    }
}
