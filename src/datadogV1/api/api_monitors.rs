// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// DeleteMonitorOptionalParams is a struct for passing parameters to the method [`MonitorsAPI::delete_monitor`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct DeleteMonitorOptionalParams {
    /// Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor).
    pub force: Option<String>,
}

impl DeleteMonitorOptionalParams {
    /// Delete the monitor even if it's referenced by other resources (for example SLO, composite monitor).
    pub fn force(mut self, value: String) -> Self {
        self.force = Some(value);
        self
    }
}

/// GetMonitorOptionalParams is a struct for passing parameters to the method [`MonitorsAPI::get_monitor`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetMonitorOptionalParams {
    /// When specified, shows additional information about the group states. Choose one or more from `all`, `alert`, `warn`, and `no data`.
    pub group_states: Option<String>,
    /// If this argument is set to true, then the returned data includes all current active downtimes for the monitor.
    pub with_downtimes: Option<bool>,
}

impl GetMonitorOptionalParams {
    /// When specified, shows additional information about the group states. Choose one or more from `all`, `alert`, `warn`, and `no data`.
    pub fn group_states(mut self, value: String) -> Self {
        self.group_states = Some(value);
        self
    }
    /// If this argument is set to true, then the returned data includes all current active downtimes for the monitor.
    pub fn with_downtimes(mut self, value: bool) -> Self {
        self.with_downtimes = Some(value);
        self
    }
}

/// ListMonitorsOptionalParams is a struct for passing parameters to the method [`MonitorsAPI::list_monitors`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListMonitorsOptionalParams {
    /// When specified, shows additional information about the group states.
    /// Choose one or more from `all`, `alert`, `warn`, and `no data`.
    pub group_states: Option<String>,
    /// A string to filter monitors by name.
    pub name: Option<String>,
    /// A comma separated list indicating what tags, if any, should be used to filter the list of monitors by scope.
    /// For example, `host:host0`.
    pub tags: Option<String>,
    /// A comma separated list indicating what service and/or custom tags, if any, should be used to filter the list of monitors.
    /// Tags created in the Datadog UI automatically have the service key prepended. For example, `service:my-app`.
    pub monitor_tags: Option<String>,
    /// If this argument is set to true, then the returned data includes all current active downtimes for each monitor.
    pub with_downtimes: Option<bool>,
    /// Use this parameter for paginating through large sets of monitors. Start with a value of zero, make a request, set the value to the last ID of result set, and then repeat until the response is empty.
    pub id_offset: Option<i64>,
    /// The page to start paginating from. If this argument is not specified, the request returns all monitors without pagination.
    pub page: Option<i64>,
    /// The number of monitors to return per page. If the page argument is not specified, the default behavior returns all monitors without a `page_size` limit. However, if page is specified and `page_size` is not, the argument defaults to 100.
    pub page_size: Option<i32>,
}

impl ListMonitorsOptionalParams {
    /// When specified, shows additional information about the group states.
    /// Choose one or more from `all`, `alert`, `warn`, and `no data`.
    pub fn group_states(mut self, value: String) -> Self {
        self.group_states = Some(value);
        self
    }
    /// A string to filter monitors by name.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    /// A comma separated list indicating what tags, if any, should be used to filter the list of monitors by scope.
    /// For example, `host:host0`.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// A comma separated list indicating what service and/or custom tags, if any, should be used to filter the list of monitors.
    /// Tags created in the Datadog UI automatically have the service key prepended. For example, `service:my-app`.
    pub fn monitor_tags(mut self, value: String) -> Self {
        self.monitor_tags = Some(value);
        self
    }
    /// If this argument is set to true, then the returned data includes all current active downtimes for each monitor.
    pub fn with_downtimes(mut self, value: bool) -> Self {
        self.with_downtimes = Some(value);
        self
    }
    /// Use this parameter for paginating through large sets of monitors. Start with a value of zero, make a request, set the value to the last ID of result set, and then repeat until the response is empty.
    pub fn id_offset(mut self, value: i64) -> Self {
        self.id_offset = Some(value);
        self
    }
    /// The page to start paginating from. If this argument is not specified, the request returns all monitors without pagination.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
    /// The number of monitors to return per page. If the page argument is not specified, the default behavior returns all monitors without a `page_size` limit. However, if page is specified and `page_size` is not, the argument defaults to 100.
    pub fn page_size(mut self, value: i32) -> Self {
        self.page_size = Some(value);
        self
    }
}

