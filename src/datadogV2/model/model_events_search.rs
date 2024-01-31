// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration of the search/filter for an events query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsSearch {
    /// The search/filter string for an events query.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl EventsSearch {
    pub fn new() -> EventsSearch {
        EventsSearch { query: None }
    }

    pub fn with_query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}
impl Default for EventsSearch {
    fn default() -> Self {
        Self::new()
    }
}
