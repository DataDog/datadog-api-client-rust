// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A group by rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsMetricGroupBy {
    /// The path to the value the log-based metric will be aggregated over.
    #[serde(rename = "path")]
    pub path: String,
    /// Eventual name of the tag that gets created. By default, the path attribute is used as the tag name.
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsMetricGroupBy {
    pub fn new(path: String) -> LogsMetricGroupBy {
        LogsMetricGroupBy {
            path,
            tag_name: None,
            _unparsed: false,
        }
    }

    pub fn tag_name(mut self, value: String) -> Self {
        self.tag_name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsMetricGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsMetricGroupByVisitor;
        impl<'a> Visitor<'a> for LogsMetricGroupByVisitor {
            type Value = LogsMetricGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut path: Option<String> = None;
                let mut tag_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_name" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;

                let content = LogsMetricGroupBy {
                    path,
                    tag_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsMetricGroupByVisitor)
    }
}
