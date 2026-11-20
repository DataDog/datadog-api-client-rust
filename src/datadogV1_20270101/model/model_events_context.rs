// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsContext {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "context_event")]
    pub context_event: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "to_event")]
    pub to_event: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl EventsContext {
    pub fn new() -> EventsContext {
        EventsContext {
            context_event: None,
            to_event: None,
            _unparsed: false,
        }
    }
    
    pub fn context_event(mut self, value: String) -> Self {
        self.context_event = Some(value);
        self
    }
    
    pub fn to_event(mut self, value: String) -> Self {
        self.to_event = Some(value);
        self
    }
    
}


impl Default for EventsContext {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventsContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsContextVisitor;
        impl<'a> Visitor<'a> for EventsContextVisitor {
            type Value = EventsContext;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut context_event: Option<String> = None;
                let mut to_event: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "context_event" => {
                            if v.is_null() {
                                continue;
                            }
                            context_event = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "to_event" => {
                            if v.is_null() {
                                continue;
                            }
                            to_event = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = EventsContext {
                    context_event,
                    to_event,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsContextVisitor)
    }
}
