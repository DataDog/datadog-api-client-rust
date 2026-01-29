// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of a dataset, including its data source, name, indexes, and columns.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringDatasetDefinition {
    /// The columns in the dataset.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV2::model::SecurityMonitoringDatasetDefinitionColumn>,
    /// The data source for the dataset.
    #[serde(rename = "data_source")]
    pub data_source: String,
    /// The indexes to use for the dataset.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// The name of the dataset. Must start with a letter, contain only lowercase letters, numbers, and underscores, and be at most 255 characters long.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringDatasetDefinition {
    pub fn new(
        columns: Vec<crate::datadogV2::model::SecurityMonitoringDatasetDefinitionColumn>,
        data_source: String,
        name: String,
    ) -> SecurityMonitoringDatasetDefinition {
        SecurityMonitoringDatasetDefinition {
            columns,
            data_source,
            indexes: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
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
                    Vec<crate::datadogV2::model::SecurityMonitoringDatasetDefinitionColumn>,
                > = None;
                let mut data_source: Option<String> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SecurityMonitoringDatasetDefinition {
                    columns,
                    data_source,
                    indexes,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringDatasetDefinitionVisitor)
    }
}
