// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Score attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScoreResponseAttributes {
    /// The aggregation type.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Score denominator.
    #[serde(rename = "denominator")]
    pub denominator: i64,
    /// Score numerator.
    #[serde(rename = "numerator")]
    pub numerator: i64,
    /// Calculated score value.
    #[serde(rename = "score")]
    pub score: f64,
    /// Total number of failing rules.
    #[serde(rename = "total_fail")]
    pub total_fail: i64,
    /// Total number of rules with no data.
    #[serde(rename = "total_no_data")]
    pub total_no_data: i64,
    /// Total number of passing rules.
    #[serde(rename = "total_pass")]
    pub total_pass: i64,
    /// Total number of skipped rules.
    #[serde(rename = "total_skip")]
    pub total_skip: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScoreResponseAttributes {
    pub fn new(
        aggregation: String,
        denominator: i64,
        numerator: i64,
        score: f64,
        total_fail: i64,
        total_no_data: i64,
        total_pass: i64,
        total_skip: i64,
    ) -> ScoreResponseAttributes {
        ScoreResponseAttributes {
            aggregation,
            denominator,
            numerator,
            score,
            total_fail,
            total_no_data,
            total_pass,
            total_skip,
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

impl<'de> Deserialize<'de> for ScoreResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScoreResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ScoreResponseAttributesVisitor {
            type Value = ScoreResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut denominator: Option<i64> = None;
                let mut numerator: Option<i64> = None;
                let mut score: Option<f64> = None;
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
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "denominator" => {
                            denominator =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator" => {
                            numerator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "score" => {
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_fail" => {
                            total_fail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_no_data" => {
                            total_no_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_pass" => {
                            total_pass = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_skip" => {
                            total_skip = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let denominator =
                    denominator.ok_or_else(|| M::Error::missing_field("denominator"))?;
                let numerator = numerator.ok_or_else(|| M::Error::missing_field("numerator"))?;
                let score = score.ok_or_else(|| M::Error::missing_field("score"))?;
                let total_fail = total_fail.ok_or_else(|| M::Error::missing_field("total_fail"))?;
                let total_no_data =
                    total_no_data.ok_or_else(|| M::Error::missing_field("total_no_data"))?;
                let total_pass = total_pass.ok_or_else(|| M::Error::missing_field("total_pass"))?;
                let total_skip = total_skip.ok_or_else(|| M::Error::missing_field("total_skip"))?;

                let content = ScoreResponseAttributes {
                    aggregation,
                    denominator,
                    numerator,
                    score,
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

        deserializer.deserialize_any(ScoreResponseAttributesVisitor)
    }
}
