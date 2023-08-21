// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventAttributes {
    /// Aggregation key of the event.
    #[serde(rename = "aggregation_key", skip_serializing_if = "Option::is_none")]
    pub aggregation_key: String,
    /// POSIX timestamp of the event. Must be sent as an integer (no quotation marks).
Limited to events no older than 18 hours.
    #[serde(rename = "date_happened", skip_serializing_if = "Option::is_none")]
    pub date_happened: i64,
    /// A device name.
    #[serde(rename = "device_name", skip_serializing_if = "Option::is_none")]
    pub device_name: String,
    /// The duration between the triggering of the event and its recovery in nanoseconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: i64,
    /// The event title.
    #[serde(rename = "event_object", skip_serializing_if = "Option::is_none")]
    pub event_object: String,
    /// The metadata associated with a request.
    #[serde(rename = "evt", skip_serializing_if = "Option::is_none")]
    pub evt: Event,
    /// Host name to associate with the event.
Any tags associated with the host are also applied to this event.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: String,
    /// Attributes from the monitor that triggered the event.
    #[serde(rename = "monitor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub monitor: NullableMonitorType,
    /// List of groups referred to in the event.
    #[serde(rename = "monitor_groups", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub monitor_groups: datadog.NullableList[String],
    /// ID of the monitor that triggered the event. When an event isn't related to a monitor, this field is empty.
    #[serde(rename = "monitor_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub monitor_id: Option<Int64>,
    /// The priority of the event's monitor. For example, `normal` or `low`.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub priority: NullableEventPriority,
    /// Related event ID.
    #[serde(rename = "related_event_id", skip_serializing_if = "Option::is_none")]
    pub related_event_id: i64,
    /// Service that triggered the event.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
    /// The type of event being posted.
For example, `nagios`, `hudson`, `jenkins`, `my_apps`, `chef`, `puppet`, `git` or `bitbucket`.
The list of standard source attribute values is [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    #[serde(rename = "source_type_name", skip_serializing_if = "Option::is_none")]
    pub source_type_name: String,
    /// Identifier for the source of the event, such as a monitor alert, an externally-submitted event, or an integration.
    #[serde(rename = "sourcecategory", skip_serializing_if = "Option::is_none")]
    pub sourcecategory: String,
    /// If an alert event is enabled, its status is one of the following:
`failure`, `error`, `warning`, `info`, `success`, `user_update`,
`recommendation`, or `snapshot`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: EventStatusType,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// POSIX timestamp of your event in milliseconds.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: i64,
    /// The event title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
}

