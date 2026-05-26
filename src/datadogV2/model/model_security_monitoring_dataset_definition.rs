// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of the dataset. The shape depends on the value of `data_source`.
/// Use `reference_table` or `managed_resource` for a referential dataset, or one of the
/// event platform sources (for example `logs`, `audit`, `events`, `spans`, `rum`) for
/// an event platform dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetDefinition {
    /// For event platform datasets, the list of columns exposed by the dataset.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV2::model::SecurityMonitoringDatasetColumn>>,
    /// The data source backing this dataset definition.
    #[serde(rename = "data_source")]
    pub data_source: String,
    /// For event platform datasets, the list of indexes to query.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The unique name of the dataset. Must start with a lowercase letter and contain only lowercase letters, digits, and underscores (max 255 characters).
    #[serde(rename = "name")]
    pub name: String,
    /// For referential datasets, an optional filter expression applied to the table.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// The search clause applied to an event platform dataset.
    #[serde(rename = "search")]
    pub search: Option<crate::datadogV2::model::SecurityMonitoringDatasetSearch>,
    /// Storage tier the dataset reads from. Applies to event platform datasets.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
    /// For referential datasets, the name of the underlying table.
    #[serde(rename = "table_name")]
    pub table_name: Option<String>,
    /// An optional time window that overrides the default query time range.
    #[serde(rename = "time_window")]
    pub time_window: Option<crate::datadogV2::model::SecurityMonitoringDatasetTimeWindow>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetDefinition {
    pub fn new(data_source: String, name: String) -> SecurityMonitoringDatasetDefinition {
        SecurityMonitoringDatasetDefinition {
            columns: None,
            data_source,
            indexes: None,
            name,
            query_filter: None,
            search: None,
            storage: None,
            table_name: None,
            time_window: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn columns(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringDatasetColumn>,
    ) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }

    pub fn search(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringDatasetSearch,
    ) -> Self {
        self.search = Some(value);
        self
    }

    pub fn storage(mut self, value: String) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn table_name(mut self, value: String) -> Self {
        self.table_name = Some(value);
        self
    }

    pub fn time_window(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringDatasetTimeWindow,
    ) -> Self {
        self.time_window = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringDatasetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringDatasetDefinitionVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringDatasetDefinitionVisitor {
            type Value = SecurityMonitoringDatasetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringDatasetColumn>,
                > = None;
                let mut data_source: Option<String> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut search: Option<crate::datadogV2::model::SecurityMonitoringDatasetSearch> =
                    None;
                let mut storage: Option<String> = None;
                let mut table_name: Option<String> = None;
                let mut time_window: Option<
                    crate::datadogV2::model::SecurityMonitoringDatasetTimeWindow,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "table_name" => {
                            if v.is_null() {
                                continue;
                            }
                            table_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_window" => {
                            if v.is_null() {
                                continue;
                            }
                            time_window =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SecurityMonitoringDatasetDefinition {
                    columns,
                    data_source,
                    indexes,
                    name,
                    query_filter,
                    search,
                    storage,
                    table_name,
                    time_window,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetDefinitionVisitor)
    }
}
