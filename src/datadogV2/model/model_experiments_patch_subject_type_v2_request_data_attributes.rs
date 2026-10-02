// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Fields supplied to update the subject type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchSubjectTypeV2RequestDataAttributes {
    /// Metadata retained for resources imported from another system.
    #[serde(rename = "migration_metadata")]
    pub migration_metadata: Option<serde_json::Value>,
    /// Display name of the subject type.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Product Analytics attribute used to identify subjects of this type.
    #[serde(rename = "product_analytics_attribute")]
    pub product_analytics_attribute: Option<String>,
    /// Warehouse columns that identify subjects of this type.
    #[serde(rename = "warehouse_column_names")]
    pub warehouse_column_names: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchSubjectTypeV2RequestDataAttributes {
    pub fn new() -> ExperimentsPatchSubjectTypeV2RequestDataAttributes {
        ExperimentsPatchSubjectTypeV2RequestDataAttributes {
            migration_metadata: None,
            name: None,
            product_analytics_attribute: None,
            warehouse_column_names: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn migration_metadata(mut self, value: serde_json::Value) -> Self {
        self.migration_metadata = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn product_analytics_attribute(mut self, value: String) -> Self {
        self.product_analytics_attribute = Some(value);
        self
    }

    pub fn warehouse_column_names(mut self, value: Vec<String>) -> Self {
        self.warehouse_column_names = Some(value);
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

impl Default for ExperimentsPatchSubjectTypeV2RequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsPatchSubjectTypeV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchSubjectTypeV2RequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchSubjectTypeV2RequestDataAttributesVisitor {
            type Value = ExperimentsPatchSubjectTypeV2RequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut migration_metadata: Option<serde_json::Value> = None;
                let mut name: Option<String> = None;
                let mut product_analytics_attribute: Option<String> = None;
                let mut warehouse_column_names: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "migration_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            migration_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_attribute" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "warehouse_column_names" => {
                            if v.is_null() {
                                continue;
                            }
                            warehouse_column_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsPatchSubjectTypeV2RequestDataAttributes {
                    migration_metadata,
                    name,
                    product_analytics_attribute,
                    warehouse_column_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchSubjectTypeV2RequestDataAttributesVisitor)
    }
}
