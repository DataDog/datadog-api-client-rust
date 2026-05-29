// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Explicit tag operations for updating records. Operations are applied in order, Remove then Add then Set. `set` is the final override; if specified, the result of `remove` and `add` is discarded.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetRecordTagOperations {
    /// List of tag strings.
    #[serde(rename = "add")]
    pub add: Option<Vec<String>>,
    /// List of tag strings.
    #[serde(rename = "remove")]
    pub remove: Option<Vec<String>>,
    /// List of tag strings.
    #[serde(rename = "set")]
    pub set: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetRecordTagOperations {
    pub fn new() -> LLMObsDatasetRecordTagOperations {
        LLMObsDatasetRecordTagOperations {
            add: None,
            remove: None,
            set: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn add(mut self, value: Vec<String>) -> Self {
        self.add = Some(value);
        self
    }

    pub fn remove(mut self, value: Vec<String>) -> Self {
        self.remove = Some(value);
        self
    }

    pub fn set(mut self, value: Vec<String>) -> Self {
        self.set = Some(value);
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

impl Default for LLMObsDatasetRecordTagOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LLMObsDatasetRecordTagOperations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetRecordTagOperationsVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetRecordTagOperationsVisitor {
            type Value = LLMObsDatasetRecordTagOperations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut add: Option<Vec<String>> = None;
                let mut remove: Option<Vec<String>> = None;
                let mut set: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "add" => {
                            if v.is_null() {
                                continue;
                            }
                            add = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "remove" => {
                            if v.is_null() {
                                continue;
                            }
                            remove = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "set" => {
                            if v.is_null() {
                                continue;
                            }
                            set = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = LLMObsDatasetRecordTagOperations {
                    add,
                    remove,
                    set,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetRecordTagOperationsVisitor)
    }
}
