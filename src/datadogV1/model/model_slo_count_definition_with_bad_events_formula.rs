// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// SLO count definition using a bad events formula alongside a good events formula.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCountDefinitionWithBadEventsFormula {
    /// A formula that specifies how to combine the results of multiple queries.
    #[serde(rename = "bad_events_formula")]
    pub bad_events_formula: crate::datadogV1::model::SLOFormula,
    /// A formula that specifies how to combine the results of multiple queries.
    #[serde(rename = "good_events_formula")]
    pub good_events_formula: crate::datadogV1::model::SLOFormula,
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCountDefinitionWithBadEventsFormula {
    pub fn new(
        bad_events_formula: crate::datadogV1::model::SLOFormula,
        good_events_formula: crate::datadogV1::model::SLOFormula,
        queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
    ) -> SLOCountDefinitionWithBadEventsFormula {
        SLOCountDefinitionWithBadEventsFormula {
            bad_events_formula,
            good_events_formula,
            queries,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOCountDefinitionWithBadEventsFormula {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCountDefinitionWithBadEventsFormulaVisitor;
        impl<'a> Visitor<'a> for SLOCountDefinitionWithBadEventsFormulaVisitor {
            type Value = SLOCountDefinitionWithBadEventsFormula;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bad_events_formula: Option<crate::datadogV1::model::SLOFormula> = None;
                let mut good_events_formula: Option<crate::datadogV1::model::SLOFormula> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bad_events_formula" => {
                            bad_events_formula =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "good_events_formula" => {
                            good_events_formula =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let bad_events_formula = bad_events_formula
                    .ok_or_else(|| M::Error::missing_field("bad_events_formula"))?;
                let good_events_formula = good_events_formula
                    .ok_or_else(|| M::Error::missing_field("good_events_formula"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = SLOCountDefinitionWithBadEventsFormula {
                    bad_events_formula,
                    good_events_formula,
                    queries,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCountDefinitionWithBadEventsFormulaVisitor)
    }
}
