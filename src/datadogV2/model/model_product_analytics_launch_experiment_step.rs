// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Experiment linked to the launch. The draft fields are cleared when `experiment_id` is set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsLaunchExperimentStep {
    /// UUID of the linked experiment.
    #[serde(
        rename = "experiment_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_id: Option<Option<String>>,
    /// Hypothesis of a draft experiment.
    #[serde(
        rename = "hypothesis",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub hypothesis: Option<Option<String>>,
    /// Name of a draft experiment.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// Protocol ID of a draft experiment.
    #[serde(
        rename = "protocol_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub protocol_id: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsLaunchExperimentStep {
    pub fn new() -> ProductAnalyticsLaunchExperimentStep {
        ProductAnalyticsLaunchExperimentStep {
            experiment_id: None,
            hypothesis: None,
            name: None,
            protocol_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn experiment_id(mut self, value: Option<String>) -> Self {
        self.experiment_id = Some(value);
        self
    }

    pub fn hypothesis(mut self, value: Option<String>) -> Self {
        self.hypothesis = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = Some(value);
        self
    }

    pub fn protocol_id(mut self, value: Option<String>) -> Self {
        self.protocol_id = Some(value);
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

impl Default for ProductAnalyticsLaunchExperimentStep {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsLaunchExperimentStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsLaunchExperimentStepVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsLaunchExperimentStepVisitor {
            type Value = ProductAnalyticsLaunchExperimentStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut experiment_id: Option<Option<String>> = None;
                let mut hypothesis: Option<Option<String>> = None;
                let mut name: Option<Option<String>> = None;
                let mut protocol_id: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "experiment_id" => {
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hypothesis" => {
                            hypothesis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "protocol_id" => {
                            protocol_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsLaunchExperimentStep {
                    experiment_id,
                    hypothesis,
                    name,
                    protocol_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsLaunchExperimentStepVisitor)
    }
}
