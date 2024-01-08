// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object representing an event.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// If an alert event is enabled, set its type.
    /// For example, `error`, `warning`, `info`, `success`, `user_update`,
    /// `recommendation`, and `snapshot`.
    #[serde(rename = "alert_type")]
    pub alert_type: Option<crate::datadogV1::model::EventAlertType>,
    /// POSIX timestamp of the event. Must be sent as an integer (that is no quotes).
    /// Limited to events no older than 18 hours.
    #[serde(rename = "date_happened")]
    pub date_happened: Option<i64>,
    /// A device name.
    #[serde(rename = "device_name")]
    pub device_name: Option<String>,
    /// Host name to associate with the event.
    /// Any tags associated with the host are also applied to this event.
    #[serde(rename = "host")]
    pub host: Option<String>,
    /// Integer ID of the event.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Handling IDs as large 64-bit numbers can cause loss of accuracy issues with some programming languages.
    /// Instead, use the string representation of the Event ID to avoid losing accuracy.
    #[serde(rename = "id_str")]
    pub id_str: Option<String>,
    /// Payload of the event.
    #[serde(rename = "payload")]
    pub payload: Option<String>,
    /// The priority of the event. For example, `normal` or `low`.
    #[serde(
        rename = "priority",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<crate::datadogV1::model::EventPriority>>,
    /// The type of event being posted. Option examples include nagios, hudson, jenkins, my_apps, chef, puppet, git, bitbucket, etc.
    /// The list of standard source attribute values [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    #[serde(rename = "source_type_name")]
    pub source_type_name: Option<String>,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The body of the event. Limited to 4000 characters. The text supports markdown.
    /// To use markdown in the event text, start the text block with `%%% \n` and end the text block with `\n %%%`.
    /// Use `msg_text` with the Datadog Ruby library.
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// The event title.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// URL of the event.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            alert_type: None,
            date_happened: None,
            device_name: None,
            host: None,
            id: None,
            id_str: None,
            payload: None,
            priority: None,
            source_type_name: None,
            tags: None,
            text: None,
            title: None,
            url: None,
        }
    }
}
impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}
