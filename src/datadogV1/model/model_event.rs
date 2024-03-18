// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing an event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    /// The list of standard source attribute values [available here](<https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value>).
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn alert_type(mut self, value: crate::datadogV1::model::EventAlertType) -> Self {
        self.alert_type = Some(value);
        self
    }

    pub fn date_happened(mut self, value: i64) -> Self {
        self.date_happened = Some(value);
        self
    }

    pub fn device_name(mut self, value: String) -> Self {
        self.device_name = Some(value);
        self
    }

    pub fn host(mut self, value: String) -> Self {
        self.host = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn id_str(mut self, value: String) -> Self {
        self.id_str = Some(value);
        self
    }

    pub fn payload(mut self, value: String) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn priority(mut self, value: Option<crate::datadogV1::model::EventPriority>) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn source_type_name(mut self, value: String) -> Self {
        self.source_type_name = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn text(mut self, value: String) -> Self {
        self.text = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventVisitor;
        impl<'a> Visitor<'a> for EventVisitor {
            type Value = Event;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_type: Option<crate::datadogV1::model::EventAlertType> = None;
                let mut date_happened: Option<i64> = None;
                let mut device_name: Option<String> = None;
                let mut host: Option<String> = None;
                let mut id: Option<i64> = None;
                let mut id_str: Option<String> = None;
                let mut payload: Option<String> = None;
                let mut priority: Option<Option<crate::datadogV1::model::EventPriority>> = None;
                let mut source_type_name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut text: Option<String> = None;
                let mut title: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_type" => {
                            if v.is_null() {
                                continue;
                            }
                            alert_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _alert_type) = alert_type {
                                match _alert_type {
                                    crate::datadogV1::model::EventAlertType::UnparsedObject(
                                        _alert_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "date_happened" => {
                            if v.is_null() {
                                continue;
                            }
                            date_happened =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device_name" => {
                            if v.is_null() {
                                continue;
                            }
                            device_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "host" => {
                            if v.is_null() {
                                continue;
                            }
                            host = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id_str" => {
                            if v.is_null() {
                                continue;
                            }
                            id_str = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "payload" => {
                            if v.is_null() {
                                continue;
                            }
                            payload = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    Some(
                                        crate::datadogV1::model::EventPriority::UnparsedObject(
                                            _priority,
                                        ),
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "source_type_name" => {
                            if v.is_null() {
                                continue;
                            }
                            source_type_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "text" => {
                            if v.is_null() {
                                continue;
                            }
                            text = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = Event {
                    alert_type,
                    date_happened,
                    device_name,
                    host,
                    id,
                    id_str,
                    payload,
                    priority,
                    source_type_name,
                    tags,
                    text,
                    title,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventVisitor)
    }
}
