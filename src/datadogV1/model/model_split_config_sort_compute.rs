// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the metric and aggregation used as the sort value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplitConfigSortCompute {
    /// How to aggregate the sort metric for the purposes of ordering.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// The metric to use for sorting graphs.
    #[serde(rename = "metric")]
    pub metric: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SplitConfigSortCompute {
    pub fn new(aggregation: String, metric: String) -> SplitConfigSortCompute {
        SplitConfigSortCompute {
            aggregation,
            metric,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SplitConfigSortCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitConfigSortComputeVisitor;
        impl<'a> Visitor<'a> for SplitConfigSortComputeVisitor {
            type Value = SplitConfigSortCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut metric: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let metric = metric.ok_or_else(|| M::Error::missing_field("metric"))?;

                let content = SplitConfigSortCompute {
                    aggregation,
                    metric,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitConfigSortComputeVisitor)
    }
}
