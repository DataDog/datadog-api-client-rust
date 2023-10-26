// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventsQueryFilter {
    /// The minimum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The search query following the event search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The maximum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl EventsQueryFilter {
    /// The search and filter query settings.
    pub fn new() -> EventsQueryFilter {
        EventsQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}
