// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The scope of the WAF custom rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleConditionParameters {
    /// Identifier of a list of data from the denylist. Can only be used as substitution from the list parameter.
    #[serde(rename = "data")]
    pub data: Option<String>,
    /// List of inputs on which at least one should match with the given operator.
    #[serde(rename = "inputs")]
    pub inputs: Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInput>,
    /// List of value to use with the condition. Only used with the phrase_match, !phrase_match, exact_match and
    /// !exact_match operator.
    #[serde(rename = "list")]
    pub list: Option<Vec<String>>,
    /// Options for the operator of this condition.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOptions>,
    /// Regex to use with the condition. Only used with match_regex and !match_regex operator.
    #[serde(rename = "regex")]
    pub regex: Option<String>,
    /// Store the captured value in the specified tag name. Only used with the capture_data operator.
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleConditionParameters {
    pub fn new(
        inputs: Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInput>,
    ) -> ApplicationSecurityWafCustomRuleConditionParameters {
        ApplicationSecurityWafCustomRuleConditionParameters {
            data: None,
            inputs,
            list: None,
            options: None,
            regex: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: String) -> Self {
        self.data = Some(value);
        self
    }

    pub fn list(mut self, value: Vec<String>) -> Self {
        self.list = Some(value);
        self
    }

    pub fn options(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOptions,
    ) -> Self {
        self.options = Some(value);
        self
    }

    pub fn regex(mut self, value: String) -> Self {
        self.regex = Some(value);
        self
    }

    pub fn value(mut self, value: String) -> Self {
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleConditionParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleConditionParametersVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleConditionParametersVisitor {
            type Value = ApplicationSecurityWafCustomRuleConditionParameters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<String> = None;
                let mut inputs: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionInput>,
                > = None;
                let mut list: Option<Vec<String>> = None;
                let mut options: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleConditionOptions,
                > = None;
                let mut regex: Option<String> = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "list" => {
                            if v.is_null() {
                                continue;
                            }
                            list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "regex" => {
                            if v.is_null() {
                                continue;
                            }
                            regex = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;

                let content = ApplicationSecurityWafCustomRuleConditionParameters {
                    data,
                    inputs,
                    list,
                    options,
                    regex,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleConditionParametersVisitor)
    }
}
