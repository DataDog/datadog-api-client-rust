// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Holds search results.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorSearchResult {
    /// Classification of the monitor.
    #[serde(rename = "classification")]
    pub classification: Option<String>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<Box<crate::datadogV1::model::Creator>>,
    /// ID of the monitor.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Latest timestamp the monitor triggered.
    #[serde(
        rename = "last_triggered_ts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_triggered_ts: Option<Option<i64>>,
    /// Metrics used by the monitor.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// The monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The notification triggered by the monitor.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<crate::datadogV1::model::MonitorSearchResultNotification>>,
    /// The ID of the organization.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// The monitor query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The scope(s) to which the downtime applies, for example `host:app2`.
    /// Provide multiple scopes as a comma-separated list, for example `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes
    /// (that is `env:dev AND env:prod`), NOT any of them.
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
    /// Tags associated with the monitor.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the monitor. For more information about `type`, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::MonitorType>,
}

impl MonitorSearchResult {
    pub fn new() -> MonitorSearchResult {
        MonitorSearchResult {
            classification: None,
            creator: None,
            id: None,
            last_triggered_ts: None,
            metrics: None,
            name: None,
            notifications: None,
            org_id: None,
            query: None,
            scopes: None,
            status: None,
            tags: None,
            type_: None,
        }
    }
}
