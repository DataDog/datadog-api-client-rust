// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StructuredAnalysisAggregation {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1_20270101::model::StructuredAnalysisCompute>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1_20270101::model::StructuredAnalysisGroupBy>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1_20270101::model::AnalysisAggregationSort>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "time_aggregation")]
    pub time_aggregation: Option<crate::datadogV1_20270101::model::StructuredAnalysisTimeAggregation>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1_20270101::model::StructuredAnalysisAggregationType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl StructuredAnalysisAggregation {
    pub fn new(type_: crate::datadogV1_20270101::model::StructuredAnalysisAggregationType) -> StructuredAnalysisAggregation {
        StructuredAnalysisAggregation {
            compute: None,
            group_by: None,
            limit: None,
            sort: None,
            time_aggregation: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn compute(mut self, value: Vec<crate::datadogV1_20270101::model::StructuredAnalysisCompute>) -> Self {
        self.compute = Some(value);
        self
    }
    
    pub fn group_by(mut self, value: Vec<crate::datadogV1_20270101::model::StructuredAnalysisGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn sort(mut self, value: crate::datadogV1_20270101::model::AnalysisAggregationSort) -> Self {
        self.sort = Some(value);
        self
    }
    
    pub fn time_aggregation(mut self, value: crate::datadogV1_20270101::model::StructuredAnalysisTimeAggregation) -> Self {
        self.time_aggregation = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for StructuredAnalysisAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StructuredAnalysisAggregationVisitor;
        impl<'a> Visitor<'a> for StructuredAnalysisAggregationVisitor {
            type Value = StructuredAnalysisAggregation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<Vec<crate::datadogV1_20270101::model::StructuredAnalysisCompute>> = None;
                let mut group_by: Option<Vec<crate::datadogV1_20270101::model::StructuredAnalysisGroupBy>> = None;
                let mut limit: Option<f64> = None;
                let mut sort: Option<crate::datadogV1_20270101::model::AnalysisAggregationSort> = None;
                let mut time_aggregation: Option<crate::datadogV1_20270101::model::StructuredAnalysisTimeAggregation> = None;
                let mut type_: Option<crate::datadogV1_20270101::model::StructuredAnalysisAggregationType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "time_aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            time_aggregation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1_20270101::model::StructuredAnalysisAggregationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = StructuredAnalysisAggregation {
                    compute,
                    group_by,
                    limit,
                    sort,
                    time_aggregation,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StructuredAnalysisAggregationVisitor)
    }
}
