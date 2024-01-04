// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single exclusion filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterExclusionFilterResponse {
    /// The exclusion filter name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The exclusion filter query.
    #[serde(rename = "query")]
    pub query: Option<String>,
}

impl SecurityFilterExclusionFilterResponse {
    pub fn new() -> SecurityFilterExclusionFilterResponse {
        SecurityFilterExclusionFilterResponse {
            name: None,
            query: None,
        }
    }
}
