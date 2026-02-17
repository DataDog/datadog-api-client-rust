// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for cross-product sampling when updating a retention filter. All fields are optional for partial updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumCrossProductSamplingUpdate {
    /// Indicates whether trace cross-product sampling is enabled.
    #[serde(rename = "trace_enabled")]
    pub trace_enabled: Option<bool>,
    /// The percentage (0-100) of retained sessions with ingested traces whose traces are indexed.
    #[serde(rename = "trace_sample_rate")]
    pub trace_sample_rate: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumCrossProductSamplingUpdate {
    pub fn new() -> RumCrossProductSamplingUpdate {
        RumCrossProductSamplingUpdate {
            trace_enabled: None,
            trace_sample_rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn trace_enabled(mut self, value: bool) -> Self {
        self.trace_enabled = Some(value);
        self
    }

    pub fn trace_sample_rate(mut self, value: f64) -> Self {
        self.trace_sample_rate = Some(value);
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

impl Default for RumCrossProductSamplingUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumCrossProductSamplingUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumCrossProductSamplingUpdateVisitor;
        impl<'a> Visitor<'a> for RumCrossProductSamplingUpdateVisitor {
            type Value = RumCrossProductSamplingUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut trace_enabled: Option<bool> = None;
                let mut trace_sample_rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "trace_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_sample_rate" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumCrossProductSamplingUpdate {
                    trace_enabled,
                    trace_sample_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumCrossProductSamplingUpdateVisitor)
    }
}
