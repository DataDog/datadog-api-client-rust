// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Batching configuration for ClickHouse inserts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClickhouseDestinationBatch {
    /// Maximum number of events per batch before it is flushed.
    #[serde(rename = "max_events")]
    pub max_events: Option<i64>,
    /// Maximum number of seconds to wait before flushing a partial batch.
    #[serde(rename = "timeout_secs")]
    pub timeout_secs: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClickhouseDestinationBatch {
    pub fn new() -> ObservabilityPipelineClickhouseDestinationBatch {
        ObservabilityPipelineClickhouseDestinationBatch {
            max_events: None,
            timeout_secs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn max_events(mut self, value: i64) -> Self {
        self.max_events = Some(value);
        self
    }

    pub fn timeout_secs(mut self, value: i64) -> Self {
        self.timeout_secs = Some(value);
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

impl Default for ObservabilityPipelineClickhouseDestinationBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestinationBatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClickhouseDestinationBatchVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClickhouseDestinationBatchVisitor {
            type Value = ObservabilityPipelineClickhouseDestinationBatch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut max_events: Option<i64> = None;
                let mut timeout_secs: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "max_events" => {
                            if v.is_null() {
                                continue;
                            }
                            max_events = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeout_secs" => {
                            if v.is_null() {
                                continue;
                            }
                            timeout_secs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineClickhouseDestinationBatch {
                    max_events,
                    timeout_secs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClickhouseDestinationBatchVisitor)
    }
}
