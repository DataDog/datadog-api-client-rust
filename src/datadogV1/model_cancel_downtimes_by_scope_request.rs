// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelDowntimesByScopeRequest {
    /// The scope(s) to which the downtime applies and must be in `key:value` format. For example, `host:app2`.
Provide multiple scopes as a comma-separated list like `env:dev,env:prod`.
The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: String,
}

