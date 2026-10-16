// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the batch migration resource to update. At least one attribute is required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringBatchMigrationResourceUpdateAttributes {
    /// The description of the resource.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The source query or natural language description to translate. Must not be blank.
    #[serde(rename = "input")]
    pub input: Option<String>,
    /// The type of the source rule: `spl` for a Splunk SPL search or saved search definition,
    /// `natural_language` for a plain-English description of the behavior to detect, or `kql`
    /// for a Microsoft KQL query. SPL inputs must be self-contained, with search macros expanded.
    #[serde(rename = "inputType")]
    pub input_type:
        Option<crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType>,
    /// The name of the resource.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringBatchMigrationResourceUpdateAttributes {
    pub fn new() -> SecurityMonitoringBatchMigrationResourceUpdateAttributes {
        SecurityMonitoringBatchMigrationResourceUpdateAttributes {
            description: None,
            input: None,
            input_type: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn input(mut self, value: String) -> Self {
        self.input = Some(value);
        self
    }

    pub fn input_type(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringBatchMigrationResourceInputType,
    ) -> Self {
        self.input_type = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for SecurityMonitoringBatchMigrationResourceUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationResourceUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringBatchMigrationResourceUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringBatchMigrationResourceUpdateAttributesVisitor {
            type Value = SecurityMonitoringBatchMigrationResourceUpdateAttributes;

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
                            if v.is_null() {
                                continue;
                            }
                            input = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputType" => {
                            if v.is_null() {
                                continue;
                            }
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = SecurityMonitoringBatchMigrationResourceUpdateAttributes {
                    description,
                    input,
                    input_type,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecurityMonitoringBatchMigrationResourceUpdateAttributesVisitor)
    }
}
