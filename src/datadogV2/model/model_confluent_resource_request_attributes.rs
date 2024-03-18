// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for updating a Confluent resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfluentResourceRequestAttributes {
    /// Enable the `custom.consumer_lag_offset` metric, which contains extra metric tags.
    #[serde(rename = "enable_custom_metrics")]
    pub enable_custom_metrics: Option<bool>,
    /// The resource type of the Resource. Can be `kafka`, `connector`, `ksql`, or `schema_registry`.
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// A list of strings representing tags. Can be a single key, or key-value pairs separated by a colon.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfluentResourceRequestAttributes {
    pub fn new(resource_type: String) -> ConfluentResourceRequestAttributes {
        ConfluentResourceRequestAttributes {
            enable_custom_metrics: None,
            resource_type,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn enable_custom_metrics(mut self, value: bool) -> Self {
        self.enable_custom_metrics = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ConfluentResourceRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfluentResourceRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ConfluentResourceRequestAttributesVisitor {
            type Value = ConfluentResourceRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enable_custom_metrics: Option<bool> = None;
                let mut resource_type: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enable_custom_metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            enable_custom_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;

                let content = ConfluentResourceRequestAttributes {
                    enable_custom_metrics,
                    resource_type,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfluentResourceRequestAttributesVisitor)
    }
}
