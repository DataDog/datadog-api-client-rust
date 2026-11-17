// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A sequence of events that users follow to use a feature.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsCoreUserJourney {
    /// Order of the steps, as step IDs joined by `->`.
    #[serde(rename = "expression")]
    pub expression: Option<String>,
    /// Relevance score of the journey. Higher is more relevant.
    #[serde(rename = "score")]
    pub score: Option<i64>,
    /// Map of step ID to the event query of the step.
    #[serde(rename = "steps")]
    pub steps: Option<std::collections::BTreeMap<String, String>>,
    /// Number of sessions that followed the journey.
    #[serde(rename = "support")]
    pub support: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsCoreUserJourney {
    pub fn new() -> ProductAnalyticsCoreUserJourney {
        ProductAnalyticsCoreUserJourney {
            expression: None,
            score: None,
            steps: None,
            support: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expression(mut self, value: String) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn steps(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn support(mut self, value: i64) -> Self {
        self.support = Some(value);
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

impl Default for ProductAnalyticsCoreUserJourney {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsCoreUserJourney {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsCoreUserJourneyVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsCoreUserJourneyVisitor {
            type Value = ProductAnalyticsCoreUserJourney;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut expression: Option<String> = None;
                let mut score: Option<i64> = None;
                let mut steps: Option<std::collections::BTreeMap<String, String>> = None;
                let mut support: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "expression" => {
                            if v.is_null() {
                                continue;
                            }
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score" => {
                            if v.is_null() {
                                continue;
                            }
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "support" => {
                            if v.is_null() {
                                continue;
                            }
                            support = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsCoreUserJourney {
                    expression,
                    score,
                    steps,
                    support,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsCoreUserJourneyVisitor)
    }
}
