// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of options associated with your monitor.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorOptions {
    /// Type of aggregation performed in the monitor query.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV1::model::MonitorOptionsAggregation>,
    /// IDs of the device the Synthetics monitor is running on.
    #[deprecated]
    #[serde(rename = "device_ids")]
    pub device_ids: Option<Vec<crate::datadogV1::model::MonitorDeviceID>>,
    /// Whether or not to send a log sample when the log monitor triggers.
    #[serde(rename = "enable_logs_sample")]
    pub enable_logs_sample: Option<bool>,
    /// Whether or not to send a list of samples when the monitor triggers. This is only used by CI Test and Pipeline monitors.
    #[serde(rename = "enable_samples")]
    pub enable_samples: Option<bool>,
    /// We recommend using the [is_renotify](<https://docs.datadoghq.com/monitors/notify/?tab=is_alert#renotify>),
    /// block in the original message instead.
    /// A message to include with a re-notification. Supports the `@username` notification we allow elsewhere.
    /// Not applicable if `renotify_interval` is `None`.
    #[serde(rename = "escalation_message")]
    pub escalation_message: Option<String>,
    /// Time (in seconds) to delay evaluation, as a non-negative integer. For example, if the value is set to `300` (5min),
    /// the timeframe is set to `last_5m` and the time is 7:00, the monitor evaluates data from 6:50 to 6:55.
    /// This is useful for AWS CloudWatch and other backfilled metrics to ensure the monitor always has data during evaluation.
    #[serde(
        rename = "evaluation_delay",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub evaluation_delay: Option<Option<i64>>,
    /// The time span after which groups with missing data are dropped from the monitor state.
    /// The minimum value is one hour, and the maximum value is 72 hours.
    /// Example values are: "60m", "1h", and "2d".
    /// This option is only available for APM Trace Analytics, Audit Trail, CI, Error Tracking, Event, Logs, and RUM monitors.
    #[serde(rename = "group_retention_duration")]
    pub group_retention_duration: Option<String>,
    /// Whether the log alert monitor triggers a single alert or multiple alerts when any group breaches a threshold.
    #[serde(rename = "groupby_simple_monitor")]
    pub groupby_simple_monitor: Option<bool>,
    /// A Boolean indicating whether notifications from this monitor automatically inserts its triggering tags into the title.
    ///
    /// **Examples**
    /// - If `True`, `[Triggered on {host:h1}] Monitor Title`
    /// - If `False`, `[Triggered] Monitor Title`
    #[serde(rename = "include_tags")]
    pub include_tags: Option<bool>,
    /// Whether or not the monitor is locked (only editable by creator and admins). Use `restricted_roles` instead.
    #[deprecated]
    #[serde(rename = "locked")]
    pub locked: Option<bool>,
    /// How long the test should be in failure before alerting (integer, number of seconds, max 7200).
    #[serde(
        rename = "min_failure_duration",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub min_failure_duration: Option<Option<i64>>,
    /// The minimum number of locations in failure at the same time during
    /// at least one moment in the `min_failure_duration` period (`min_location_failed` and `min_failure_duration`
    /// are part of the advanced alerting rules - integer, >= 1).
    #[serde(
        rename = "min_location_failed",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub min_location_failed: Option<Option<i64>>,
    /// Time (in seconds) to skip evaluations for new groups.
    ///
    /// For example, this option can be used to skip evaluations for new hosts while they initialize.
    ///
    /// Must be a non negative integer.
    #[serde(
        rename = "new_group_delay",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub new_group_delay: Option<Option<i64>>,
    /// Time (in seconds) to allow a host to boot and applications
    /// to fully start before starting the evaluation of monitor results.
    /// Should be a non negative integer.
    ///
    /// Use new_group_delay instead.
    #[deprecated]
    #[serde(
        rename = "new_host_delay",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub new_host_delay: Option<Option<i64>>,
    /// The number of minutes before a monitor notifies after data stops reporting.
    /// Datadog recommends at least 2x the monitor timeframe for query alerts or 2 minutes for service checks.
    /// If omitted, 2x the evaluation timeframe is used for query alerts, and 24 hours is used for service checks.
    #[serde(
        rename = "no_data_timeframe",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub no_data_timeframe: Option<Option<i64>>,
    /// Toggles the display of additional content sent in the monitor notification.
    #[serde(rename = "notification_preset_name")]
    pub notification_preset_name:
        Option<crate::datadogV1::model::MonitorOptionsNotificationPresets>,
    /// A Boolean indicating whether tagged users is notified on changes to this monitor.
    #[serde(rename = "notify_audit")]
    pub notify_audit: Option<bool>,
    /// Controls what granularity a monitor alerts on. Only available for monitors with groupings.
    /// For instance, a monitor grouped by `cluster`, `namespace`, and `pod` can be configured to only notify on each
    /// new `cluster` violating the alert conditions by setting `notify_by` to `["cluster"]`. Tags mentioned
    /// in `notify_by` must be a subset of the grouping tags in the query.
    /// For example, a query grouped by `cluster` and `namespace` cannot notify on `region`.
    /// Setting `notify_by` to `[*]` configures the monitor to notify as a simple-alert.
    #[serde(rename = "notify_by")]
    pub notify_by: Option<Vec<String>>,
    /// A Boolean indicating whether this monitor notifies when data stops reporting. Defaults to `false`.
    #[serde(rename = "notify_no_data")]
    pub notify_no_data: Option<bool>,
    /// Controls how groups or monitors are treated if an evaluation does not return any data points.
    /// The default option results in different behavior depending on the monitor query type.
    /// For monitors using Count queries, an empty monitor evaluation is treated as 0 and is compared to the threshold conditions.
    /// For monitors using any query type other than Count, for example Gauge, Measure, or Rate, the monitor shows the last known status.
    /// This option is only available for APM Trace Analytics, Audit Trail, CI, Error Tracking, Event, Logs, and RUM monitors.
    #[serde(rename = "on_missing_data")]
    pub on_missing_data: Option<crate::datadogV1::model::OnMissingDataOption>,
    /// The number of minutes after the last notification before a monitor re-notifies on the current status.
    /// It only re-notifies if it’s not resolved.
    #[serde(
        rename = "renotify_interval",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub renotify_interval: Option<Option<i64>>,
    /// The number of times re-notification messages should be sent on the current status at the provided re-notification interval.
    #[serde(
        rename = "renotify_occurrences",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub renotify_occurrences: Option<Option<i64>>,
    /// The types of monitor statuses for which re-notification messages are sent.
    #[serde(
        rename = "renotify_statuses",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub renotify_statuses: Option<Option<Vec<crate::datadogV1::model::MonitorRenotifyStatusType>>>,
    /// A Boolean indicating whether this monitor needs a full window of data before it’s evaluated.
    /// We highly recommend you set this to `false` for sparse metrics,
    /// otherwise some evaluations are skipped. Default is false.
    #[serde(rename = "require_full_window")]
    pub require_full_window: Option<bool>,
    /// Configuration options for scheduling.
    #[serde(rename = "scheduling_options")]
    pub scheduling_options: Option<crate::datadogV1::model::MonitorOptionsSchedulingOptions>,
    /// Information about the downtime applied to the monitor. Only shows v1 downtimes.
    #[deprecated]
    #[serde(rename = "silenced")]
    pub silenced: Option<std::collections::BTreeMap<String, i64>>,
    /// ID of the corresponding Synthetic check.
    #[deprecated]
    #[serde(
        rename = "synthetics_check_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub synthetics_check_id: Option<Option<String>>,
    /// Alerting time window options.
    #[serde(rename = "threshold_windows")]
    pub threshold_windows: Option<crate::datadogV1::model::MonitorThresholdWindowOptions>,
    /// List of the different monitor threshold available.
    #[serde(rename = "thresholds")]
    pub thresholds: Option<crate::datadogV1::model::MonitorThresholds>,
    /// The number of hours of the monitor not reporting data before it automatically resolves from a triggered state. The minimum allowed value is 0 hours. The maximum allowed value is 24 hours.
    #[serde(
        rename = "timeout_h",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub timeout_h: Option<Option<i64>>,
    /// List of requests that can be used in the monitor query. **This feature is currently in beta.**
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition>>,
}

