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
pub struct QueryValueBackendRequest20270101OneOf4460907072 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "comparison")]
    pub comparison: Option<crate::datadogV1_20270101::model::ComparisonRequest>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compliance_findings_query")]
    pub compliance_findings_query: crate::datadogV1_20270101::model::LogQueryDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryValueBackendRequest20270101OneOf4460907072 {
    pub fn new(compliance_findings_query: crate::datadogV1_20270101::model::LogQueryDefinition) -> QueryValueBackendRequest20270101OneOf4460907072 {
        QueryValueBackendRequest20270101OneOf4460907072 {
            aggregator: None,
            comparison: None,
            compliance_findings_query,
            conditional_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1_20270101::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn comparison(mut self, value: crate::datadogV1_20270101::model::ComparisonRequest) -> Self {
        self.comparison = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryValueBackendRequest20270101OneOf4460907072 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryValueBackendRequest20270101OneOf4460907072Visitor;
        impl<'a> Visitor<'a> for QueryValueBackendRequest20270101OneOf4460907072Visitor {
            type Value = QueryValueBackendRequest20270101OneOf4460907072;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1_20270101::model::ScalarAggregator> = None;
                let mut comparison: Option<crate::datadogV1_20270101::model::ComparisonRequest> = None;
                let mut compliance_findings_query: Option<crate::datadogV1_20270101::model::LogQueryDefinition> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV1_20270101::model::ScalarAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "comparison" => {
                            if v.is_null() {
                                continue;
                            }
                            comparison = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "compliance_findings_query" => {
                            compliance_findings_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let compliance_findings_query = compliance_findings_query.ok_or_else(|| M::Error::missing_field("compliance_findings_query"))?;

                let content = QueryValueBackendRequest20270101OneOf4460907072 {
                    aggregator,
                    comparison,
                    compliance_findings_query,
                    conditional_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryValueBackendRequest20270101OneOf4460907072Visitor)
    }
}
