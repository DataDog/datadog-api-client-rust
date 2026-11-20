// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// The prior the bandit's estimates start from, in the metric's units. Omitted fields keep their current values, or their defaults on create. A change takes effect on the next refresh.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditBayesianPriorInputV2 {
    /// Prior standard deviation of each arm's mean around the shared mean. Must be between 1e-12 and 1e12. Defaults to 100.
    #[serde(rename = "arm_standard_deviation")]
    pub arm_standard_deviation: Option<f64>,
    /// Prior mean of the shared mean of the arms. Its absolute value must be at most 1e12 times `grand_mean_standard_deviation`. Defaults to 0.
    #[serde(rename = "grand_mean")]
    pub grand_mean: Option<f64>,
    /// Prior standard deviation of the shared mean of the arms. Must be between 1e-12 and 1e12, and at most 1e7 times `arm_standard_deviation`. Defaults to 10000.
    #[serde(rename = "grand_mean_standard_deviation")]
    pub grand_mean_standard_deviation: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsBanditBayesianPriorInputV2 {
    pub fn new() -> ExperimentsBanditBayesianPriorInputV2 {
        ExperimentsBanditBayesianPriorInputV2 {
            arm_standard_deviation: None,
            grand_mean: None,
            grand_mean_standard_deviation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn arm_standard_deviation(mut self, value: f64) -> Self {
        self.arm_standard_deviation = Some(value);
        self
    }
    
    pub fn grand_mean(mut self, value: f64) -> Self {
        self.grand_mean = Some(value);
        self
    }
    
    pub fn grand_mean_standard_deviation(mut self, value: f64) -> Self {
        self.grand_mean_standard_deviation = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for ExperimentsBanditBayesianPriorInputV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsBanditBayesianPriorInputV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditBayesianPriorInputV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditBayesianPriorInputV2Visitor {
            type Value = ExperimentsBanditBayesianPriorInputV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arm_standard_deviation: Option<f64> = None;
                let mut grand_mean: Option<f64> = None;
                let mut grand_mean_standard_deviation: Option<f64> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arm_standard_deviation" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            arm_standard_deviation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "grand_mean" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            grand_mean = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "grand_mean_standard_deviation" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            grand_mean_standard_deviation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ExperimentsBanditBayesianPriorInputV2 {
                    arm_standard_deviation,
                    grand_mean,
                    grand_mean_standard_deviation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditBayesianPriorInputV2Visitor)
    }
}
