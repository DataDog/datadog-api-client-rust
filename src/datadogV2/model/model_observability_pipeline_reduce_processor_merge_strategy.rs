// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines how a specific field should be merged across grouped events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineReduceProcessorMergeStrategy {
    /// The field path in the log event.
    #[serde(rename = "path")]
    pub path: String,
    /// The merge strategy to apply.
    #[serde(rename = "strategy")]
    pub strategy:
        crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategyStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineReduceProcessorMergeStrategy {
    pub fn new(
        path: String,
        strategy: crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategyStrategy,
    ) -> ObservabilityPipelineReduceProcessorMergeStrategy {
        ObservabilityPipelineReduceProcessorMergeStrategy {
            path,
            strategy,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineReduceProcessorMergeStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineReduceProcessorMergeStrategyVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineReduceProcessorMergeStrategyVisitor {
            type Value = ObservabilityPipelineReduceProcessorMergeStrategy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut path: Option<String> = None;
                let mut strategy: Option<crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategyStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "path" => {
                            path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategyStrategy::UnparsedObject(_strategy) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let path = path.ok_or_else(|| M::Error::missing_field("path"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = ObservabilityPipelineReduceProcessorMergeStrategy {
                    path,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineReduceProcessorMergeStrategyVisitor)
    }
}
