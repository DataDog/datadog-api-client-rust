// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An array of service level objective objects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckCanDeleteSLOResponseData {
    /// An array of of SLO IDs that can be safely deleted.
    #[serde(rename = "ok")]
    pub ok: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CheckCanDeleteSLOResponseData {
    pub fn new() -> CheckCanDeleteSLOResponseData {
        CheckCanDeleteSLOResponseData {
            ok: None,
            _unparsed: false,
        }
    }

    pub fn ok(mut self, value: Vec<String>) -> Self {
        self.ok = Some(value);
        self
    }
}

impl Default for CheckCanDeleteSLOResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CheckCanDeleteSLOResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckCanDeleteSLOResponseDataVisitor;
        impl<'a> Visitor<'a> for CheckCanDeleteSLOResponseDataVisitor {
            type Value = CheckCanDeleteSLOResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ok: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ok" => {
                            if v.is_null() {
                                continue;
                            }
                            ok = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CheckCanDeleteSLOResponseData { ok, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CheckCanDeleteSLOResponseDataVisitor)
    }
}