/// SearchMonitorGroupsOptionalParams is a struct for passing parameters to the method [`MonitorsAPI::search_monitor_groups`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchMonitorGroupsOptionalParams {
    /// After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
    /// URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
    /// page to learn more.
    ///
    /// The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.
    ///
    /// [1]: <https://app.datadoghq.com/monitors/manage>
    /// [2]: /monitors/manage/#find-the-monitors
    pub query: Option<String>,
    /// Page to start paginating from.
    pub page: Option<i64>,
    /// Number of monitors to return per page.
    pub per_page: Option<i64>,
    /// String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:
    ///
    /// * `name`
    /// * `status`
    /// * `tags`
    pub sort: Option<String>,
}

impl SearchMonitorGroupsOptionalParams {
    /// After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
    /// URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
    /// page to learn more.
    ///
    /// The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.
    ///
    /// [1]: <https://app.datadoghq.com/monitors/manage>
    /// [2]: /monitors/manage/#find-the-monitors
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// Page to start paginating from.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
    /// Number of monitors to return per page.
    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }
    /// String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:
    ///
    /// * `name`
    /// * `status`
    /// * `tags`
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// SearchMonitorsOptionalParams is a struct for passing parameters to the method [`MonitorsAPI::search_monitors`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct SearchMonitorsOptionalParams {
    /// After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
    /// URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
    /// page to learn more.
    ///
    /// The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.
    ///
    /// [1]: <https://app.datadoghq.com/monitors/manage>
    /// [2]: /monitors/manage/#find-the-monitors
    pub query: Option<String>,
    /// Page to start paginating from.
    pub page: Option<i64>,
    /// Number of monitors to return per page.
    pub per_page: Option<i64>,
    /// String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:
    ///
    /// * `name`
    /// * `status`
    /// * `tags`
    pub sort: Option<String>,
}

impl SearchMonitorsOptionalParams {
    /// After entering a search query in your [Manage Monitor page][1] use the query parameter value in the
    /// URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation][2]
    /// page to learn more.
    ///
    /// The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.
    ///
    /// [1]: <https://app.datadoghq.com/monitors/manage>
    /// [2]: /monitors/manage/#find-the-monitors
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// Page to start paginating from.
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }
    /// Number of monitors to return per page.
    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }
    /// String for sort order, composed of field and sort order separate by a comma, for example `name,asc`. Supported sort directions: `asc`, `desc`. Supported fields:
    ///
    /// * `name`
    /// * `status`
    /// * `tags`
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
}

/// CheckCanDeleteMonitorError is a struct for typed errors of method [`MonitorsAPI::check_can_delete_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCanDeleteMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::CheckCanDeleteMonitorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateMonitorError is a struct for typed errors of method [`MonitorsAPI::create_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteMonitorError is a struct for typed errors of method [`MonitorsAPI::delete_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status401(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetMonitorError is a struct for typed errors of method [`MonitorsAPI::get_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMonitorsError is a struct for typed errors of method [`MonitorsAPI::list_monitors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchMonitorGroupsError is a struct for typed errors of method [`MonitorsAPI::search_monitor_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchMonitorGroupsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SearchMonitorsError is a struct for typed errors of method [`MonitorsAPI::search_monitors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchMonitorsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateMonitorError is a struct for typed errors of method [`MonitorsAPI::update_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status401(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ValidateExistingMonitorError is a struct for typed errors of method [`MonitorsAPI::validate_existing_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateExistingMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ValidateMonitorError is a struct for typed errors of method [`MonitorsAPI::validate_monitor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidateMonitorError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct MonitorsAPI {
    config: configuration::Configuration,
}

