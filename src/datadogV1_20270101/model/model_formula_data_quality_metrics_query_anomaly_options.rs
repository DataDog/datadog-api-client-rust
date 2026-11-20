// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaDataQualityMetricsQueryAnomalyOptions {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "direction")]
    pub direction: Option<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptionsDirection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "rule_id")]
    pub rule_id: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaDataQualityMetricsQueryAnomalyOptions {
    pub fn new() -> FormulaDataQualityMetricsQueryAnomalyOptions {
        FormulaDataQualityMetricsQueryAnomalyOptions {
            direction: None,
            rule_id: None,
            _unparsed: false,
        }
    }
    
    pub fn direction(mut self, value: crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptionsDirection) -> Self {
        self.direction = Some(value);
        self
    }
    
    pub fn rule_id(mut self, value: f64) -> Self {
        self.rule_id = Some(value);
        self
    }
    
}


impl Default for FormulaDataQualityMetricsQueryAnomalyOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FormulaDataQualityMetricsQueryAnomalyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaDataQualityMetricsQueryAnomalyOptionsVisitor;
        impl<'a> Visitor<'a> for FormulaDataQualityMetricsQueryAnomalyOptionsVisitor {
            type Value = FormulaDataQualityMetricsQueryAnomalyOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut direction: Option<crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptionsDirection> = None;
                let mut rule_id: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "direction" => {
                            if v.is_null() {
                                continue;
                            }
                            direction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _direction) = direction {
                                match _direction {
                                    crate::datadogV1_20270101::model::FormulaDataQualityMetricsQueryAnomalyOptionsDirection::UnparsedObject(_direction) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "rule_id" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = FormulaDataQualityMetricsQueryAnomalyOptions {
                    direction,
                    rule_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaDataQualityMetricsQueryAnomalyOptionsVisitor)
    }
}
