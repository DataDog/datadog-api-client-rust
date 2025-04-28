// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the user who created the Agent rule
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudWorkloadSecurityAgentRuleCreatorAttributes {
    /// The handle of the user
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// The name of the user
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudWorkloadSecurityAgentRuleCreatorAttributes {
    pub fn new() -> CloudWorkloadSecurityAgentRuleCreatorAttributes {
        CloudWorkloadSecurityAgentRuleCreatorAttributes {
            handle: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn handle(mut self, value: String) -> Self {
        self.handle = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
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

impl Default for CloudWorkloadSecurityAgentRuleCreatorAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CloudWorkloadSecurityAgentRuleCreatorAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudWorkloadSecurityAgentRuleCreatorAttributesVisitor;
        impl<'a> Visitor<'a> for CloudWorkloadSecurityAgentRuleCreatorAttributesVisitor {
            type Value = CloudWorkloadSecurityAgentRuleCreatorAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut handle: Option<String> = None;
                let mut name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = CloudWorkloadSecurityAgentRuleCreatorAttributes {
                    handle,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudWorkloadSecurityAgentRuleCreatorAttributesVisitor)
    }
}
