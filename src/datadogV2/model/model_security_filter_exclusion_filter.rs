// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Exclusion filter for the security filter.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilterExclusionFilter {
    /// Exclusion filter name.
    #[serde(rename = "name")]
    pub name: String,
    /// Exclusion filter query. Logs that match this query are excluded from the security filter.
    #[serde(rename = "query")]
    pub query: String,
}

impl SecurityFilterExclusionFilter {
    pub fn new(name: String, query: String) -> SecurityFilterExclusionFilter {
        SecurityFilterExclusionFilter { name, query }
    }
}
