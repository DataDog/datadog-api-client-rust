// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions Cloud Cost query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionCloudCostQueryDefinition {
    /// Aggregator used for the request.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::WidgetAggregator>,
    /// The source organization UUID for cross organization queries. Feature in Private Beta.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Data source for Cloud Cost queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Query for Cloud Cost data.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionCloudCostQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource,
        name: String,
        query: String,
    ) -> FormulaAndFunctionCloudCostQueryDefinition {
        FormulaAndFunctionCloudCostQueryDefinition {
            aggregator: None,
            cross_org_uuids: None,
            data_source,
            name,
            query,
            _unparsed: false,
        }
    }

    pub fn aggregator(mut self, value: crate::datadogV1::model::WidgetAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }

    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionCloudCostQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionCloudCostQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionCloudCostQueryDefinitionVisitor {
            type Value = FormulaAndFunctionCloudCostQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1::model::WidgetAggregator> = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
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
                                    crate::datadogV1::model::WidgetAggregator::UnparsedObject(
                                        _aggregator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "cross_org_uuids" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_org_uuids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FormulaAndFunctionCloudCostDataSource::UnparsedObject(_data_source) => {
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
                        &_ => {}
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = FormulaAndFunctionCloudCostQueryDefinition {
                    aggregator,
                    cross_org_uuids,
                    data_source,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionCloudCostQueryDefinitionVisitor)
    }
}
