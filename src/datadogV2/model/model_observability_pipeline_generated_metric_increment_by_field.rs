// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Strategy that increments a generated metric based on the value of a log field.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGeneratedMetricIncrementByField {
    /// Name of the log field containing the numeric value to increment the metric by.
    #[serde(rename = "field")]
    pub field: String,
    /// Uses a numeric field in the log event as the metric increment.
    #[serde(rename = "strategy")]
    pub strategy:
        crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByFieldStrategy,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGeneratedMetricIncrementByField {
    pub fn new(
        field: String,
        strategy: crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByFieldStrategy,
    ) -> ObservabilityPipelineGeneratedMetricIncrementByField {
        ObservabilityPipelineGeneratedMetricIncrementByField {
            field,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGeneratedMetricIncrementByField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGeneratedMetricIncrementByFieldVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGeneratedMetricIncrementByFieldVisitor {
            type Value = ObservabilityPipelineGeneratedMetricIncrementByField;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field: Option<String> = None;
                let mut strategy: Option<crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByFieldStrategy> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "strategy" => {
                            strategy = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _strategy) = strategy {
                                match _strategy {
                                    crate::datadogV2::model::ObservabilityPipelineGeneratedMetricIncrementByFieldStrategy::UnparsedObject(_strategy) => {
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
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;
                let strategy = strategy.ok_or_else(|| M::Error::missing_field("strategy"))?;

                let content = ObservabilityPipelineGeneratedMetricIncrementByField {
                    field,
                    strategy,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGeneratedMetricIncrementByFieldVisitor)
    }
}
