// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A central interval around point_estimate at confidence_level. Null when the estimate is unavailable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditNullableConfidenceIntervalV2 {
    /// Lower bound of the interval.
    #[serde(rename = "lower")]
    pub lower: f64,
    /// Upper bound of the interval.
    #[serde(rename = "upper")]
    pub upper: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditNullableConfidenceIntervalV2 {
    pub fn new(lower: f64, upper: f64) -> ExperimentsBanditNullableConfidenceIntervalV2 {
        ExperimentsBanditNullableConfidenceIntervalV2 {
            lower,
            upper,
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

impl<'de> Deserialize<'de> for ExperimentsBanditNullableConfidenceIntervalV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditNullableConfidenceIntervalV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditNullableConfidenceIntervalV2Visitor {
            type Value = ExperimentsBanditNullableConfidenceIntervalV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lower: Option<f64> = None;
                let mut upper: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lower" => {
                            lower = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "upper" => {
                            upper = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let lower = lower.ok_or_else(|| M::Error::missing_field("lower"))?;
                let upper = upper.ok_or_else(|| M::Error::missing_field("upper"))?;

                let content = ExperimentsBanditNullableConfidenceIntervalV2 {
                    lower,
                    upper,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditNullableConfidenceIntervalV2Visitor)
    }
}
