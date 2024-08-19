// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The time-slice condition, composed of 3 parts: 1. the metric timeseries query, 2. the comparator,
/// and 3. the threshold. Optionally, a fourth part, the query interval, can be provided.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOTimeSliceCondition {
    /// The comparator used to compare the SLI value to the threshold.
    #[serde(rename = "comparator")]
    pub comparator: crate::datadogV1::model::SLOTimeSliceComparator,
    /// The queries and formula used to calculate the SLI value.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::SLOTimeSliceQuery,
    /// The interval used when querying data, which defines the size of a time slice.
    /// Two values are allowed: 60 (1 minute) and 300 (5 minutes).
    /// If not provided, the value defaults to 300 (5 minutes).
    #[serde(rename = "query_interval_seconds")]
    pub query_interval_seconds: Option<crate::datadogV1::model::SLOTimeSliceInterval>,
    /// The threshold value to which each SLI value will be compared.
    #[serde(rename = "threshold")]
    pub threshold: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOTimeSliceCondition {
    pub fn new(
        comparator: crate::datadogV1::model::SLOTimeSliceComparator,
        query: crate::datadogV1::model::SLOTimeSliceQuery,
        threshold: f64,
    ) -> SLOTimeSliceCondition {
        SLOTimeSliceCondition {
            comparator,
            query,
            query_interval_seconds: None,
            threshold,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query_interval_seconds(
        mut self,
        value: crate::datadogV1::model::SLOTimeSliceInterval,
    ) -> Self {
        self.query_interval_seconds = Some(value);
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

impl<'de> Deserialize<'de> for SLOTimeSliceCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOTimeSliceConditionVisitor;
        impl<'a> Visitor<'a> for SLOTimeSliceConditionVisitor {
            type Value = SLOTimeSliceCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comparator: Option<crate::datadogV1::model::SLOTimeSliceComparator> = None;
                let mut query: Option<crate::datadogV1::model::SLOTimeSliceQuery> = None;
                let mut query_interval_seconds: Option<
                    crate::datadogV1::model::SLOTimeSliceInterval,
                > = None;
                let mut threshold: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comparator" => {
                            comparator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _comparator) = comparator {
                                match _comparator {
                                    crate::datadogV1::model::SLOTimeSliceComparator::UnparsedObject(_comparator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_interval_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            query_interval_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _query_interval_seconds) = query_interval_seconds {
                                match _query_interval_seconds {
                                    crate::datadogV1::model::SLOTimeSliceInterval::UnparsedObject(_query_interval_seconds) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "threshold" => {
                            threshold = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let comparator = comparator.ok_or_else(|| M::Error::missing_field("comparator"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let threshold = threshold.ok_or_else(|| M::Error::missing_field("threshold"))?;

                let content = SLOTimeSliceCondition {
                    comparator,
                    query,
                    query_interval_seconds,
                    threshold,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOTimeSliceConditionVisitor)
    }
}
