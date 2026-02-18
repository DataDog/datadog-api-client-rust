// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single entry in a lookup table for value transformation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
    /// The substring to match in the source value.
    #[serde(rename = "contains")]
    pub contains: Option<String>,
    /// The exact value to match in the source.
    #[serde(rename = "equals")]
    pub equals: Option<serde_json::Value>,
    /// The source field to match against.
    #[serde(rename = "equals_source")]
    pub equals_source: Option<String>,
    /// A regex pattern to match in the source value.
    #[serde(rename = "matches")]
    pub matches: Option<String>,
    /// A regex pattern that must not match the source value.
    #[serde(rename = "not_matches")]
    pub not_matches: Option<String>,
    /// The value to use when a match is found.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
    pub fn new() -> ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
        ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
            contains: None,
            equals: None,
            equals_source: None,
            matches: None,
            not_matches: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn contains(mut self, value: String) -> Self {
        self.contains = Some(value);
        self
    }

    pub fn equals(mut self, value: serde_json::Value) -> Self {
        self.equals = Some(value);
        self
    }

    pub fn equals_source(mut self, value: String) -> Self {
        self.equals_source = Some(value);
        self
    }

    pub fn matches(mut self, value: String) -> Self {
        self.matches = Some(value);
        self
    }

    pub fn not_matches(mut self, value: String) -> Self {
        self.not_matches = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
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

impl Default for ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineOcsfMappingCustomLookupTableEntryVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineOcsfMappingCustomLookupTableEntryVisitor {
            type Value = ObservabilityPipelineOcsfMappingCustomLookupTableEntry;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut contains: Option<String> = None;
                let mut equals: Option<serde_json::Value> = None;
                let mut equals_source: Option<String> = None;
                let mut matches: Option<String> = None;
                let mut not_matches: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "contains" => {
                            if v.is_null() {
                                continue;
                            }
                            contains = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "equals" => {
                            if v.is_null() {
                                continue;
                            }
                            equals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "equals_source" => {
                            if v.is_null() {
                                continue;
                            }
                            equals_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "matches" => {
                            if v.is_null() {
                                continue;
                            }
                            matches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "not_matches" => {
                            if v.is_null() {
                                continue;
                            }
                            not_matches =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ObservabilityPipelineOcsfMappingCustomLookupTableEntry {
                    contains,
                    equals,
                    equals_source,
                    matches,
                    not_matches,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineOcsfMappingCustomLookupTableEntryVisitor)
    }
}
