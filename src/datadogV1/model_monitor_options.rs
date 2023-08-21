// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptions {
    /// Type of aggregation performed in the monitor query.
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: MonitorOptionsAggregation,
    /// IDs of the device the Synthetics monitor is running on.
    #[serde(rename = "device_ids", skip_serializing_if = "Option::is_none")]
    pub device_ids: Vec<MonitorDeviceID>,
    /// Whether or not to send a log sample when the log monitor triggers.
    #[serde(rename = "enable_logs_sample", skip_serializing_if = "Option::is_none")]
    pub enable_logs_sample: bool,
    /// Whether or not to send a list of samples when the monitor triggers. This is only used by CI Test and Pipeline monitors.
    #[serde(rename = "enable_samples", skip_serializing_if = "Option::is_none")]
    pub enable_samples: bool,
    /// We recommend using the [is_renotify](https://docs.datadoghq.com/monitors/notify/?tab=is_alert#renotify),
block in the original message instead.
A message to include with a re-notification. Supports the `@username` notification we allow elsewhere.
Not applicable if `renotify_interval` is `None`.
    #[serde(rename = "escalation_message", skip_serializing_if = "Option::is_none")]
    pub escalation_message: String,
    /// Time (in seconds) to delay evaluation, as a non-negative integer. For example, if the value is set to `300` (5min),
the timeframe is set to `last_5m` and the time is 7:00, the monitor evaluates data from 6:50 to 6:55.
This is useful for AWS CloudWatch and other backfilled metrics to ensure the monitor always has data during evaluation.
    #[serde(rename = "evaluation_delay", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub evaluation_delay: Option<Int64>,
    /// The time span after which groups with missing data are dropped from the monitor state.
The minimum value is one hour, and the maximum value is 72 hours.
Example values are: "60m", "1h", and "2d".
This option is only available for APM Trace Analytics, Audit Trail, CI, Error Tracking, Event, Logs, and RUM monitors.
    #[serde(rename = "group_retention_duration", skip_serializing_if = "Option::is_none")]
    pub group_retention_duration: String,
    /// Whether the log alert monitor triggers a single alert or multiple alerts when any group breaches a threshold.
    #[serde(rename = "groupby_simple_monitor", skip_serializing_if = "Option::is_none")]
    pub groupby_simple_monitor: bool,
    /// A Boolean indicating whether notifications from this monitor automatically inserts its triggering tags into the title.

**Examples**
- If `True`, `[Triggered on {host:h1}] Monitor Title`
- If `False`, `[Triggered] Monitor Title`
    #[serde(rename = "include_tags", skip_serializing_if = "Option::is_none")]
    pub include_tags: bool,
    /// Whether or not the monitor is locked (only editable by creator and admins). Use `restricted_roles` instead.
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: bool,
    /// How long the test should be in failure before alerting (integer, number of seconds, max 7200).
    #[serde(rename = "min_failure_duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub min_failure_duration: Option<Int64>,
    /// The minimum number of locations in failure at the same time during
at least one moment in the `min_failure_duration` period (`min_location_failed` and `min_failure_duration`
are part of the advanced alerting rules - integer, >= 1).
    #[serde(rename = "min_location_failed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub min_location_failed: Option<Int64>,
    /// Time (in seconds) to skip evaluations for new groups.

For example, this option can be used to skip evaluations for new hosts while they initialize.

Must be a non negative integer.
    #[serde(rename = "new_group_delay", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub new_group_delay: Option<Int64>,
    /// Time (in seconds) to allow a host to boot and applications
to fully start before starting the evaluation of monitor results.
Should be a non negative integer.

Use new_group_delay instead.
    #[serde(rename = "new_host_delay", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub new_host_delay: Option<Int64>,
    /// The number of minutes before a monitor notifies after data stops reporting.
