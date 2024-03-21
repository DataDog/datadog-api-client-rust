// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The time-slice condition, composed of 3 parts: 1. the metric timeseries query, 2. the comparator,
/// and 3. the threshold.
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
    /// The threshold value to which each SLI value will be compared.
    #[serde(rename = "threshold")]
    pub threshold: f64,
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
            threshold,
            _unparsed: false,
        }
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
                let mut threshold: Option<f64> = None;
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
                        "threshold" => {
                            threshold = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let comparator = comparator.ok_or_else(|| M::Error::missing_field("comparator"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let threshold = threshold.ok_or_else(|| M::Error::missing_field("threshold"))?;

                let content = SLOTimeSliceCondition {
                    comparator,
                    query,
                    threshold,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOTimeSliceConditionVisitor)
    }
}
