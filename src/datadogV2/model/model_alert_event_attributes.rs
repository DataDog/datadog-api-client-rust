// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Alert event attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlertEventAttributes {
    /// Aggregation key of the event.
    #[serde(rename = "aggregation_key")]
    pub aggregation_key: Option<String>,
    /// JSON object of custom attributes.
    #[serde(rename = "custom")]
    pub custom: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// JSON object of event system attributes.
    #[serde(rename = "evt")]
    pub evt: Option<crate::datadogV2::model::EventSystemAttributes>,
    /// The links related to the event.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::AlertEventAttributesLinksItem>>,
    /// The priority of the alert.
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::AlertEventAttributesPriority>,
    /// Service that triggered the event.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// The status of the alert.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::AlertEventAttributesStatus>,
    /// POSIX timestamp of the event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    /// The title of the event.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AlertEventAttributes {
    pub fn new() -> AlertEventAttributes {
        AlertEventAttributes {
            aggregation_key: None,
            custom: None,
            evt: None,
            links: None,
            priority: None,
            service: None,
            status: None,
            timestamp: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregation_key(mut self, value: String) -> Self {
        self.aggregation_key = Some(value);
        self
    }

    pub fn custom(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn evt(mut self, value: crate::datadogV2::model::EventSystemAttributes) -> Self {
        self.evt = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: Vec<crate::datadogV2::model::AlertEventAttributesLinksItem>,
    ) -> Self {
        self.links = Some(value);
        self
    }

    pub fn priority(
        mut self,
        value: crate::datadogV2::model::AlertEventAttributesPriority,
    ) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::AlertEventAttributesStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for AlertEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AlertEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AlertEventAttributesVisitor;
        impl<'a> Visitor<'a> for AlertEventAttributesVisitor {
            type Value = AlertEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_key: Option<String> = None;
                let mut custom: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut evt: Option<crate::datadogV2::model::EventSystemAttributes> = None;
                let mut links: Option<Vec<crate::datadogV2::model::AlertEventAttributesLinksItem>> =
                    None;
                let mut priority: Option<crate::datadogV2::model::AlertEventAttributesPriority> =
                    None;
                let mut service: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::AlertEventAttributesStatus> = None;
                let mut timestamp: Option<i64> = None;
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
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evt" => {
                            if v.is_null() {
                                continue;
                            }
                            evt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _priority) = priority {
                                match _priority {
                                    crate::datadogV2::model::AlertEventAttributesPriority::UnparsedObject(_priority) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::AlertEventAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AlertEventAttributes {
                    aggregation_key,
                    custom,
                    evt,
                    links,
                    priority,
                    service,
                    status,
                    timestamp,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AlertEventAttributesVisitor)
    }
}
