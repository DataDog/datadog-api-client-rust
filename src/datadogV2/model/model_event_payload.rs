// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Event attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventPayload {
    /// An arbitrary string to use for aggregation when correlating events. Limited to 100 characters.
    #[serde(rename = "aggregation_key")]
    pub aggregation_key: Option<String>,
    /// JSON object for custom attributes. Schema are different per each event category.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::EventPayloadAttributes,
    /// Event category to identify the type of event. Only the value `change` is supported. Support for other categories are coming. please reach out to datadog support if you're interested.
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::EventCategory,
    /// The body of the event. Limited to 4000 characters.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// A list of tags to apply to the event.
    /// Refer to [Tags docs](<https://docs.datadoghq.com/getting_started/tagging/>).
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp when the event occurred. Must follow [ISO 8601](<https://www.iso.org/iso-8601-date-and-time-format.html>) format.
    /// For example `"2017-01-15T01:30:15.010000Z"`.
    /// Defaults to the timestamp of receipt. Limited to values no older than 18 hours.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// The event title. Limited to 500 characters.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventPayload {
    pub fn new(
        attributes: crate::datadogV2::model::EventPayloadAttributes,
        category: crate::datadogV2::model::EventCategory,
        title: String,
    ) -> EventPayload {
        EventPayload {
            aggregation_key: None,
            attributes,
            category,
            message: None,
            tags: None,
            timestamp: None,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregation_key(mut self, value: String) -> Self {
        self.aggregation_key = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for EventPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventPayloadVisitor;
        impl<'a> Visitor<'a> for EventPayloadVisitor {
            type Value = EventPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_key: Option<String> = None;
                let mut attributes: Option<crate::datadogV2::model::EventPayloadAttributes> = None;
                let mut category: Option<crate::datadogV2::model::EventCategory> = None;
                let mut message: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation_key" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _attributes) = attributes {
                                match _attributes {
                                    crate::datadogV2::model::EventPayloadAttributes::UnparsedObject(_attributes) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::EventCategory::UnparsedObject(
                                        _category,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = EventPayload {
                    aggregation_key,
                    attributes,
                    category,
                    message,
                    tags,
                    timestamp,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventPayloadVisitor)
    }
}
