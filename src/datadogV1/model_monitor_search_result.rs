// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResult {
    /// Classification of the monitor.
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: String,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Creator,
    /// ID of the monitor.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// Latest timestamp the monitor triggered.
    #[serde(rename = "last_triggered_ts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub last_triggered_ts: Option<Int64>,
    /// Metrics used by the monitor.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Vec<String>,
    /// The monitor name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The notification triggered by the monitor.
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Vec<MonitorSearchResultNotification>,
    /// The ID of the organization.
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: i64,
    /// The monitor query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The scope(s) to which the downtime applies, for example `host:app2`.
Provide multiple scopes as a comma-separated list, for example `env:dev,env:prod`.
The resulting downtime applies to sources that matches ALL provided scopes
(that is `env:dev AND env:prod`), NOT any of them.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Vec<String>,
    /// The different states your monitor can be in.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: MonitorOverallStates,
    /// Tags associated with the monitor.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The type of the monitor. For more information about `type`, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: MonitorType,
}

