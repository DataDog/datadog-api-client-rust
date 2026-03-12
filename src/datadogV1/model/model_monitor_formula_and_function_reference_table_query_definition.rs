// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A reference table query for use in aggregate queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionReferenceTableQueryDefinition {
    /// List of columns to retrieve from the reference table.
    #[serde(rename = "columns")]
    pub columns:
        Option<Vec<crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableColumn>>,
    /// Data source for reference table queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableDataSource,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Optional filter expression for the reference table query.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Name of the reference table.
    #[serde(rename = "table_name")]
    pub table_name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionReferenceTableQueryDefinition {
    pub fn new(
        data_source: crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableDataSource,
        table_name: String,
    ) -> MonitorFormulaAndFunctionReferenceTableQueryDefinition {
        MonitorFormulaAndFunctionReferenceTableQueryDefinition {
            columns: None,
            data_source,
            name: None,
            query_filter: None,
            table_name,
            _unparsed: false,
        }
    }

    pub fn columns(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableColumn>,
    ) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionReferenceTableQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionReferenceTableQueryDefinitionVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionReferenceTableQueryDefinitionVisitor {
            type Value = MonitorFormulaAndFunctionReferenceTableQueryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<
                    Vec<crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableColumn>,
                > = None;
                let mut data_source: Option<
                    crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableDataSource,
                > = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::MonitorFormulaAndFunctionReferenceTableDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_name" => {
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let table_name = table_name.ok_or_else(|| M::Error::missing_field("table_name"))?;

                let content = MonitorFormulaAndFunctionReferenceTableQueryDefinition {
                    columns,
                    data_source,
                    name,
                    query_filter,
                    table_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionReferenceTableQueryDefinitionVisitor)
    }
}
