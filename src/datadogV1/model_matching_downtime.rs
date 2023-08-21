// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchingDowntime {
    /// POSIX timestamp to end the downtime.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub end: Option<Int64>,
    /// The downtime ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// The scope(s) to which the downtime applies. Must be in `key:value` format. For example, `host:app2`.
Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Vec<String>,
    /// POSIX timestamp to start the downtime.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: i64,
}

