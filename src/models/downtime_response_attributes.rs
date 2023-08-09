/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DowntimeResponseAttributes : Downtime details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DowntimeResponseAttributes {
    /// Time that the downtime was canceled.
    #[serde(rename = "canceled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Option<String>>,
    /// Creation time of the downtime.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The timezone in which to display the downtime's start and end times in Datadog applications. This is not used as an offset for scheduling.
    #[serde(rename = "display_timezone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_timezone: Option<Option<String>>,
    /// A message to include with notifications for this downtime. Email notifications can be sent to specific users by using the same `@username` notation as events.
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    /// Time that the downtime was last modified.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(rename = "monitor_identifier", skip_serializing_if = "Option::is_none")]
    pub monitor_identifier: Option<Box<crate::models::DowntimeMonitorIdentifier>>,
    /// If the first recovery notification during a downtime should be muted.
    #[serde(rename = "mute_first_recovery_notification", skip_serializing_if = "Option::is_none")]
    pub mute_first_recovery_notification: Option<bool>,
    /// States that will trigger a monitor notification when the `notify_end_types` action occurs.
    #[serde(rename = "notify_end_states", skip_serializing_if = "Option::is_none")]
    pub notify_end_states: Option<Vec<crate::models::DowntimeNotifyEndStateTypes>>,
    /// Actions that will trigger a monitor notification if the downtime is in the `notify_end_types` state.
    #[serde(rename = "notify_end_types", skip_serializing_if = "Option::is_none")]
    pub notify_end_types: Option<Vec<crate::models::DowntimeNotifyEndStateActions>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::DowntimeScheduleResponse>>,
    /// The scope to which the downtime applies. Must follow the [common search syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::DowntimeStatus>,
}

impl DowntimeResponseAttributes {
    /// Downtime details.
    pub fn new() -> DowntimeResponseAttributes {
        DowntimeResponseAttributes {
            canceled: None,
            created: None,
            display_timezone: None,
            message: None,
            modified: None,
            monitor_identifier: None,
            mute_first_recovery_notification: None,
            notify_end_states: None,
            notify_end_types: None,
            schedule: None,
            scope: None,
            status: None,
        }
    }
}


