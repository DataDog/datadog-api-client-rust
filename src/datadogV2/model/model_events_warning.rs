// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A warning message indicating something is wrong with the query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsWarning {
    /// A unique code for this type of warning.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// A detailed explanation of this specific warning.
    #[serde(rename = "detail")]
    pub detail: Option<String>,
    /// A short human-readable summary of the warning.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventsWarning {
    pub fn new() -> EventsWarning {
        EventsWarning {
            code: None,
            detail: None,
            title: None,
            _unparsed: false,
        }
    }

    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }

    pub fn detail(mut self, value: String) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }
}

impl Default for EventsWarning {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventsWarning {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventsWarningVisitor;
        impl<'a> Visitor<'a> for EventsWarningVisitor {
            type Value = EventsWarning;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<String> = None;
                let mut detail: Option<String> = None;
                let mut title: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            if v.is_null() {
                                continue;
                            }
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detail" => {
                            if v.is_null() {
                                continue;
                            }
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = EventsWarning {
                    code,
                    detail,
                    title,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventsWarningVisitor)
    }
}
