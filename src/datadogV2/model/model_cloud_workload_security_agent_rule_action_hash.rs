// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Hash file specified by the field attribute
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleActionHash {
    /// The field of the hash action
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The maximum size of the files to hash, in bytes.
    #[serde(rename = "max_file_size")]
    pub max_file_size: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CloudWorkloadSecurityAgentRuleActionHash {
    pub fn new() -> CloudWorkloadSecurityAgentRuleActionHash {
        CloudWorkloadSecurityAgentRuleActionHash {
            field: None,
            max_file_size: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }
    
    pub fn max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for CloudWorkloadSecurityAgentRuleActionHash {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleActionHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleActionHashVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleActionHashVisitor {
            type Value = CloudWorkloadSecurityAgentRuleActionHash;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut field: Option<String> = None;
                let mut max_file_size: Option<i64> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "max_file_size" => {
                            if v.is_null() {
                                continue;
                            }
                            max_file_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = CloudWorkloadSecurityAgentRuleActionHash {
                    field,
                    max_file_size,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleActionHashVisitor)
    }
}
