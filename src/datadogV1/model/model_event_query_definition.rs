// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The event query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventQueryDefinition {
    /// The query being made on the event.
    #[serde(rename = "search")]
    pub search: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution")]
    pub tags_execution: String,
}

impl EventQueryDefinition {
    pub fn new(search: String, tags_execution: String) -> EventQueryDefinition {
        EventQueryDefinition {
            search,
            tags_execution,
        }
    }
}
