// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metric origin information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricOrigin {
    /// The origin metric type code
    #[serde(rename = "metric_type")]
    pub metric_type: Option<i32>,
    /// The origin product code
    #[serde(rename = "product")]
    pub product: Option<i32>,
    /// The origin service code
    #[serde(rename = "service")]
    pub service: Option<i32>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricOrigin {
    pub fn new() -> MetricOrigin {
        MetricOrigin {
            metric_type: None,
            product: None,
            service: None,
            _unparsed: false,
        }
    }

    pub fn metric_type(mut self, value: i32) -> Self {
        self.metric_type = Some(value);
        self
    }

    pub fn product(mut self, value: i32) -> Self {
        self.product = Some(value);
        self
    }

    pub fn service(mut self, value: i32) -> Self {
        self.service = Some(value);
        self
    }
}

impl Default for MetricOrigin {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricOrigin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricOriginVisitor;
        impl<'a> Visitor<'a> for MetricOriginVisitor {
            type Value = MetricOrigin;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metric_type: Option<i32> = None;
                let mut product: Option<i32> = None;
                let mut service: Option<i32> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metric_type" => {
                            if v.is_null() {
                                continue;
                            }
                            metric_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product" => {
                            if v.is_null() {
                                continue;
                            }
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricOrigin {
                    metric_type,
                    product,
                    service,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricOriginVisitor)
    }
}
