// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The search and filter query settings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsQueryFilter {
    /// The minimum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The search query following the event search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl EventsQueryFilter {
    pub fn new() -> EventsQueryFilter {
        EventsQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }

    pub fn from(mut self, value: String) -> Self {
        self.from = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn to(mut self, value: String) -> Self {
        self.to = Some(value);
        self
    }
}

impl Default for EventsQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}
