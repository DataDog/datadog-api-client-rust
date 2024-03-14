// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The query results
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsAggregateResponseData {
    /// The list of matching buckets, one item per bucket
    #[serde(rename = "buckets")]
    pub buckets: Option<Vec<crate::datadogV2::model::LogsAggregateBucket>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsAggregateResponseData {
    pub fn new() -> LogsAggregateResponseData {
        LogsAggregateResponseData {
            buckets: None,
            _unparsed: false,
        }
    }

    pub fn buckets(mut self, value: Vec<crate::datadogV2::model::LogsAggregateBucket>) -> Self {
        self.buckets = Some(value);
        self
    }
}

impl Default for LogsAggregateResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsAggregateResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsAggregateResponseDataVisitor;
        impl<'a> Visitor<'a> for LogsAggregateResponseDataVisitor {
            type Value = LogsAggregateResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut buckets: Option<Vec<crate::datadogV2::model::LogsAggregateBucket>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "buckets" => {
                            if v.is_null() {
                                continue;
                            }
                            buckets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsAggregateResponseData { buckets, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsAggregateResponseDataVisitor)
    }
}
