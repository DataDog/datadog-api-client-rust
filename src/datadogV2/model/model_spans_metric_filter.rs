// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansMetricFilter {
    /// The search query - following the span search syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansMetricFilter {
    pub fn new() -> SpansMetricFilter {
        SpansMetricFilter {
            query: None,
            _unparsed: false,
        }
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }
}

impl Default for SpansMetricFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SpansMetricFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansMetricFilterVisitor;
        impl<'a> Visitor<'a> for SpansMetricFilterVisitor {
            type Value = SpansMetricFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SpansMetricFilter { query, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansMetricFilterVisitor)
    }
}
