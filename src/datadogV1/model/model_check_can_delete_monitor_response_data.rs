// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Wrapper object with the list of monitor IDs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckCanDeleteMonitorResponseData {
    /// An array of of Monitor IDs that can be safely deleted.
    #[serde(rename = "ok")]
    pub ok: Option<Vec<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CheckCanDeleteMonitorResponseData {
    pub fn new() -> CheckCanDeleteMonitorResponseData {
        CheckCanDeleteMonitorResponseData {
            ok: None,
            _unparsed: false,
        }
    }

    pub fn ok(&mut self, value: Vec<i64>) -> &mut Self {
        self.ok = Some(value);
        self
    }
}

impl Default for CheckCanDeleteMonitorResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CheckCanDeleteMonitorResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckCanDeleteMonitorResponseDataVisitor;
        impl<'a> Visitor<'a> for CheckCanDeleteMonitorResponseDataVisitor {
            type Value = CheckCanDeleteMonitorResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ok: Option<Vec<i64>> = None;
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

                let content = CheckCanDeleteMonitorResponseData { ok, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CheckCanDeleteMonitorResponseDataVisitor)
    }
}
