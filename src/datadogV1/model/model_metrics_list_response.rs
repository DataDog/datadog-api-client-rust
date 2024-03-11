// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object listing all metric names stored by Datadog since a given time.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsListResponse {
    /// Time when the metrics were active, seconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: Option<String>,
    /// List of metric names.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricsListResponse {
    pub fn new() -> MetricsListResponse {
        MetricsListResponse {
            from: None,
            metrics: None,
            _unparsed: false,
        }
    }

    pub fn from(&mut self, value: String) -> &mut Self {
        self.from = Some(value);
        self
    }

    pub fn metrics(&mut self, value: Vec<String>) -> &mut Self {
        self.metrics = Some(value);
        self
    }
}

impl Default for MetricsListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricsListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsListResponseVisitor;
        impl<'a> Visitor<'a> for MetricsListResponseVisitor {
            type Value = MetricsListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<String> = None;
                let mut metrics: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricsListResponse {
                    from,
                    metrics,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsListResponseVisitor)
    }
}
