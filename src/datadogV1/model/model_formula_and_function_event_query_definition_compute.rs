// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Compute options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionEventQueryDefinitionCompute {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// A time interval in milliseconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Measurable attribute to compute.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionEventQueryDefinitionCompute {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> FormulaAndFunctionEventQueryDefinitionCompute {
        FormulaAndFunctionEventQueryDefinitionCompute {
            aggregation,
            interval: None,
            metric: None,
            _unparsed: false,
        }
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn metric(mut self, value: String) -> Self {
        self.metric = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryDefinitionCompute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryDefinitionComputeVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryDefinitionComputeVisitor {
            type Value = FormulaAndFunctionEventQueryDefinitionCompute;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventAggregation,
                > = None;
                let mut interval: Option<i64> = None;
                let mut metric: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV1::model::FormulaAndFunctionEventAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric" => {
                            if v.is_null() {
                                continue;
                            }
                            metric = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;

                let content = FormulaAndFunctionEventQueryDefinitionCompute {
                    aggregation,
                    interval,
                    metric,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryDefinitionComputeVisitor)
    }
}
