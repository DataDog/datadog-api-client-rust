// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging attributes for listing events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppQueryPageOptions {
    /// List following results with a cursor provided in the previous query.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Maximum number of events in the response.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
}

impl CIAppQueryPageOptions {
    pub fn new() -> CIAppQueryPageOptions {
        CIAppQueryPageOptions {
            cursor: None,
            limit: None,
        }
    }

    pub fn cursor(&mut self, value: String) -> &mut Self {
        self.cursor = Some(value);
        self
    }

    pub fn limit(&mut self, value: i32) -> &mut Self {
        self.limit = Some(value);
        self
    }
}

impl Default for CIAppQueryPageOptions {
    fn default() -> Self {
        Self::new()
    }
}
