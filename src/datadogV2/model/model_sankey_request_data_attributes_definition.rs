// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SankeyRequestDataAttributesDefinition {
    #[serde(rename = "entries_per_step")]
    pub entries_per_step: Option<i64>,
    #[serde(rename = "number_of_steps")]
    pub number_of_steps: Option<i64>,
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(rename = "target")]
    pub target: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SankeyRequestDataAttributesDefinition {
    pub fn new() -> SankeyRequestDataAttributesDefinition {
        SankeyRequestDataAttributesDefinition {
            entries_per_step: None,
            number_of_steps: None,
            source: None,
            target: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn entries_per_step(mut self, value: i64) -> Self {
        self.entries_per_step = Some(value);
        self
    }

    pub fn number_of_steps(mut self, value: i64) -> Self {
        self.number_of_steps = Some(value);
        self
    }

    pub fn source(mut self, value: String) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target(mut self, value: String) -> Self {
        self.target = Some(value);
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

impl Default for SankeyRequestDataAttributesDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SankeyRequestDataAttributesDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SankeyRequestDataAttributesDefinitionVisitor;
        impl<'a> Visitor<'a> for SankeyRequestDataAttributesDefinitionVisitor {
            type Value = SankeyRequestDataAttributesDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entries_per_step: Option<i64> = None;
                let mut number_of_steps: Option<i64> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entries_per_step" => {
                            if v.is_null() {
                                continue;
                            }
                            entries_per_step =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number_of_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SankeyRequestDataAttributesDefinition {
                    entries_per_step,
                    number_of_steps,
                    source,
                    target,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SankeyRequestDataAttributesDefinitionVisitor)
    }
}
