// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional metadata associated to the response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityFilterMeta {
    /// A warning message.
    #[serde(rename = "warning")]
    pub warning: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityFilterMeta {
    pub fn new() -> SecurityFilterMeta {
        SecurityFilterMeta {
            warning: None,
            _unparsed: false,
        }
    }

    pub fn warning(mut self, value: String) -> Self {
        self.warning = Some(value);
        self
    }
}

impl Default for SecurityFilterMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityFilterMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityFilterMetaVisitor;
        impl<'a> Visitor<'a> for SecurityFilterMetaVisitor {
            type Value = SecurityFilterMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut warning: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "warning" => {
                            if v.is_null() {
                                continue;
                            }
                            warning = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityFilterMeta { warning, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityFilterMetaVisitor)
    }
}
