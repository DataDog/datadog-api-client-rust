// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing a timeseries formula request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV2::model::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// A time interval in milliseconds.
    /// May be overridden by a larger interval if the query would result in
    /// too many points for the specified timeframe.
    /// Defaults to a reasonable interval for the given timeframe.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::TimeseriesQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesFormulaRequestAttributes {
    pub fn new(
        from: i64,
        queries: Vec<crate::datadogV2::model::TimeseriesQuery>,
        to: i64,
    ) -> TimeseriesFormulaRequestAttributes {
        TimeseriesFormulaRequestAttributes {
            formulas: None,
            from,
            interval: None,
            queries,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV2::model::QueryFormula>) -> Self {
        self.formulas = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
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

impl<'de> Deserialize<'de> for TimeseriesFormulaRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesFormulaRequestAttributesVisitor;
        impl<'a> Visitor<'a> for TimeseriesFormulaRequestAttributesVisitor {
            type Value = TimeseriesFormulaRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut formulas: Option<Vec<crate::datadogV2::model::QueryFormula>> = None;
                let mut from: Option<i64> = None;
                let mut interval: Option<i64> = None;
                let mut queries: Option<Vec<crate::datadogV2::model::TimeseriesQuery>> = None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "formulas" => {
                            if v.is_null() {
                                continue;
                            }
                            formulas = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval" => {
                            if v.is_null() {
                                continue;
                            }
                            interval = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = TimeseriesFormulaRequestAttributes {
                    formulas,
                    from,
                    interval,
                    queries,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesFormulaRequestAttributesVisitor)
    }
}
