// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A formula and functions events query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionEventQueryDefinition {
    /// Compute options.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute,
    /// The source organization UUID for cross organization queries. Feature in Private Beta.
    #[serde(rename = "cross_org_uuids")]
    pub cross_org_uuids: Option<Vec<String>>,
    /// Data source for event platform-based queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionEventsDataSource,
    /// Group by options.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBy>>,
    /// An array of index names to query in the stream. Omit or use `[]` to query all indexes at once.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Name of the query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    /// Search options.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch>,
    /// Option for storage location. Feature in Private Beta.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionEventQueryDefinition {
    pub fn new(
        compute: crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute,
        data_source: crate::datadogV1::model::FormulaAndFunctionEventsDataSource,
        name: String,
    ) -> FormulaAndFunctionEventQueryDefinition {
        FormulaAndFunctionEventQueryDefinition {
            compute,
            cross_org_uuids: None,
            data_source,
            group_by: None,
            indexes: None,
            name,
            search: None,
            storage: None,
            _unparsed: false,
        }
    }

    pub fn cross_org_uuids(mut self, value: Vec<String>) -> Self {
        self.cross_org_uuids = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBy>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn search(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch,
    ) -> Self {
        self.search = Some(value);
        self
    }

    pub fn storage(mut self, value: String) -> Self {
        self.storage = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryDefinitionVisitor {
            type Value = FormulaAndFunctionEventQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute,
                > = None;
                let mut cross_org_uuids: Option<Vec<String>> = None;
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventsDataSource,
                > = None;
                let mut group_by: Option<
                    Vec<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBy>,
                > = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut search: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch,
                > = None;
                let mut storage: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::FormulaAndFunctionEventsDataSource::UnparsedObject(_data_source) => {
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
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "storage" => {
                            if v.is_null() {
                                continue;
                            }
                            storage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = FormulaAndFunctionEventQueryDefinition {
                    compute,
                    cross_org_uuids,
                    data_source,
                    group_by,
                    indexes,
                    name,
                    search,
                    storage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryDefinitionVisitor)
    }
}
