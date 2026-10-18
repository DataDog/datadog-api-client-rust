// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A source rule to translate into a Cloud SIEM detection rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationResourceCreateItem {
    /// The description of the resource.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The source query or natural language description to translate. Must not be blank.
    #[serde(rename = "input")]
    pub input: String,
    /// The type of the source rule: `spl` for a Splunk SPL search or saved search definition,
    /// `natural_language` for a plain-English description of the behavior to detect, or `kql`
    /// for a Microsoft KQL query. SPL inputs must be self-contained, with search macros expanded.
    #[serde(rename = "inputType")]
    pub input_type: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
    /// The name of the resource.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringBatchMigrationResourceCreateItem {
    pub fn new(
        input: String,
        input_type: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
    ) -> SecurityMonitoringBatchMigrationResourceCreateItem {
        SecurityMonitoringBatchMigrationResourceCreateItem {
            description: None,
            input,
            input_type,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationResourceCreateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationResourceCreateItemVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationResourceCreateItemVisitor {
            type Value = SecurityMonitoringBatchMigrationResourceCreateItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut input: Option<String> = None;
                let mut input_type: Option<
                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
                > = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "input" => {
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputType" => {
                            input_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _input_type) = input_type {
                                match _input_type {
                                    crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType::UnparsedObject(_input_type) => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let input = input.ok_or_else(|| M::Error::missing_field("input"))?;
                let input_type = input_type.ok_or_else(|| M::Error::missing_field("input_type"))?;

                let content = SecurityMonitoringBatchMigrationResourceCreateItem {
                    description,
                    input,
                    input_type,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringBatchMigrationResourceCreateItemVisitor)
    }
}
