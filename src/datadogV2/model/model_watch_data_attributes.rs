// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WatchDataAttributes {
    #[serde(rename = "application_id")]
    pub application_id: String,
    #[serde(rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WatchDataAttributes {
    pub fn new(
        application_id: String,
        event_id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> WatchDataAttributes {
        WatchDataAttributes {
            application_id,
            data_source: None,
            event_id,
            timestamp,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_source(mut self, value: String) -> Self {
        self.data_source = Some(value);
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

impl<'de> Deserialize<'de> for WatchDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WatchDataAttributesVisitor;
        impl<'a> Visitor<'a> for WatchDataAttributesVisitor {
            type Value = WatchDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut data_source: Option<String> = None;
                let mut event_id: Option<String> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_id" => {
                            event_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let application_id =
                    application_id.ok_or_else(|| M::Error::missing_field("application_id"))?;
                let event_id = event_id.ok_or_else(|| M::Error::missing_field("event_id"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;

                let content = WatchDataAttributes {
                    application_id,
                    data_source,
                    event_id,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WatchDataAttributesVisitor)
    }
}
