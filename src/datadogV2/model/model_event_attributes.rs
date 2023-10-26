// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventAttributes {
    /// Aggregation key of the event.
    #[serde(rename = "aggregation_key", skip_serializing_if = "Option::is_none")]
    pub aggregation_key: Option<String>,
    /// POSIX timestamp of the event. Must be sent as an integer (no quotation marks).
    /// Limited to events no older than 18 hours.
    #[serde(rename = "date_happened", skip_serializing_if = "Option::is_none")]
    pub date_happened: Option<i64>,
    /// A device name.
    #[serde(rename = "device_name", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// The duration between the triggering of the event and its recovery in nanoseconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// The event title.
    #[serde(rename = "event_object", skip_serializing_if = "Option::is_none")]
    pub event_object: Option<String>,
    /// The metadata associated with a request.
    #[serde(rename = "evt", skip_serializing_if = "Option::is_none")]
    pub evt: Option<Box<crate::datadogV2::model::Event>>,
    /// Host name to associate with the event.
    /// Any tags associated with the host are also applied to this event.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Attributes from the monitor that triggered the event.
    #[serde(
        rename = "monitor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor: Option<Option<Box<crate::datadogV2::model::MonitorType>>>,
    /// List of groups referred to in the event.
    #[serde(
        rename = "monitor_groups",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_groups: Option<Option<Vec<String>>>,
    /// ID of the monitor that triggered the event. When an event isn't related to a monitor, this field is empty.
    #[serde(
        rename = "monitor_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub monitor_id: Option<Option<i64>>,
    /// The priority of the event's monitor. For example, `normal` or `low`.
    #[serde(
        rename = "priority",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub priority: Option<Option<crate::datadogV2::model::EventPriority>>,
    /// Related event ID.
    #[serde(rename = "related_event_id", skip_serializing_if = "Option::is_none")]
    pub related_event_id: Option<i64>,
    /// Service that triggered the event.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The type of event being posted.
    /// For example, `nagios`, `hudson`, `jenkins`, `my_apps`, `chef`, `puppet`, `git` or `bitbucket`.
    /// The list of standard source attribute values is [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    #[serde(rename = "source_type_name", skip_serializing_if = "Option::is_none")]
    pub source_type_name: Option<String>,
    /// Identifier for the source of the event, such as a monitor alert, an externally-submitted event, or an integration.
    #[serde(rename = "sourcecategory", skip_serializing_if = "Option::is_none")]
    pub sourcecategory: Option<String>,
    /// If an alert event is enabled, its status is one of the following:
    /// `failure`, `error`, `warning`, `info`, `success`, `user_update`,
    /// `recommendation`, or `snapshot`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::datadogV2::model::EventStatusType>,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// POSIX timestamp of your event in milliseconds.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The event title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl EventAttributes {
    /// Object description of attributes from your event.
    pub fn new() -> EventAttributes {
        EventAttributes {
            aggregation_key: None,
            date_happened: None,
            device_name: None,
            duration: None,
            event_object: None,
            evt: None,
            hostname: None,
            monitor: None,
            monitor_groups: None,
            monitor_id: None,
            priority: None,
            related_event_id: None,
            service: None,
            source_type_name: None,
            sourcecategory: None,
            status: None,
            tags: None,
            timestamp: None,
            title: None,
        }
    }
}
