// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Pagination attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResponseMetadataPage {
    /// The cursor to use to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after")]
    pub after: Option<String>,
}

impl EventsResponseMetadataPage {
    pub fn new() -> EventsResponseMetadataPage {
        EventsResponseMetadataPage { after: None }
    }
}
impl Default for EventsResponseMetadataPage {
    fn default() -> Self {
        Self::new()
    }
}
