// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A count-based (metric) SLI specification, composed of three parts: the good events formula, the total events formula,
/// and the underlying queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOCountDefinition {
    /// A formula that specifies how to combine the results of multiple queries.
    #[serde(rename = "good_events_formula")]
    pub good_events_formula: crate::datadogV1::model::SLOFormula,
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
    /// A formula that specifies how to combine the results of multiple queries.
    #[serde(rename = "total_events_formula")]
    pub total_events_formula: crate::datadogV1::model::SLOFormula,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOCountDefinition {
    pub fn new(
        good_events_formula: crate::datadogV1::model::SLOFormula,
        queries: Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
        total_events_formula: crate::datadogV1::model::SLOFormula,
    ) -> SLOCountDefinition {
        SLOCountDefinition {
            good_events_formula,
            queries,
            total_events_formula,
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

impl<'de> Deserialize<'de> for SLOCountDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOCountDefinitionVisitor;
        impl<'a> Visitor<'a> for SLOCountDefinitionVisitor {
            type Value = SLOCountDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut good_events_formula: Option<crate::datadogV1::model::SLOFormula> = None;
                let mut queries: Option<
                    Vec<crate::datadogV1::model::SLODataSourceQueryDefinition>,
                > = None;
                let mut total_events_formula: Option<crate::datadogV1::model::SLOFormula> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "good_events_formula" => {
                            good_events_formula =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_events_formula" => {
                            total_events_formula =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let good_events_formula = good_events_formula
                    .ok_or_else(|| M::Error::missing_field("good_events_formula"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let total_events_formula = total_events_formula
                    .ok_or_else(|| M::Error::missing_field("total_events_formula"))?;

                let content = SLOCountDefinition {
                    good_events_formula,
                    queries,
                    total_events_formula,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOCountDefinitionVisitor)
    }
}
