// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A facet item.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorSearchCountItem {
    /// The number of found monitors with the listed value.
    #[serde(rename = "count")]
    pub count: Option<i64>,
    /// The facet value.
    #[serde(rename = "name")]
    pub name: Option<serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorSearchCountItem {
    pub fn new() -> MonitorSearchCountItem {
        MonitorSearchCountItem {
            count: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn name(mut self, value: serde_json::Value) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for MonitorSearchCountItem {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorSearchCountItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSearchCountItemVisitor;
        impl<'a> Visitor<'a> for MonitorSearchCountItemVisitor {
            type Value = MonitorSearchCountItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i64> = None;
                let mut name: Option<serde_json::Value> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorSearchCountItem {
                    count,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSearchCountItemVisitor)
    }
}
