// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// If an alert event is enabled, set its type.
For example, `error`, `warning`, `info`, `success`, `user_update`,
`recommendation`, and `snapshot`.
    #[serde(rename = "alert_type", skip_serializing_if = "Option::is_none")]
    pub alert_type: EventAlertType,
    /// POSIX timestamp of the event. Must be sent as an integer (that is no quotes).
Limited to events no older than 18 hours.
    #[serde(rename = "date_happened", skip_serializing_if = "Option::is_none")]
    pub date_happened: i64,
    /// A device name.
    #[serde(rename = "device_name", skip_serializing_if = "Option::is_none")]
    pub device_name: String,
    /// Host name to associate with the event.
Any tags associated with the host are also applied to this event.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: String,
    /// Integer ID of the event.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// Handling IDs as large 64-bit numbers can cause loss of accuracy issues with some programming languages.
Instead, use the string representation of the Event ID to avoid losing accuracy.
    #[serde(rename = "id_str", skip_serializing_if = "Option::is_none")]
    pub id_str: String,
    /// Payload of the event.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: String,
    /// The priority of the event. For example, `normal` or `low`.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub priority: NullableEventPriority,
    /// The type of event being posted. Option examples include nagios, hudson, jenkins, my_apps, chef, puppet, git, bitbucket, etc.
The list of standard source attribute values [available here](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value).
    #[serde(rename = "source_type_name", skip_serializing_if = "Option::is_none")]
    pub source_type_name: String,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The body of the event. Limited to 4000 characters. The text supports markdown.
To use markdown in the event text, start the text block with `%%% \n` and end the text block with `\n %%%`.
Use `msg_text` with the Datadog Ruby library.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: String,
    /// The event title.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: String,
    /// URL of the event.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: String,
}

