// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CheckCanDeleteMonitorParams is a struct for passing parameters to the method [`CheckCanDeleteMonitor`]
#[derive(Clone, Debug)]
pub struct CheckCanDeleteMonitorParams {
    /* The IDs of the monitor to check. */
    pub monitor_ids: Vec<i64>,
}

// CreateMonitorParams is a struct for passing parameters to the method [`CreateMonitor`]
#[derive(Clone, Debug)]
pub struct CreateMonitorParams {
    /* Create a monitor request body. */
    pub body: Monitor,
}

// DeleteMonitorParams is a struct for passing parameters to the method [`DeleteMonitor`]
#[derive(Clone, Debug)]
pub struct DeleteMonitorParams {
    /* The ID of the monitor. */
    pub monitor_id: i64,
    /* Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor). */
    pub force: String,
}

// GetMonitorParams is a struct for passing parameters to the method [`GetMonitor`]
#[derive(Clone, Debug)]
pub struct GetMonitorParams {
    /* The ID of the monitor */
    pub monitor_id: i64,
    /* When specified, shows additional information about the group states. Choose one or more from `all`, `alert`, `warn`, and `no data`. */
    pub group_states: String,
    /* If this argument is set to true, then the returned data includes all current active downtimes for the monitor. */
    pub with_downtimes: bool,
}

// ListMonitorsParams is a struct for passing parameters to the method [`ListMonitors`]
#[derive(Clone, Debug)]
pub struct ListMonitorsParams {
    /* When specified, shows additional information about the group states.
Choose one or more from `all`, `alert`, `warn`, and `no data`. */
    pub group_states: String,
    /* A string to filter monitors by name. */
    pub name: String,
    /* A comma separated list indicating what tags, if any, should be used to filter the list of monitors by scope.
For example, `host:host0`. */
    pub tags: String,
    /* A comma separated list indicating what service and/or custom tags, if any, should be used to filter the list of monitors.
Tags created in the Datadog UI automatically have the service key prepended. For example, `service:my-app`. */
    pub monitor_tags: String,
    /* If this argument is set to true, then the returned data includes all current active downtimes for each monitor. */
    pub with_downtimes: bool,
    /* Use this parameter for paginating through large sets of monitors. Start with a value of zero, make a request, set the value to the last ID of result set, and then repeat until the response is empty. */
    pub id_offset: i64,
    /* The page to start paginating from. If this argument is not specified, the request returns all monitors without pagination. */
    pub page: i64,
    /* The number of monitors to return per page. If the page argument is not specified, the default behavior returns all monitors without a `page_size` limit. However, if page is specified and `page_size` is not, the argument defaults to 100. */
    pub page_size: i32,
}

// SearchMonitorGroupsParams is a struct for passing parameters to the method [`SearchMonitorGroups`]
#[derive(Clone, Debug)]
pub struct SearchMonitorGroupsParams {
    /* After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
page to learn more.

The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.

[1]: https://app.datadoghq.com/monitors/manage
[2]: /monitors/manage/#find-the-monitors */
    pub query: String,
    /* Page to start paginating from. */
    pub page: i64,
    /* Number of monitors to return per page. */
    pub per_page: i64,
    /* String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:

* `name`
* `status`
* `tags` */
    pub sort: String,
}

// SearchMonitorsParams is a struct for passing parameters to the method [`SearchMonitors`]
#[derive(Clone, Debug)]
pub struct SearchMonitorsParams {
    /* After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
page to learn more.

The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.

[1]: https://app.datadoghq.com/monitors/manage
[2]: /monitors/manage/#find-the-monitors */
    pub query: String,
    /* Page to start paginating from. */
    pub page: i64,
    /* Number of monitors to return per page. */
    pub per_page: i64,
    /* String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:

* `name`
* `status`
* `tags` */
    pub sort: String,
}

// UpdateMonitorParams is a struct for passing parameters to the method [`UpdateMonitor`]
#[derive(Clone, Debug)]
pub struct UpdateMonitorParams {
    /* The ID of the monitor. */
    pub monitor_id: i64,
    /* Edit a monitor request body. */
    pub body: MonitorUpdateRequest,
}

// ValidateExistingMonitorParams is a struct for passing parameters to the method [`ValidateExistingMonitor`]
#[derive(Clone, Debug)]
pub struct ValidateExistingMonitorParams {
    /* The ID of the monitor */
    pub monitor_id: i64,
    /* Monitor request object */
    pub body: Monitor,
}

// ValidateMonitorParams is a struct for passing parameters to the method [`ValidateMonitor`]
#[derive(Clone, Debug)]
pub struct ValidateMonitorParams {
    /* Monitor request object */
    pub body: Monitor,
}




/// CheckCanDeleteMonitorError is a struct for typed errors of method [`CheckCanDeleteMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCanDeleteMonitorError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    Status409(CheckCanDeleteMonitorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateMonitorError is a struct for typed errors of method [`CreateMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMonitorError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteMonitorError is a struct for typed errors of method [`DeleteMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMonitorError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetMonitorError is a struct for typed errors of method [`GetMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonitorError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListMonitorsError is a struct for typed errors of method [`ListMonitors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchMonitorGroupsError is a struct for typed errors of method [`SearchMonitorGroups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchMonitorGroupsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SearchMonitorsError is a struct for typed errors of method [`SearchMonitors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchMonitorsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateMonitorError is a struct for typed errors of method [`UpdateMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMonitorError {
    Status400(APIErrorResponse),
    Status401(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateExistingMonitorError is a struct for typed errors of method [`ValidateExistingMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateExistingMonitorError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ValidateMonitorError is a struct for typed errors of method [`ValidateMonitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateMonitorError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}