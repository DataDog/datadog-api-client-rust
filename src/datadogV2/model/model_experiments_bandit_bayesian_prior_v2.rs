// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The prior the bandit's estimates start from.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditBayesianPriorV2 {
    /// Prior standard deviation of each arm's mean around the shared mean.
    #[serde(rename = "arm_standard_deviation")]
    pub arm_standard_deviation: f64,
    /// Prior mean of the shared mean of the arms, in the metric's units.
    #[serde(rename = "grand_mean")]
    pub grand_mean: f64,
    /// Prior standard deviation of the shared mean of the arms.
    #[serde(rename = "grand_mean_standard_deviation")]
    pub grand_mean_standard_deviation: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditBayesianPriorV2 {
    pub fn new(
        arm_standard_deviation: f64,
        grand_mean: f64,
        grand_mean_standard_deviation: f64,
    ) -> ExperimentsBanditBayesianPriorV2 {
        ExperimentsBanditBayesianPriorV2 {
            arm_standard_deviation,
            grand_mean,
            grand_mean_standard_deviation,
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

impl<'de> Deserialize<'de> for ExperimentsBanditBayesianPriorV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditBayesianPriorV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditBayesianPriorV2Visitor {
            type Value = ExperimentsBanditBayesianPriorV2;

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
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arm_standard_deviation" => {
                            arm_standard_deviation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grand_mean" => {
                            grand_mean = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grand_mean_standard_deviation" => {
                            grand_mean_standard_deviation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let arm_standard_deviation = arm_standard_deviation
                    .ok_or_else(|| M::Error::missing_field("arm_standard_deviation"))?;
                let grand_mean = grand_mean.ok_or_else(|| M::Error::missing_field("grand_mean"))?;
                let grand_mean_standard_deviation = grand_mean_standard_deviation
                    .ok_or_else(|| M::Error::missing_field("grand_mean_standard_deviation"))?;

                let content = ExperimentsBanditBayesianPriorV2 {
                    arm_standard_deviation,
                    grand_mean,
                    grand_mean_standard_deviation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditBayesianPriorV2Visitor)
    }
}
