// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsResponseMetadataPage {
    /// The cursor to use to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after")]
    pub after: Option<String>,
}

impl LogsResponseMetadataPage {
    pub fn new() -> LogsResponseMetadataPage {
        LogsResponseMetadataPage { after: None }
    }

    pub fn after(&mut self, value: String) -> &mut Self {
        self.after = Some(value);
        self
    }
}

impl Default for LogsResponseMetadataPage {
    fn default() -> Self {
        Self::new()
    }
}
