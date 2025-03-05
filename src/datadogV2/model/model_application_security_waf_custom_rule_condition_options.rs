// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options for the operator of this condition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleConditionOptions {
    /// Evaluate the value as case sensitive.
    #[serde(rename = "case_sensitive")]
    pub case_sensitive: Option<bool>,
    /// Only evaluate this condition if the value has a minimum amount of characters.
    #[serde(rename = "min_length")]
    pub min_length: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleConditionOptions {
    pub fn new() -> ApplicationSecurityWafCustomRuleConditionOptions {
        ApplicationSecurityWafCustomRuleConditionOptions {
            case_sensitive: None,
            min_length: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = Some(value);
        self
    }

    pub fn min_length(mut self, value: i64) -> Self {
        self.min_length = Some(value);
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

impl Default for ApplicationSecurityWafCustomRuleConditionOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleConditionOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleConditionOptionsVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleConditionOptionsVisitor {
            type Value = ApplicationSecurityWafCustomRuleConditionOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case_sensitive: Option<bool> = None;
                let mut min_length: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "case_sensitive" => {
                            if v.is_null() {
                                continue;
                            }
                            case_sensitive =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "min_length" => {
                            if v.is_null() {
                                continue;
                            }
                            min_length = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityWafCustomRuleConditionOptions {
                    case_sensitive,
                    min_length,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleConditionOptionsVisitor)
    }
}
