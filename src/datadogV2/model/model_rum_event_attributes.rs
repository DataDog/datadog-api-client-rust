// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object containing all event attributes and their associated values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMEventAttributes {
    /// JSON object of attributes from RUM events.
    #[serde(rename = "attributes")]
    pub attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The name of the application or service generating RUM events.
    /// It is used to switch from RUM to APM, so make sure you define the same
    /// value when you use both products.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Array of tags associated with your event.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Timestamp of your event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMEventAttributes {
    pub fn new() -> RUMEventAttributes {
        RUMEventAttributes {
            attributes: None,
            service: None,
            tags: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
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

impl Default for RUMEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMEventAttributesVisitor;
        impl<'a> Visitor<'a> for RUMEventAttributesVisitor {
            type Value = RUMEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut service: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMEventAttributes {
                    attributes,
                    service,
                    tags,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMEventAttributesVisitor)
    }
}
