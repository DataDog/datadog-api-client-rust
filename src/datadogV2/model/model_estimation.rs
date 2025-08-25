// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recommended resource values for a Spark driver or executor, derived from recent real usage metrics. Used by SPA to propose more efficient pod sizing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Estimation {
    /// CPU usage statistics derived from historical Spark job metrics. Provides multiple estimates so users can choose between conservative and cost-saving risk profiles.
    #[serde(rename = "cpu")]
    pub cpu: Option<crate::datadogV2::model::Cpu>,
    /// Recommended ephemeral storage allocation (in MiB). Derived from job temporary storage patterns.
    #[serde(rename = "ephemeral_storage")]
    pub ephemeral_storage: Option<i64>,
    /// Recommended JVM heap size (in MiB).
    #[serde(rename = "heap")]
    pub heap: Option<i64>,
    /// Recommended total memory allocation (in MiB). Includes both heap and overhead.
    #[serde(rename = "memory")]
    pub memory: Option<i64>,
    /// Recommended JVM overhead (in MiB). Computed as total memory - heap.
    #[serde(rename = "overhead")]
    pub overhead: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Estimation {
    pub fn new() -> Estimation {
        Estimation {
            cpu: None,
            ephemeral_storage: None,
            heap: None,
            memory: None,
            overhead: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cpu(mut self, value: crate::datadogV2::model::Cpu) -> Self {
        self.cpu = Some(value);
        self
    }

    pub fn ephemeral_storage(mut self, value: i64) -> Self {
        self.ephemeral_storage = Some(value);
        self
    }

    pub fn heap(mut self, value: i64) -> Self {
        self.heap = Some(value);
        self
    }

    pub fn memory(mut self, value: i64) -> Self {
        self.memory = Some(value);
        self
    }

    pub fn overhead(mut self, value: i64) -> Self {
        self.overhead = Some(value);
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

impl Default for Estimation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for Estimation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EstimationVisitor;
        impl<'a> Visitor<'a> for EstimationVisitor {
            type Value = Estimation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cpu: Option<crate::datadogV2::model::Cpu> = None;
                let mut ephemeral_storage: Option<i64> = None;
                let mut heap: Option<i64> = None;
                let mut memory: Option<i64> = None;
                let mut overhead: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cpu" => {
                            if v.is_null() {
                                continue;
                            }
                            cpu = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ephemeral_storage" => {
                            if v.is_null() {
                                continue;
                            }
                            ephemeral_storage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "heap" => {
                            if v.is_null() {
                                continue;
                            }
                            heap = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "memory" => {
                            if v.is_null() {
                                continue;
                            }
                            memory = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overhead" => {
                            if v.is_null() {
                                continue;
                            }
                            overhead = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = Estimation {
                    cpu,
                    ephemeral_storage,
                    heap,
                    memory,
                    overhead,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EstimationVisitor)
    }
}
