// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a scorecard score.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScorecardScoreAttributes {
    /// Dimension to group scores by.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::ScorecardScoresAggregation>,
    /// The denominator used to compute the score ratio.
    #[serde(rename = "denominator")]
    pub denominator: Option<i64>,
    /// The maturity level of the associated rule.
    #[serde(rename = "level")]
    pub level: Option<i64>,
    /// The numerator used to compute the score ratio.
    #[serde(rename = "numerator")]
    pub numerator: Option<i64>,
    /// The computed score ratio (numerator/denominator), from 0 to 1.
    #[serde(rename = "score")]
    pub score: Option<f64>,
    /// The total number of entities evaluated.
    #[serde(rename = "total_entities")]
    pub total_entities: Option<i64>,
    /// The number of rules that failed.
    #[serde(rename = "total_fail")]
    pub total_fail: Option<i64>,
    /// The number of rules with no data.
    #[serde(rename = "total_no_data")]
    pub total_no_data: Option<i64>,
    /// The number of rules that passed.
    #[serde(rename = "total_pass")]
    pub total_pass: Option<i64>,
    /// The number of rules that were skipped.
    #[serde(rename = "total_skip")]
    pub total_skip: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScorecardScoreAttributes {
    pub fn new() -> ScorecardScoreAttributes {
        ScorecardScoreAttributes {
            aggregation: None,
            denominator: None,
            level: None,
            numerator: None,
            score: None,
            total_entities: None,
            total_fail: None,
            total_no_data: None,
            total_pass: None,
            total_skip: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregation(
        mut self,
        value: crate::datadogV2::model::ScorecardScoresAggregation,
    ) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn denominator(mut self, value: i64) -> Self {
        self.denominator = Some(value);
        self
    }

    pub fn level(mut self, value: i64) -> Self {
        self.level = Some(value);
        self
    }

    pub fn numerator(mut self, value: i64) -> Self {
        self.numerator = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn total_entities(mut self, value: i64) -> Self {
        self.total_entities = Some(value);
        self
    }

    pub fn total_fail(mut self, value: i64) -> Self {
        self.total_fail = Some(value);
        self
    }

    pub fn total_no_data(mut self, value: i64) -> Self {
        self.total_no_data = Some(value);
        self
    }

    pub fn total_pass(mut self, value: i64) -> Self {
        self.total_pass = Some(value);
        self
    }

    pub fn total_skip(mut self, value: i64) -> Self {
        self.total_skip = Some(value);
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

impl Default for ScorecardScoreAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScorecardScoreAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScorecardScoreAttributesVisitor;
        impl<'a> Visitor<'a> for ScorecardScoreAttributesVisitor {
            type Value = ScorecardScoreAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<crate::datadogV2::model::ScorecardScoresAggregation> =
                    None;
                let mut denominator: Option<i64> = None;
                let mut level: Option<i64> = None;
                let mut numerator: Option<i64> = None;
                let mut score: Option<f64> = None;
                let mut total_entities: Option<i64> = None;
                let mut total_fail: Option<i64> = None;
                let mut total_no_data: Option<i64> = None;
                let mut total_pass: Option<i64> = None;
                let mut total_skip: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::ScorecardScoresAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "denominator" => {
                            if v.is_null() {
                                continue;
                            }
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "level" => {
                            if v.is_null() {
                                continue;
                            }
                            level = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            if v.is_null() {
                                continue;
                            }
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_entities" => {
                            if v.is_null() {
                                continue;
                            }
                            total_entities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_fail" => {
                            if v.is_null() {
                                continue;
                            }
                            total_fail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_no_data" => {
                            if v.is_null() {
                                continue;
                            }
                            total_no_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_pass" => {
                            if v.is_null() {
                                continue;
                            }
                            total_pass = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_skip" => {
                            if v.is_null() {
                                continue;
                            }
                            total_skip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScorecardScoreAttributes {
                    aggregation,
                    denominator,
                    level,
                    numerator,
                    score,
                    total_entities,
                    total_fail,
                    total_no_data,
                    total_pass,
                    total_skip,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScorecardScoreAttributesVisitor)
    }
}
