// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a source to target monitor, which compares the same measure
/// across two data entities and alerts on the difference between them.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
    /// How the difference between the source and target measures is computed.
    /// `absolute` subtracts the two values, `diff_percent` expresses the difference
    /// as a percentage of the source value.
    #[serde(rename = "diff_type")]
    pub diff_type: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffType,
    /// Type of the data entities being compared.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// Measure configuration for one side of a source to target comparison.
    #[serde(rename = "source")]
    pub source: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
    /// Measure configuration for one side of a source to target comparison.
    #[serde(rename = "target")]
    pub target: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
    pub fn new(
        diff_type: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffType,
        entity_type: String,
        source: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
        target: crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
    ) -> MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
        MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
            diff_type,
            entity_type,
            source,
            target,
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

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionDataQualitySourceToTargetConfigVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionDataQualitySourceToTargetConfigVisitor {
            type Value = MonitorFormulaAndFunctionDataQualitySourceToTargetConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut diff_type: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffType,
                > = None;
                let mut entity_type: Option<String> = None;
                let mut source: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
                > = None;
                let mut target: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityEntityMetricConfig,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "diff_type" => {
                            diff_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _diff_type) = diff_type {
                                match _diff_type {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionDataQualityDiffType::UnparsedObject(_diff_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "entity_type" => {
                            entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let diff_type = diff_type.ok_or_else(|| M::Error::missing_field("diff_type"))?;
                let entity_type =
                    entity_type.ok_or_else(|| M::Error::missing_field("entity_type"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;

                let content = MonitorFormulaAndFunctionDataQualitySourceToTargetConfig {
                    diff_type,
                    entity_type,
                    source,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(MonitorFormulaAndFunctionDataQualitySourceToTargetConfigVisitor)
    }
}
