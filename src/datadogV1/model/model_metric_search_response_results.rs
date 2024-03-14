// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Search result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricSearchResponseResults {
    /// List of metrics that match the search query.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricSearchResponseResults {
    pub fn new() -> MetricSearchResponseResults {
        MetricSearchResponseResults {
            metrics: None,
            _unparsed: false,
        }
    }

    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = Some(value);
        self
    }
}

impl Default for MetricSearchResponseResults {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricSearchResponseResults {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricSearchResponseResultsVisitor;
        impl<'a> Visitor<'a> for MetricSearchResponseResultsVisitor {
            type Value = MetricSearchResponseResults;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metrics: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricSearchResponseResults { metrics, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricSearchResponseResultsVisitor)
    }
}
