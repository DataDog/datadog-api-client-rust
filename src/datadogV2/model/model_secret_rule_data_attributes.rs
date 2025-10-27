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
pub struct SecretRuleDataAttributes {
    #[serde(rename = "default_included_keywords")]
    pub default_included_keywords: Option<Vec<String>>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "license")]
    pub license: Option<String>,
    #[serde(rename = "match_validation")]
    pub match_validation: Option<crate::datadogV2::model::SecretRuleDataAttributesMatchValidation>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    #[serde(rename = "priority")]
    pub priority: Option<String>,
    #[serde(rename = "sds_id")]
    pub sds_id: Option<String>,
    #[serde(rename = "validators")]
    pub validators: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecretRuleDataAttributes {
    pub fn new() -> SecretRuleDataAttributes {
        SecretRuleDataAttributes {
            default_included_keywords: None,
            description: None,
            license: None,
            match_validation: None,
            name: None,
            pattern: None,
            priority: None,
            sds_id: None,
            validators: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_included_keywords(mut self, value: Vec<String>) -> Self {
        self.default_included_keywords = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn license(mut self, value: String) -> Self {
        self.license = Some(value);
        self
    }

    pub fn match_validation(
        mut self,
        value: crate::datadogV2::model::SecretRuleDataAttributesMatchValidation,
    ) -> Self {
        self.match_validation = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn priority(mut self, value: String) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn sds_id(mut self, value: String) -> Self {
        self.sds_id = Some(value);
        self
    }

    pub fn validators(mut self, value: Vec<String>) -> Self {
        self.validators = Some(value);
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

impl Default for SecretRuleDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecretRuleDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecretRuleDataAttributesVisitor;
        impl<'a> Visitor<'a> for SecretRuleDataAttributesVisitor {
            type Value = SecretRuleDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_included_keywords: Option<Vec<String>> = None;
                let mut description: Option<String> = None;
                let mut license: Option<String> = None;
                let mut match_validation: Option<
                    crate::datadogV2::model::SecretRuleDataAttributesMatchValidation,
                > = None;
                let mut name: Option<String> = None;
                let mut pattern: Option<String> = None;
                let mut priority: Option<String> = None;
                let mut sds_id: Option<String> = None;
                let mut validators: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default_included_keywords" => {
                            if v.is_null() {
                                continue;
                            }
                            default_included_keywords =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "license" => {
                            if v.is_null() {
                                continue;
                            }
                            license = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "match_validation" => {
                            if v.is_null() {
                                continue;
                            }
                            match_validation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_id" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validators" => {
                            if v.is_null() {
                                continue;
                            }
                            validators = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecretRuleDataAttributes {
                    default_included_keywords,
                    description,
                    license,
                    match_validation,
                    name,
                    pattern,
                    priority,
                    sds_id,
                    validators,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecretRuleDataAttributesVisitor)
    }
}
