// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventsListRequest {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::EventsQueryFilter>>,
    /// The global query options that are used. Either provide a timezone or a time offset but not both,
    /// otherwise the query fails.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::EventsQueryOptions>>,
    /// Pagination settings.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::EventsRequestPage>>,
    /// The sort parameters when querying events.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::EventsSort>,
}

impl EventsListRequest {
    /// The object sent with the request to retrieve a list of events from your organization.
    pub fn new() -> EventsListRequest {
        EventsListRequest {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }
}
