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
pub struct SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
    #[serde(rename = "end")]
    pub end: Option<i64>,
    #[serde(rename = "start")]
    pub start: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
    pub fn new() -> SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
        SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
            end: None,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
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

impl Default for SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItemsVisitor;
        impl<'a> Visitor<'a> for SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItemsVisitor {
            type Value = SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<i64> = None;
                let mut start: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItems {
                    end,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SecretRuleDataAttributesMatchValidationValidHttpStatusCodeItemsVisitor)
    }
}
