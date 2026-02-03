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
pub struct WatcherDataAttributes {
    #[serde(rename = "handle")]
    pub handle: String,
    #[serde(rename = "icon")]
    pub icon: Option<String>,
    #[serde(rename = "last_watched_at")]
    pub last_watched_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "watch_count")]
    pub watch_count: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WatcherDataAttributes {
    pub fn new(
        handle: String,
        last_watched_at: chrono::DateTime<chrono::Utc>,
        watch_count: i32,
    ) -> WatcherDataAttributes {
        WatcherDataAttributes {
            handle,
            icon: None,
            last_watched_at,
            name: None,
            watch_count,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn icon(mut self, value: String) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl<'de> Deserialize<'de> for WatcherDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WatcherDataAttributesVisitor;
        impl<'a> Visitor<'a> for WatcherDataAttributesVisitor {
            type Value = WatcherDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut handle: Option<String> = None;
                let mut icon: Option<String> = None;
                let mut last_watched_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut watch_count: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon" => {
                            if v.is_null() {
                                continue;
                            }
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_watched_at" => {
                            last_watched_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "watch_count" => {
                            watch_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let last_watched_at =
                    last_watched_at.ok_or_else(|| M::Error::missing_field("last_watched_at"))?;
                let watch_count =
                    watch_count.ok_or_else(|| M::Error::missing_field("watch_count"))?;

                let content = WatcherDataAttributes {
                    handle,
                    icon,
                    last_watched_at,
                    name,
                    watch_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WatcherDataAttributesVisitor)
    }
}
