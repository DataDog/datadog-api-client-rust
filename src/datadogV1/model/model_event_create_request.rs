// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object representing an event.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventCreateRequest {
    /// An arbitrary string to use for aggregation. Limited to 100 characters.
    /// If you specify a key, all events using that key are grouped together in the Event Stream.
    #[serde(rename = "aggregation_key")]
    pub aggregation_key: Option<String>,
    /// If an alert event is enabled, set its type.
    /// For example, `error`, `warning`, `info`, `success`, `user_update`,
    /// `recommendation`, and `snapshot`.
    #[serde(rename = "alert_type")]
    pub alert_type: Option<crate::datadogV1::model::EventAlertType>,
    /// POSIX timestamp of the event. Must be sent as an integer (that is no quotes).
    /// Limited to events no older than 18 hours
    #[serde(rename = "date_happened")]
    pub date_happened: Option<i64>,
    /// A device name.
    #[serde(rename = "device_name")]
    pub device_name: Option<String>,
    /// Host name to associate with the event.
    /// Any tags associated with the host are also applied to this event.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// The priority of the event. For example, `normal` or `low`.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option")]
    pub priority: Option<Option<crate::datadogV1::model::EventPriority>>,
    /// ID of the parent event. Must be sent as an integer (that is no quotes).
    #[serde(rename = "related_event_id")]
    pub related_event_id: Option<i64>,
    /// The type of event being posted. Option examples include nagios, hudson, jenkins, my_apps, chef, puppet, git, bitbucket, etc.
    /// A complete list of source attribute values [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    #[serde(rename = "source_type_name")]
    pub source_type_name: Option<String>,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The body of the event. Limited to 4000 characters. The text supports markdown.
    /// To use markdown in the event text, start the text block with `%%% \n` and end the text block with `\n %%%`.
    /// Use `msg_text` with the Datadog Ruby library.
    #[serde(rename = "text")]
    pub text: String,
    /// The event title.
    #[serde(rename = "title")]
    pub title: String,
}

impl EventCreateRequest {
    pub fn new(text: String, title: String) -> EventCreateRequest {
        EventCreateRequest {
            aggregation_key: None,
            alert_type: None,
            date_happened: None,
            device_name: None,
            host: None,
            priority: None,
            related_event_id: None,
            source_type_name: None,
            tags: None,
            text,
            title,
        }
    }
}
