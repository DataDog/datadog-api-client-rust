// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Lookup table configuration for mapping source values to destination values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMappingCustomLookup {
    /// The default value to use if no lookup match is found.
    #[serde(rename = "default")]
    pub default: Option<serde_json::Value>,
    /// A list of lookup table entries for value transformation.
    #[serde(rename = "table")]
    pub table: Option<
        Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookupTableEntry>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMappingCustomLookup {
    pub fn new() -> ObservabilityPipelineOcsfMappingCustomLookup {
        ObservabilityPipelineOcsfMappingCustomLookup {
            default: None,
            table: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn table(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookupTableEntry>,
    ) -> Self {
        self.table = Some(value);
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

impl Default for ObservabilityPipelineOcsfMappingCustomLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingCustomLookup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMappingCustomLookupVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMappingCustomLookupVisitor {
            type Value = ObservabilityPipelineOcsfMappingCustomLookup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default: Option<serde_json::Value> = None;
                let mut table: Option<Vec<crate::datadogV2::model::ObservabilityPipelineOcsfMappingCustomLookupTableEntry>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default" => {
                            if v.is_null() {
                                continue;
                            }
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table" => {
                            if v.is_null() {
                                continue;
                            }
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineOcsfMappingCustomLookup {
                    default,
                    table,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMappingCustomLookupVisitor)
    }
}
