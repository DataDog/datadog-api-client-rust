// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing a change event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEvent {
    /// An arbitrary string to use for aggregation. Limited to 100 characters.
    /// If you specify a key, all events using that key are grouped together in the Event Stream.
    #[serde(rename = "aggregation_key")]
    pub aggregation_key: Option<String>,
    /// Object representing custom event attributes required for Change events. The overall object
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ChangeEventCustomAttributes,
    /// Event category to identify the type of event. Currently only `change` value is supported.
    #[serde(rename = "category")]
    pub category: crate::datadogV2::model::ChangeEventCategory,
    /// The body of the event. Limited to 4000 characters. The text supports markdown.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// A list of tags to apply to the event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp in which the event occurred. Must follow [ISO 8601](<https://www.iso.org/iso-8601-date-and-time-format.html>) format.
    /// Defaults to now. Limited to events no older than 18 hours.
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

impl ChangeEvent {
    pub fn new(
        attributes: crate::datadogV2::model::ChangeEventCustomAttributes,
        category: crate::datadogV2::model::ChangeEventCategory,
        title: String,
    ) -> ChangeEvent {
        ChangeEvent {
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

impl<'de> Deserialize<'de> for ChangeEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeEventVisitor;
        impl<'a> Visitor<'a> for ChangeEventVisitor {
            type Value = ChangeEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_key: Option<String> = None;
                let mut attributes: Option<crate::datadogV2::model::ChangeEventCustomAttributes> =
                    None;
                let mut category: Option<crate::datadogV2::model::ChangeEventCategory> = None;
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
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::ChangeEventCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
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

                let content = ChangeEvent {
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

        deserializer.deserialize_any(ChangeEventVisitor)
    }
}
