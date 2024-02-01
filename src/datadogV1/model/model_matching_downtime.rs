// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing a downtime that matches this monitor.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchingDowntime {
    /// POSIX timestamp to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<i64>>,
    /// The downtime ID.
    #[serde(rename = "id")]
    pub id: i64,
    /// The scope(s) to which the downtime applies. Must be in `key:value` format. For example, `host:app2`.
    /// Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope")]
    pub scope: Option<Vec<String>>,
    /// POSIX timestamp to start the downtime.
    #[serde(rename = "start")]
    pub start: Option<i64>,
}

impl MatchingDowntime {
    pub fn new(id: i64) -> MatchingDowntime {
        MatchingDowntime {
            end: None,
            id,
            scope: None,
            start: None,
        }
    }

    pub fn end(&mut self, value: Option<i64>) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn scope(&mut self, value: Vec<String>) -> &mut Self {
        self.scope = Some(value);
        self
    }

    pub fn start(&mut self, value: i64) -> &mut Self {
        self.start = Some(value);
        self
    }
}
