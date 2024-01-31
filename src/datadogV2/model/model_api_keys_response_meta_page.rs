// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Additional information related to the API keys response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct APIKeysResponseMetaPage {
    /// Total filtered application key count.
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
}

impl APIKeysResponseMetaPage {
    pub fn new() -> APIKeysResponseMetaPage {
        APIKeysResponseMetaPage {
            total_filtered_count: None,
        }
    }

    pub fn with_total_filtered_count(&mut self, value: i64) -> &mut Self {
        self.total_filtered_count = Some(value);
        self
    }
}
impl Default for APIKeysResponseMetaPage {
    fn default() -> Self {
        Self::new()
    }
}
