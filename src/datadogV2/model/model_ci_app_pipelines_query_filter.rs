// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The search and filter query settings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelinesQueryFilter {
    /// The minimum time for the requested events; supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// The search query following the CI Visibility Explorer search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The maximum time for the requested events, supports date, math, and regular timestamps (in milliseconds).
    #[serde(rename = "to")]
    pub to: Option<String>,
}

impl CIAppPipelinesQueryFilter {
    pub fn new() -> CIAppPipelinesQueryFilter {
        CIAppPipelinesQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }

    pub fn from(&mut self, value: String) -> &mut Self {
        self.from = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn to(&mut self, value: String) -> &mut Self {
        self.to = Some(value);
        self
    }
}

impl Default for CIAppPipelinesQueryFilter {
    fn default() -> Self {
        Self::new()
    }
}
