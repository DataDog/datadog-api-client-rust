// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing array of IDs of canceled downtimes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CanceledDowntimesIds {
    /// ID of downtimes that were canceled.
    #[serde(rename = "cancelled_ids")]
    pub cancelled_ids: Option<Vec<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CanceledDowntimesIds {
    pub fn new() -> CanceledDowntimesIds {
        CanceledDowntimesIds {
            cancelled_ids: None,
            _unparsed: false,
        }
    }

    pub fn cancelled_ids(mut self, value: Vec<i64>) -> Self {
        self.cancelled_ids = Some(value);
        self
    }
}

impl Default for CanceledDowntimesIds {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CanceledDowntimesIds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CanceledDowntimesIdsVisitor;
        impl<'a> Visitor<'a> for CanceledDowntimesIdsVisitor {
            type Value = CanceledDowntimesIds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cancelled_ids: Option<Vec<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cancelled_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            cancelled_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = CanceledDowntimesIds {
                    cancelled_ids,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CanceledDowntimesIdsVisitor)
    }
}
