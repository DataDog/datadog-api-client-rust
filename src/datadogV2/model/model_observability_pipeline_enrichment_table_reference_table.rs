// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Uses a Datadog reference table to enrich logs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineEnrichmentTableReferenceTable {
    /// Name of the environment variable or secret that holds the Datadog application key used to access the reference table.
    #[serde(rename = "app_key_key")]
    pub app_key_key: Option<String>,
    /// List of column names to include from the reference table. If not provided, all columns are included.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,
    /// Path to the field in the log event to match against the reference table.
    #[serde(rename = "key_field")]
    pub key_field: String,
    /// The unique identifier of the reference table.
    #[serde(rename = "table_id")]
    pub table_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineEnrichmentTableReferenceTable {
    pub fn new(
        key_field: String,
        table_id: String,
    ) -> ObservabilityPipelineEnrichmentTableReferenceTable {
        ObservabilityPipelineEnrichmentTableReferenceTable {
            app_key_key: None,
            columns: None,
            key_field,
            table_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn app_key_key(mut self, value: String) -> Self {
        self.app_key_key = Some(value);
        self
    }

    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineEnrichmentTableReferenceTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineEnrichmentTableReferenceTableVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineEnrichmentTableReferenceTableVisitor {
            type Value = ObservabilityPipelineEnrichmentTableReferenceTable;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut app_key_key: Option<String> = None;
                let mut columns: Option<Vec<String>> = None;
                let mut key_field: Option<String> = None;
                let mut table_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "app_key_key" => {
                            if v.is_null() {
                                continue;
                            }
                            app_key_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key_field" => {
                            key_field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table_id" => {
                            table_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let key_field = key_field.ok_or_else(|| M::Error::missing_field("key_field"))?;
                let table_id = table_id.ok_or_else(|| M::Error::missing_field("table_id"))?;

                let content = ObservabilityPipelineEnrichmentTableReferenceTable {
                    app_key_key,
                    columns,
                    key_field,
                    table_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineEnrichmentTableReferenceTableVisitor)
    }
}
