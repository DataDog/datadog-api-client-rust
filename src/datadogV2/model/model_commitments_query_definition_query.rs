// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A metric query of a query definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsQueryDefinitionQuery {
    /// The reducer a scalar widget applies to the query.
    #[serde(rename = "aggregator")]
    pub aggregator: crate::datadogV2::model::CommitmentsQueryDefinitionAggregator,
    /// The data source of the query.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::CommitmentsQueryDefinitionDataSource,
    /// The name formulas use to reference the query.
    #[serde(rename = "name")]
    pub name: String,
    /// The metric query.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsQueryDefinitionQuery {
    pub fn new(
        aggregator: crate::datadogV2::model::CommitmentsQueryDefinitionAggregator,
        data_source: crate::datadogV2::model::CommitmentsQueryDefinitionDataSource,
        name: String,
        query: String,
    ) -> CommitmentsQueryDefinitionQuery {
        CommitmentsQueryDefinitionQuery {
            aggregator,
            data_source,
            name,
            query,
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

impl<'de> Deserialize<'de> for CommitmentsQueryDefinitionQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsQueryDefinitionQueryVisitor;
        impl<'a> Visitor<'a> for CommitmentsQueryDefinitionQueryVisitor {
            type Value = CommitmentsQueryDefinitionQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinitionAggregator,
                > = None;
                let mut data_source: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinitionDataSource,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV2::model::CommitmentsQueryDefinitionAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::CommitmentsQueryDefinitionDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let aggregator = aggregator.ok_or_else(|| M::Error::missing_field("aggregator"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = CommitmentsQueryDefinitionQuery {
                    aggregator,
                    data_source,
                    name,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsQueryDefinitionQueryVisitor)
    }
}