impl Default for MonitorsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl MonitorsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Check if the given monitors can be deleted.
    pub async fn check_can_delete_monitor(
        &self,
        monitor_ids: Vec<i64>,
    ) -> Result<
        crate::datadogV1::model::CheckCanDeleteMonitorResponse,
        Error<CheckCanDeleteMonitorError>,
    > {
        match self
            .check_can_delete_monitor_with_http_info(monitor_ids)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Check if the given monitors can be deleted.
    pub async fn check_can_delete_monitor_with_http_info(
        &self,
        monitor_ids: Vec<i64>,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::CheckCanDeleteMonitorResponse>,
        Error<CheckCanDeleteMonitorError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.check_can_delete_monitor";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/can_delete",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[(
            "monitor_ids",
            &monitor_ids
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]);

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::CheckCanDeleteMonitorResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CheckCanDeleteMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a monitor using the specified options.
    ///
    /// #### Monitor Types
    ///
    /// The type of monitor chosen from:
    ///
    /// - anomaly: `query alert`
    /// - APM: `query alert` or `trace-analytics alert`
    /// - composite: `composite`
    /// - custom: `service check`
    /// - event: `event alert`
    /// - forecast: `query alert`
    /// - host: `service check`
    /// - integration: `query alert` or `service check`
    /// - live process: `process alert`
    /// - logs: `log alert`
    /// - metric: `query alert`
    /// - network: `service check`
    /// - outlier: `query alert`
    /// - process: `service check`
    /// - rum: `rum alert`
    /// - SLO: `slo alert`
    /// - watchdog: `event-v2 alert`
    /// - event-v2: `event-v2 alert`
    /// - audit: `audit alert`
    /// - error-tracking: `error-tracking alert`
    /// - database-monitoring: `database-monitoring alert`
    ///
    /// **Notes**:
    /// - Synthetic monitors are created through the Synthetics API. See the [Synthetics API] (<https://docs.datadoghq.com/api/latest/synthetics/>) documentation for more information.
    /// - Log monitors require an unscoped App Key.
    ///
    /// #### Query Types
    ///
    /// ##### Metric Alert Query
    ///
    /// Example: `time_aggr(time_window):space_aggr:metric{tags} [by {key}] operator #`
    ///
    /// - `time_aggr`: avg, sum, max, min, change, or pct_change
    /// - `time_window`: `last_#m` (with `#` between 1 and 10080 depending on the monitor type) or `last_#h`(with `#` between 1 and 168 depending on the monitor type) or `last_1d`, or `last_1w`
    /// - `space_aggr`: avg, sum, min, or max
    /// - `tags`: one or more tags (comma-separated), or *
    /// - `key`: a 'key' in key:value tag syntax; defines a separate alert for each tag in the group (multi-alert)
    /// - `operator`: <, <=, >, >=, ==, or !=
    /// - `#`: an integer or decimal number used to set the threshold
    ///
    /// If you are using the `_change_` or `_pct_change_` time aggregator, instead use `change_aggr(time_aggr(time_window),
    /// timeshift):space_aggr:metric{tags} [by {key}] operator #` with:
    ///
    /// - `change_aggr` change, pct_change
    /// - `time_aggr` avg, sum, max, min [Learn more](<https://docs.datadoghq.com/monitors/create/types/#define-the-conditions>)
    /// - `time_window` last\_#m (between 1 and 2880 depending on the monitor type), last\_#h (between 1 and 48 depending on the monitor type), or last_#d (1 or 2)
    /// - `timeshift` #m_ago (5, 10, 15, or 30), #h_ago (1, 2, or 4), or 1d_ago
    ///
    /// Use this to create an outlier monitor using the following query:
    /// `avg(last_30m):outliers(avg:system.cpu.user{role:es-events-data} by {host}, 'dbscan', 7) > 0`
    ///
    /// ##### Service Check Query
    ///
    /// Example: `"check".over(tags).last(count).by(group).count_by_status()`
    ///
    /// - `check` name of the check, for example `datadog.agent.up`
    /// - `tags` one or more quoted tags (comma-separated), or "*". for example: `.over("env:prod", "role:db")`; `over` cannot be blank.
    /// - `count` must be at greater than or equal to your max threshold (defined in the `options`). It is limited to 100.
    /// For example, if you've specified to notify on 1 critical, 3 ok, and 2 warn statuses, `count` should be at least 3.
    /// - `group` must be specified for check monitors. Per-check grouping is already explicitly known for some service checks.
    /// For example, Postgres integration monitors are tagged by `db`, `host`, and `port`, and Network monitors by `host`, `instance`, and `url`. See [Service Checks](<https://docs.datadoghq.com/api/latest/service-checks/>) documentation for more information.
    ///
    /// ##### Event Alert Query
    ///
    /// **Note:** The Event Alert Query has been replaced by the Event V2 Alert Query. For more information, see the [Event Migration guide](<https://docs.datadoghq.com/service_management/events/guides/migrating_to_new_events_features/>).
    ///
    /// ##### Event V2 Alert Query
    ///
    /// Example: `events(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Process Alert Query
    ///
    /// Example: `processes(search).over(tags).rollup('count').last(timeframe) operator #`
    ///
    /// - `search` free text search string for querying processes.
    /// Matching processes match results on the [Live Processes](<https://docs.datadoghq.com/infrastructure/process/?tab=linuxwindows>) page.
    /// - `tags` one or more tags (comma-separated)
    /// - `timeframe` the timeframe to roll up the counts. Examples: 10m, 4h. Supported timeframes: s, m, h and d
    /// - `operator` <, <=, >, >=, ==, or !=
    /// - `#` an integer or decimal number used to set the threshold
    ///
    /// ##### Logs Alert Query
    ///
    /// Example: `logs(query).index(index_name).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `index_name` For multi-index organizations, the log index in which the request is performed.
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Composite Query
    ///
    /// Example: `12345 && 67890`, where `12345` and `67890` are the IDs of non-composite monitors
    ///
    /// * `name` [*required*, *default* = **dynamic, based on query**]: The name of the alert.
    /// * `message` [*required*, *default* = **dynamic, based on query**]: A message to include with notifications for this monitor.
    /// Email notifications can be sent to specific users by using the same '@username' notation as events.
    /// * `tags` [*optional*, *default* = **empty list**]: A list of tags to associate with your monitor.
    /// When getting all monitor details via the API, use the `monitor_tags` argument to filter results by these tags.
    /// It is only available via the API and isn't visible or editable in the Datadog UI.
    ///
    /// ##### SLO Alert Query
    ///
    /// Example: `error_budget("slo_id").over("time_window") operator #`
    ///
    /// - `slo_id`: The alphanumeric SLO ID of the SLO you are configuring the alert for.
    /// - `time_window`: The time window of the SLO target you wish to alert on. Valid options: `7d`, `30d`, `90d`.
    /// - `operator`: `>=` or `>`
    ///
    /// ##### Audit Alert Query
    ///
    /// Example: `audits(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### CI Pipelines Alert Query
    ///
    /// Example: `ci-pipelines(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### CI Tests Alert Query
    ///
    /// Example: `ci-tests(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Error Tracking Alert Query
    ///
    /// Example(RUM): `error-tracking-rum(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    /// Example(APM Traces): `error-tracking-traces(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// **Database Monitoring Alert Query**
    ///
    /// Example: `database-monitoring(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// **NOTE** Database Monitoring monitors are in alpha on US1.
    pub async fn create_monitor(
        &self,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<crate::datadogV1::model::Monitor, Error<CreateMonitorError>> {
        match self.create_monitor_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a monitor using the specified options.
    ///
    /// #### Monitor Types
    ///
    /// The type of monitor chosen from:
    ///
    /// - anomaly: `query alert`
    /// - APM: `query alert` or `trace-analytics alert`
    /// - composite: `composite`
    /// - custom: `service check`
    /// - event: `event alert`
    /// - forecast: `query alert`
    /// - host: `service check`
    /// - integration: `query alert` or `service check`
    /// - live process: `process alert`
    /// - logs: `log alert`
    /// - metric: `query alert`
    /// - network: `service check`
    /// - outlier: `query alert`
    /// - process: `service check`
    /// - rum: `rum alert`
    /// - SLO: `slo alert`
    /// - watchdog: `event-v2 alert`
    /// - event-v2: `event-v2 alert`
    /// - audit: `audit alert`
    /// - error-tracking: `error-tracking alert`
    /// - database-monitoring: `database-monitoring alert`
    ///
    /// **Notes**:
    /// - Synthetic monitors are created through the Synthetics API. See the [Synthetics API] (<https://docs.datadoghq.com/api/latest/synthetics/>) documentation for more information.
    /// - Log monitors require an unscoped App Key.
    ///
    /// #### Query Types
    ///
    /// ##### Metric Alert Query
    ///
    /// Example: `time_aggr(time_window):space_aggr:metric{tags} [by {key}] operator #`
    ///
    /// - `time_aggr`: avg, sum, max, min, change, or pct_change
    /// - `time_window`: `last_#m` (with `#` between 1 and 10080 depending on the monitor type) or `last_#h`(with `#` between 1 and 168 depending on the monitor type) or `last_1d`, or `last_1w`
    /// - `space_aggr`: avg, sum, min, or max
    /// - `tags`: one or more tags (comma-separated), or *
    /// - `key`: a 'key' in key:value tag syntax; defines a separate alert for each tag in the group (multi-alert)
    /// - `operator`: <, <=, >, >=, ==, or !=
    /// - `#`: an integer or decimal number used to set the threshold
    ///
    /// If you are using the `_change_` or `_pct_change_` time aggregator, instead use `change_aggr(time_aggr(time_window),
    /// timeshift):space_aggr:metric{tags} [by {key}] operator #` with:
    ///
    /// - `change_aggr` change, pct_change
    /// - `time_aggr` avg, sum, max, min [Learn more](<https://docs.datadoghq.com/monitors/create/types/#define-the-conditions>)
    /// - `time_window` last\_#m (between 1 and 2880 depending on the monitor type), last\_#h (between 1 and 48 depending on the monitor type), or last_#d (1 or 2)
    /// - `timeshift` #m_ago (5, 10, 15, or 30), #h_ago (1, 2, or 4), or 1d_ago
    ///
    /// Use this to create an outlier monitor using the following query:
    /// `avg(last_30m):outliers(avg:system.cpu.user{role:es-events-data} by {host}, 'dbscan', 7) > 0`
    ///
    /// ##### Service Check Query
    ///
    /// Example: `"check".over(tags).last(count).by(group).count_by_status()`
    ///
    /// - `check` name of the check, for example `datadog.agent.up`
    /// - `tags` one or more quoted tags (comma-separated), or "*". for example: `.over("env:prod", "role:db")`; `over` cannot be blank.
    /// - `count` must be at greater than or equal to your max threshold (defined in the `options`). It is limited to 100.
    /// For example, if you've specified to notify on 1 critical, 3 ok, and 2 warn statuses, `count` should be at least 3.
    /// - `group` must be specified for check monitors. Per-check grouping is already explicitly known for some service checks.
    /// For example, Postgres integration monitors are tagged by `db`, `host`, and `port`, and Network monitors by `host`, `instance`, and `url`. See [Service Checks](<https://docs.datadoghq.com/api/latest/service-checks/>) documentation for more information.
    ///
    /// ##### Event Alert Query
    ///
    /// **Note:** The Event Alert Query has been replaced by the Event V2 Alert Query. For more information, see the [Event Migration guide](<https://docs.datadoghq.com/service_management/events/guides/migrating_to_new_events_features/>).
    ///
    /// ##### Event V2 Alert Query
    ///
    /// Example: `events(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Process Alert Query
    ///
    /// Example: `processes(search).over(tags).rollup('count').last(timeframe) operator #`
    ///
    /// - `search` free text search string for querying processes.
    /// Matching processes match results on the [Live Processes](<https://docs.datadoghq.com/infrastructure/process/?tab=linuxwindows>) page.
    /// - `tags` one or more tags (comma-separated)
    /// - `timeframe` the timeframe to roll up the counts. Examples: 10m, 4h. Supported timeframes: s, m, h and d
    /// - `operator` <, <=, >, >=, ==, or !=
    /// - `#` an integer or decimal number used to set the threshold
    ///
    /// ##### Logs Alert Query
    ///
    /// Example: `logs(query).index(index_name).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `index_name` For multi-index organizations, the log index in which the request is performed.
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Composite Query
    ///
    /// Example: `12345 && 67890`, where `12345` and `67890` are the IDs of non-composite monitors
    ///
    /// * `name` [*required*, *default* = **dynamic, based on query**]: The name of the alert.
    /// * `message` [*required*, *default* = **dynamic, based on query**]: A message to include with notifications for this monitor.
    /// Email notifications can be sent to specific users by using the same '@username' notation as events.
    /// * `tags` [*optional*, *default* = **empty list**]: A list of tags to associate with your monitor.
    /// When getting all monitor details via the API, use the `monitor_tags` argument to filter results by these tags.
    /// It is only available via the API and isn't visible or editable in the Datadog UI.
    ///
    /// ##### SLO Alert Query
    ///
    /// Example: `error_budget("slo_id").over("time_window") operator #`
    ///
    /// - `slo_id`: The alphanumeric SLO ID of the SLO you are configuring the alert for.
    /// - `time_window`: The time window of the SLO target you wish to alert on. Valid options: `7d`, `30d`, `90d`.
    /// - `operator`: `>=` or `>`
    ///
    /// ##### Audit Alert Query
    ///
    /// Example: `audits(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg` and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### CI Pipelines Alert Query
    ///
    /// Example: `ci-pipelines(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### CI Tests Alert Query
    ///
    /// Example: `ci-tests(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// ##### Error Tracking Alert Query
    ///
    /// Example(RUM): `error-tracking-rum(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    /// Example(APM Traces): `error-tracking-traces(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// **Database Monitoring Alert Query**
    ///
    /// Example: `database-monitoring(query).rollup(rollup_method[, measure]).last(time_window) operator #`
    ///
    /// - `query` The search query - following the [Log search syntax](<https://docs.datadoghq.com/logs/search_syntax/>).
    /// - `rollup_method` The stats roll-up method - supports `count`, `avg`, and `cardinality`.
    /// - `measure` For `avg` and cardinality `rollup_method` - specify the measure or the facet name you want to use.
    /// - `time_window` #m (between 1 and 2880), #h (between 1 and 48).
    /// - `operator` `<`, `<=`, `>`, `>=`, `==`, or `!=`.
    /// - `#` an integer or decimal number used to set the threshold.
    ///
    /// **NOTE** Database Monitoring monitors are in alpha on US1.
    pub async fn create_monitor_with_http_info(
        &self,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<ResponseContent<crate::datadogV1::model::Monitor>, Error<CreateMonitorError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.create_monitor";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::Monitor>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete the specified monitor
    pub async fn delete_monitor(
        &self,
        monitor_id: i64,
        params: DeleteMonitorOptionalParams,
    ) -> Result<crate::datadogV1::model::DeletedMonitor, Error<DeleteMonitorError>> {
        match self.delete_monitor_with_http_info(monitor_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete the specified monitor
    pub async fn delete_monitor_with_http_info(
        &self,
        monitor_id: i64,
        params: DeleteMonitorOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::DeletedMonitor>, Error<DeleteMonitorError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_monitor";

        // unbox and build optional parameters
        let force = params.force;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/{monitor_id}",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        if let Some(ref local_query_param) = force {
            local_req_builder =
                local_req_builder.query(&[("force", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::DeletedMonitor>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<DeleteMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get details about the specified monitor from your organization.
    pub async fn get_monitor(
        &self,
        monitor_id: i64,
        params: GetMonitorOptionalParams,
    ) -> Result<crate::datadogV1::model::Monitor, Error<GetMonitorError>> {
        match self.get_monitor_with_http_info(monitor_id, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get details about the specified monitor from your organization.
    pub async fn get_monitor_with_http_info(
        &self,
        monitor_id: i64,
        params: GetMonitorOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Monitor>, Error<GetMonitorError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_monitor";

        // unbox and build optional parameters
        let group_states = params.group_states;
        let with_downtimes = params.with_downtimes;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/{monitor_id}",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = group_states {
            local_req_builder =
                local_req_builder.query(&[("group_states", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = with_downtimes {
            local_req_builder =
                local_req_builder.query(&[("with_downtimes", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::Monitor>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetMonitorError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get details about the specified monitor from your organization.
    pub async fn list_monitors(
        &self,
        params: ListMonitorsOptionalParams,
    ) -> Result<Vec<crate::datadogV1::model::Monitor>, Error<ListMonitorsError>> {
        match self.list_monitors_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_monitors_with_pagination(
        &self,
        mut params: ListMonitorsOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV1::model::Monitor, Error<ListMonitorsError>>> + '_
    {
        try_stream! {
            let mut page_size: i32 = 100;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            if params.page.is_none() {
                params.page = Some(0);
            }
            loop {
                let resp = self.list_monitors(params.clone()).await?;

                let r = resp;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                params.page = Some(params.page.unwrap() + 1);
            }
        }
    }

    /// Get details about the specified monitor from your organization.
    pub async fn list_monitors_with_http_info(
        &self,
        params: ListMonitorsOptionalParams,
    ) -> Result<ResponseContent<Vec<crate::datadogV1::model::Monitor>>, Error<ListMonitorsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.list_monitors";

        // unbox and build optional parameters
        let group_states = params.group_states;
        let name = params.name;
        let tags = params.tags;
        let monitor_tags = params.monitor_tags;
        let with_downtimes = params.with_downtimes;
        let id_offset = params.id_offset;
        let page = params.page;
        let page_size = params.page_size;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = group_states {
            local_req_builder =
                local_req_builder.query(&[("group_states", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = name {
            local_req_builder =
                local_req_builder.query(&[("name", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = monitor_tags {
            local_req_builder =
                local_req_builder.query(&[("monitor_tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = with_downtimes {
            local_req_builder =
                local_req_builder.query(&[("with_downtimes", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = id_offset {
            local_req_builder =
                local_req_builder.query(&[("id_offset", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<Vec<crate::datadogV1::model::Monitor>>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListMonitorsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Search and filter your monitor groups details.
    pub async fn search_monitor_groups(
        &self,
        params: SearchMonitorGroupsOptionalParams,
    ) -> Result<crate::datadogV1::model::MonitorGroupSearchResponse, Error<SearchMonitorGroupsError>>
    {
        match self.search_monitor_groups_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Search and filter your monitor groups details.
    pub async fn search_monitor_groups_with_http_info(
        &self,
        params: SearchMonitorGroupsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MonitorGroupSearchResponse>,
        Error<SearchMonitorGroupsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.search_monitor_groups";

        // unbox and build optional parameters
        let query = params.query;
        let page = params.page;
        let per_page = params.per_page;
        let sort = params.sort;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/groups/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = per_page {
            local_req_builder =
                local_req_builder.query(&[("per_page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MonitorGroupSearchResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SearchMonitorGroupsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Search and filter your monitors details.
    pub async fn search_monitors(
        &self,
        params: SearchMonitorsOptionalParams,
    ) -> Result<crate::datadogV1::model::MonitorSearchResponse, Error<SearchMonitorsError>> {
        match self.search_monitors_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Search and filter your monitors details.
    pub async fn search_monitors_with_http_info(
        &self,
        params: SearchMonitorsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::MonitorSearchResponse>,
        Error<SearchMonitorsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.search_monitors";

        // unbox and build optional parameters
        let query = params.query;
        let page = params.page;
        let per_page = params.per_page;
        let sort = params.sort;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/search",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = per_page {
            local_req_builder =
                local_req_builder.query(&[("per_page", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::MonitorSearchResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<SearchMonitorsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit the specified monitor.
    pub async fn update_monitor(
        &self,
        monitor_id: i64,
        body: crate::datadogV1::model::MonitorUpdateRequest,
    ) -> Result<crate::datadogV1::model::Monitor, Error<UpdateMonitorError>> {
        match self.update_monitor_with_http_info(monitor_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Edit the specified monitor.
    pub async fn update_monitor_with_http_info(
        &self,
        monitor_id: i64,
        body: crate::datadogV1::model::MonitorUpdateRequest,
    ) -> Result<ResponseContent<crate::datadogV1::model::Monitor>, Error<UpdateMonitorError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.update_monitor";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/{monitor_id}",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::Monitor>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Validate the monitor provided in the request.
    pub async fn validate_existing_monitor(
        &self,
        monitor_id: i64,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<
        std::collections::BTreeMap<String, serde_json::Value>,
        Error<ValidateExistingMonitorError>,
    > {
        match self
            .validate_existing_monitor_with_http_info(monitor_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Validate the monitor provided in the request.
    pub async fn validate_existing_monitor_with_http_info(
        &self,
        monitor_id: i64,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<ValidateExistingMonitorError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.validate_existing_monitor";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/{monitor_id}/validate",
            local_configuration.get_operation_host(operation_id),
            monitor_id = monitor_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ValidateExistingMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Validate the monitor provided in the request.
    ///
    /// **Note**: Log monitors require an unscoped App Key.
    pub async fn validate_monitor(
        &self,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<std::collections::BTreeMap<String, serde_json::Value>, Error<ValidateMonitorError>>
    {
        match self.validate_monitor_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Validate the monitor provided in the request.
    ///
    /// **Note**: Log monitors require an unscoped App Key.
    pub async fn validate_monitor_with_http_info(
        &self,
        body: crate::datadogV1::model::Monitor,
    ) -> Result<
        ResponseContent<std::collections::BTreeMap<String, serde_json::Value>>,
        Error<ValidateMonitorError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.validate_monitor";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/validate",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<std::collections::BTreeMap<String, serde_json::Value>>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ValidateMonitorError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}
