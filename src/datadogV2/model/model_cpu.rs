// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CPU usage statistics derived from historical Spark job metrics. Provides multiple estimates so users can choose between conservative and cost-saving risk profiles.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Cpu {
    /// Maximum CPU usage observed for the job, expressed in millicores. This represents the upper bound of usage.
    #[serde(rename = "max")]
    pub max: Option<i64>,
    /// 75th percentile of CPU usage (millicores). Represents a cost-saving configuration while covering most workloads.
    #[serde(rename = "p75")]
    pub p75: Option<i64>,
    /// 95th percentile of CPU usage (millicores). Balances performance and cost, providing a safer margin than p75.
    #[serde(rename = "p95")]
    pub p95: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            max: None,
            p75: None,
            p95: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn max(mut self, value: i64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn p75(mut self, value: i64) -> Self {
        self.p75 = Some(value);
        self
    }

    pub fn p95(mut self, value: i64) -> Self {
        self.p95 = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Cpu {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CpuVisitor;
        impl<'a> Visitor<'a> for CpuVisitor {
            type Value = Cpu;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max: Option<i64> = None;
                let mut p75: Option<i64> = None;
                let mut p95: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max" => {
                            if v.is_null() {
                                continue;
                            }
                            max = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "p75" => {
                            if v.is_null() {
                                continue;
                            }
                            p75 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "p95" => {
                            if v.is_null() {
                                continue;
                            }
                            p95 = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = Cpu {
                    max,
                    p75,
                    p95,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CpuVisitor)
    }
}
