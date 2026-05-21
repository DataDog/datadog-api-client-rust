// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for publishing a status page notice.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentStatusPageNoticeCreateDataAttributes {
    /// Map of component identifiers to their status.
    #[serde(rename = "components")]
    pub components: Option<std::collections::BTreeMap<String, String>>,
    /// The message body of the notice.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The status of the notice.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The title of the notice.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentStatusPageNoticeCreateDataAttributes {
    pub fn new() -> IncidentStatusPageNoticeCreateDataAttributes {
        IncidentStatusPageNoticeCreateDataAttributes {
            components: None,
            message: None,
            status: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn components(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
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

impl Default for IncidentStatusPageNoticeCreateDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentStatusPageNoticeCreateDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentStatusPageNoticeCreateDataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentStatusPageNoticeCreateDataAttributesVisitor {
            type Value = IncidentStatusPageNoticeCreateDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut components: Option<std::collections::BTreeMap<String, String>> = None;
                let mut message: Option<String> = None;
                let mut status: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "components" => {
                            if v.is_null() {
                                continue;
                            }
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = IncidentStatusPageNoticeCreateDataAttributes {
                    components,
                    message,
                    status,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentStatusPageNoticeCreateDataAttributesVisitor)
    }
}
