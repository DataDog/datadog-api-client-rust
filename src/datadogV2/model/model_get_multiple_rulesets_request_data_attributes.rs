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
pub struct GetMultipleRulesetsRequestDataAttributes {
    #[serde(rename = "include_testing_rules")]
    pub include_testing_rules: Option<bool>,
    #[serde(rename = "include_tests")]
    pub include_tests: Option<bool>,
    #[serde(rename = "rulesets")]
    pub rulesets: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetMultipleRulesetsRequestDataAttributes {
    pub fn new() -> GetMultipleRulesetsRequestDataAttributes {
        GetMultipleRulesetsRequestDataAttributes {
            include_testing_rules: None,
            include_tests: None,
            rulesets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_testing_rules(mut self, value: bool) -> Self {
        self.include_testing_rules = Some(value);
        self
    }

    pub fn include_tests(mut self, value: bool) -> Self {
        self.include_tests = Some(value);
        self
    }

    pub fn rulesets(mut self, value: Vec<String>) -> Self {
        self.rulesets = Some(value);
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

impl Default for GetMultipleRulesetsRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetMultipleRulesetsRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMultipleRulesetsRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for GetMultipleRulesetsRequestDataAttributesVisitor {
            type Value = GetMultipleRulesetsRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_testing_rules: Option<bool> = None;
                let mut include_tests: Option<bool> = None;
                let mut rulesets: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_testing_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            include_testing_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_tests" => {
                            if v.is_null() {
                                continue;
                            }
                            include_tests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rulesets" => {
                            if v.is_null() {
                                continue;
                            }
                            rulesets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetMultipleRulesetsRequestDataAttributes {
                    include_testing_rules,
                    include_tests,
                    rulesets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetMultipleRulesetsRequestDataAttributesVisitor)
    }
}
