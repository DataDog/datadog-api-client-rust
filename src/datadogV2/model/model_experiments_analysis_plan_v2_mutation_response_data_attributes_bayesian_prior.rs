// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Parameters of the prior distribution used for Bayesian analysis.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
    /// Degrees of freedom of the prior distribution.
    #[serde(
        rename = "degrees_of_freedom",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub degrees_of_freedom: Option<Option<f64>>,
    /// Standard deviation of the prior distribution.
    #[serde(rename = "standard_deviation")]
    pub standard_deviation: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
    pub fn new() -> ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
        ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
            degrees_of_freedom: None,
            standard_deviation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn degrees_of_freedom(mut self, value: Option<f64>) -> Self {
        self.degrees_of_freedom = Some(value);
        self
    }

    pub fn standard_deviation(mut self, value: f64) -> Self {
        self.standard_deviation = Some(value);
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

impl Default for ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPriorVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPriorVisitor
        {
            type Value = ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut degrees_of_freedom: Option<Option<f64>> = None;
                let mut standard_deviation: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "degrees_of_freedom" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            degrees_of_freedom =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "standard_deviation" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            standard_deviation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content =
                    ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPrior {
                        degrees_of_freedom,
                        standard_deviation,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsAnalysisPlanV2MutationResponseDataAttributesBayesianPriorVisitor,
        )
    }
}
