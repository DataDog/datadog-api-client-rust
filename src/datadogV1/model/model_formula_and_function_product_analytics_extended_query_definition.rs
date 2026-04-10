// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions Product Analytics Extended query for advanced analytics features.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
    /// Compute configuration for Product Analytics Extended queries.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1::model::ProductAnalyticsExtendedCompute,
    /// Data source for Product Analytics Extended queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedDataSource,
    /// Group by configuration.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::ProductAnalyticsExtendedGroupBy>>,
    /// Event indexes to query.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionIndexesItems>>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Product Analytics event query.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ProductAnalyticsBaseQuery,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
    pub fn new(
        compute: crate::datadogV1::model::ProductAnalyticsExtendedCompute,
        data_source: crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedDataSource,
        name: String,
        query: crate::datadogV1::model::ProductAnalyticsBaseQuery,
    ) -> FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
        FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
            compute,
            data_source,
            group_by: None,
            indexes: None,
            name,
            query,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::ProductAnalyticsExtendedGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(
        mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionIndexesItems>,
    ) -> Self {
        self.indexes = Some(value);
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

impl<'de> Deserialize<'de> for FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionVisitor {
            type Value = FormulaAndFunctionProductAnalyticsExtendedQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::ProductAnalyticsExtendedCompute> =
                    None;
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedDataSource,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::ProductAnalyticsExtendedGroupBy>,
                > = None;
                let mut indexes: Option<Vec<crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionIndexesItems>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<crate::datadogV1::model::ProductAnalyticsBaseQuery> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FormulaAndFunctionProductAnalyticsExtendedDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = FormulaAndFunctionProductAnalyticsExtendedQueryDefinition {
                    compute,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(FormulaAndFunctionProductAnalyticsExtendedQueryDefinitionVisitor)
    }
}