impl MonitorOptions {
    pub fn new() -> MonitorOptions {
        #[allow(deprecated)]
        MonitorOptions {
            aggregation: None,
            device_ids: None,
            enable_logs_sample: None,
            enable_samples: None,
            escalation_message: None,
            evaluation_delay: None,
            group_retention_duration: None,
            groupby_simple_monitor: None,
            include_tags: None,
            locked: None,
            min_failure_duration: None,
            min_location_failed: None,
            new_group_delay: None,
            new_host_delay: None,
            no_data_timeframe: None,
            notification_preset_name: None,
            notify_audit: None,
            notify_by: None,
            notify_no_data: None,
            on_missing_data: None,
            renotify_interval: None,
            renotify_occurrences: None,
            renotify_statuses: None,
            require_full_window: None,
            scheduling_options: None,
            silenced: None,
            synthetics_check_id: None,
            threshold_windows: None,
            thresholds: None,
            timeout_h: None,
            variables: None,
        }
    }

    #[allow(deprecated)]
    pub fn aggregation(
        mut self,
        value: crate::datadogV1::model::MonitorOptionsAggregation,
    ) -> Self {
        self.aggregation = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn device_ids(mut self, value: Vec<crate::datadogV1::model::MonitorDeviceID>) -> Self {
        self.device_ids = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn enable_logs_sample(mut self, value: bool) -> Self {
        self.enable_logs_sample = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn enable_samples(mut self, value: bool) -> Self {
        self.enable_samples = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn escalation_message(mut self, value: String) -> Self {
        self.escalation_message = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn evaluation_delay(mut self, value: Option<i64>) -> Self {
        self.evaluation_delay = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn group_retention_duration(mut self, value: String) -> Self {
        self.group_retention_duration = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn groupby_simple_monitor(mut self, value: bool) -> Self {
        self.groupby_simple_monitor = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn include_tags(mut self, value: bool) -> Self {
        self.include_tags = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn locked(mut self, value: bool) -> Self {
        self.locked = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn min_failure_duration(mut self, value: Option<i64>) -> Self {
        self.min_failure_duration = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn min_location_failed(mut self, value: Option<i64>) -> Self {
        self.min_location_failed = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn new_group_delay(mut self, value: Option<i64>) -> Self {
        self.new_group_delay = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn new_host_delay(mut self, value: Option<i64>) -> Self {
        self.new_host_delay = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn no_data_timeframe(mut self, value: Option<i64>) -> Self {
        self.no_data_timeframe = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn notification_preset_name(
        mut self,
        value: crate::datadogV1::model::MonitorOptionsNotificationPresets,
    ) -> Self {
        self.notification_preset_name = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn notify_audit(mut self, value: bool) -> Self {
        self.notify_audit = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn notify_by(mut self, value: Vec<String>) -> Self {
        self.notify_by = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn notify_no_data(mut self, value: bool) -> Self {
        self.notify_no_data = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn on_missing_data(mut self, value: crate::datadogV1::model::OnMissingDataOption) -> Self {
        self.on_missing_data = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn renotify_interval(mut self, value: Option<i64>) -> Self {
        self.renotify_interval = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn renotify_occurrences(mut self, value: Option<i64>) -> Self {
        self.renotify_occurrences = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn renotify_statuses(
        mut self,
        value: Option<Vec<crate::datadogV1::model::MonitorRenotifyStatusType>>,
    ) -> Self {
        self.renotify_statuses = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn require_full_window(mut self, value: bool) -> Self {
        self.require_full_window = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn scheduling_options(
        mut self,
        value: crate::datadogV1::model::MonitorOptionsSchedulingOptions,
    ) -> Self {
        self.scheduling_options = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn silenced(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.silenced = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn synthetics_check_id(mut self, value: Option<String>) -> Self {
        self.synthetics_check_id = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn threshold_windows(
        mut self,
        value: crate::datadogV1::model::MonitorThresholdWindowOptions,
    ) -> Self {
        self.threshold_windows = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn thresholds(mut self, value: crate::datadogV1::model::MonitorThresholds) -> Self {
        self.thresholds = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn timeout_h(mut self, value: Option<i64>) -> Self {
        self.timeout_h = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn variables(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionQueryDefinition>,
    ) -> Self {
        self.variables = Some(value);
        self
    }
}

impl Default for MonitorOptions {
    fn default() -> Self {
        Self::new()
    }
}
