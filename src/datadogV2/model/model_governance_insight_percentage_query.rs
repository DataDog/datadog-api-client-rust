// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A percentage query that computes an insight value as a ratio of two metric queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceInsightPercentageQuery {
    /// A metric query used to compute an insight value.
    #[serde(rename = "denominator_query")]
    pub denominator_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
    /// A metric query used to compute an insight value.
    #[serde(rename = "numerator_query")]
    pub numerator_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceInsightPercentageQuery {
    pub fn new(
        denominator_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
        numerator_query: crate::datadogV2::model::GovernanceInsightMetricQuery,
    ) -> GovernanceInsightPercentageQuery {
        GovernanceInsightPercentageQuery {
            denominator_query,
            numerator_query,
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

impl<'de> Deserialize<'de> for GovernanceInsightPercentageQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceInsightPercentageQueryVisitor;
        impl<'a> Visitor<'a> for GovernanceInsightPercentageQueryVisitor {
            type Value = GovernanceInsightPercentageQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denominator_query: Option<
                    crate::datadogV2::model::GovernanceInsightMetricQuery,
                > = None;
                let mut numerator_query: Option<
                    crate::datadogV2::model::GovernanceInsightMetricQuery,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denominator_query" => {
                            denominator_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator_query" => {
                            numerator_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let denominator_query = denominator_query
                    .ok_or_else(|| M::Error::missing_field("denominator_query"))?;
                let numerator_query =
                    numerator_query.ok_or_else(|| M::Error::missing_field("numerator_query"))?;

                let content = GovernanceInsightPercentageQuery {
                    denominator_query,
                    numerator_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceInsightPercentageQueryVisitor)
    }
}
