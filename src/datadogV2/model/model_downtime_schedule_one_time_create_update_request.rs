// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A one-time downtime definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleOneTimeCreateUpdateRequest {
    /// ISO-8601 Datetime to end the downtime. Must include a UTC offset of zero. If not provided, the
    /// downtime continues forever.
    #[serde(rename = "end", default, with = "::serde_with::rust::double_option")]
    pub end: Option<Option<String>>,
    /// ISO-8601 Datetime to start the downtime. Must include a UTC offset of zero. If not provided, the
    /// downtime starts the moment it is created.
    #[serde(rename = "start", default, with = "::serde_with::rust::double_option")]
    pub start: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleOneTimeCreateUpdateRequest {
    pub fn new() -> DowntimeScheduleOneTimeCreateUpdateRequest {
        DowntimeScheduleOneTimeCreateUpdateRequest {
            end: None,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end(&mut self, value: Option<String>) -> &mut Self {
        self.end = Some(value);
        self
    }

    pub fn start(&mut self, value: Option<String>) -> &mut Self {
        self.start = Some(value);
        self
    }
}

impl Default for DowntimeScheduleOneTimeCreateUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeScheduleOneTimeCreateUpdateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleOneTimeCreateUpdateRequestVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleOneTimeCreateUpdateRequestVisitor {
            type Value = DowntimeScheduleOneTimeCreateUpdateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<Option<String>> = None;
                let mut start: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DowntimeScheduleOneTimeCreateUpdateRequest {
                    end,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleOneTimeCreateUpdateRequestVisitor)
    }
}