Datadog recommends at least 2x the monitor timeframe for query alerts or 2 minutes for service checks.
If omitted, 2x the evaluation timeframe is used for query alerts, and 24 hours is used for service checks.
    #[serde(rename = "no_data_timeframe", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub no_data_timeframe: Option<Int64>,
    /// Toggles the display of additional content sent in the monitor notification.
    #[serde(rename = "notification_preset_name", skip_serializing_if = "Option::is_none")]
    pub notification_preset_name: MonitorOptionsNotificationPresets,
    /// A Boolean indicating whether tagged users is notified on changes to this monitor.
    #[serde(rename = "notify_audit", skip_serializing_if = "Option::is_none")]
    pub notify_audit: bool,
    /// Controls what granularity a monitor alerts on. Only available for monitors with groupings.
For instance, a monitor grouped by `cluster`, `namespace`, and `pod` can be configured to only notify on each
new `cluster` violating the alert conditions by setting `notify_by` to `["cluster"]`. Tags mentioned
in `notify_by` must be a subset of the grouping tags in the query.
For example, a query grouped by `cluster` and `namespace` cannot notify on `region`.
Setting `notify_by` to `[*]` configures the monitor to notify as a simple-alert.
    #[serde(rename = "notify_by", skip_serializing_if = "Option::is_none")]
    pub notify_by: Vec<String>,
    /// A Boolean indicating whether this monitor notifies when data stops reporting.
    #[serde(rename = "notify_no_data", skip_serializing_if = "Option::is_none")]
    pub notify_no_data: bool,
    /// Controls how groups or monitors are treated if an evaluation does not return any data points.
The default option results in different behavior depending on the monitor query type.
For monitors using Count queries, an empty monitor evaluation is treated as 0 and is compared to the threshold conditions.
For monitors using any query type other than Count, for example Gauge, Measure, or Rate, the monitor shows the last known status.
This option is only available for APM Trace Analytics, Audit Trail, CI, Error Tracking, Event, Logs, and RUM monitors.
    #[serde(rename = "on_missing_data", skip_serializing_if = "Option::is_none")]
    pub on_missing_data: OnMissingDataOption,
    /// The number of minutes after the last notification before a monitor re-notifies on the current status.
It only re-notifies if it’s not resolved.
    #[serde(rename = "renotify_interval", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub renotify_interval: Option<Int64>,
    /// The number of times re-notification messages should be sent on the current status at the provided re-notification interval.
    #[serde(rename = "renotify_occurrences", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub renotify_occurrences: Option<Int64>,
    /// The types of monitor statuses for which re-notification messages are sent.
    #[serde(rename = "renotify_statuses", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub renotify_statuses: Vec<MonitorRenotifyStatusType>,
    /// A Boolean indicating whether this monitor needs a full window of data before it’s evaluated.
We highly recommend you set this to `false` for sparse metrics,
otherwise some evaluations are skipped. Default is false.
    #[serde(rename = "require_full_window", skip_serializing_if = "Option::is_none")]
    pub require_full_window: bool,
    /// Configuration options for scheduling.
    #[serde(rename = "scheduling_options", skip_serializing_if = "Option::is_none")]
    pub scheduling_options: MonitorOptionsSchedulingOptions,
    /// Information about the downtime applied to the monitor.
    #[serde(rename = "silenced", skip_serializing_if = "Option::is_none")]
    pub silenced: map[string]i64,
    /// ID of the corresponding Synthetic check.
    #[serde(rename = "synthetics_check_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub synthetics_check_id: Option<String>,
    /// Alerting time window options.
    #[serde(rename = "threshold_windows", skip_serializing_if = "Option::is_none")]
    pub threshold_windows: MonitorThresholdWindowOptions,
    /// List of the different monitor threshold available.
    #[serde(rename = "thresholds", skip_serializing_if = "Option::is_none")]
    pub thresholds: MonitorThresholds,
    /// The number of hours of the monitor not reporting data before it automatically resolves from a triggered state. The minimum allowed value is 0 hours. The maximum allowed value is 24 hours.
    #[serde(rename = "timeout_h", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub timeout_h: Option<Int64>,
    /// List of requests that can be used in the monitor query. **This feature is currently in beta.**
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Vec<MonitorFormulaAndFunctionQueryDefinition>,
}

