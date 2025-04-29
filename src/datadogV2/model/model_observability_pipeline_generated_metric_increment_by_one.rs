// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Strategy that increments a generated metric by one for each matching event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGeneratedMetricIncrementByOne {
    /// Increments the metric by 1 for each matching event.
    #[serde(rename = "strategy")]
    pub strategy:
        crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOneStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGeneratedMetricIncrementByOne {
    pub fn new(
        strategy: crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOneStrategy,
    ) -> ObservabilityPipelineGeneratedMetricIncrementByOne {
        ObservabilityPipelineGeneratedMetricIncrementByOne {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGeneratedMetricIncrementByOne {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGeneratedMetricIncrementByOneVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGeneratedMetricIncrementByOneVisitor {
            type Value = ObservabilityPipelineGeneratedMetricIncrementByOne;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut strategy: Option<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOneStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByOneStrategy::UnparsedObject(_strategy) => {
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
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = ObservabilityPipelineGeneratedMetricIncrementByOne {
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGeneratedMetricIncrementByOneVisitor)
    }
}
