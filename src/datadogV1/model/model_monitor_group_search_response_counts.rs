// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The counts of monitor groups per different criteria.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorGroupSearchResponseCounts {
    /// Search facets.
    #[serde(rename = "status")]
    pub status: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    /// Search facets.
    #[serde(rename = "type")]
    pub type_: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorGroupSearchResponseCounts {
    pub fn new() -> MonitorGroupSearchResponseCounts {
        MonitorGroupSearchResponseCounts {
            status: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn status(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn type_(mut self, value: Vec<crate::datadogV1::model::MonitorSearchCountItem>) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for MonitorGroupSearchResponseCounts {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorGroupSearchResponseCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorGroupSearchResponseCountsVisitor;
        impl<'a> Visitor<'a> for MonitorGroupSearchResponseCountsVisitor {
            type Value = MonitorGroupSearchResponseCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut status: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut type_: Option<Vec<crate::datadogV1::model::MonitorSearchCountItem>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorGroupSearchResponseCounts {
                    status,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorGroupSearchResponseCountsVisitor)
    }
}
