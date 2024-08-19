// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The suppression rule properties to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSuppressionUpdateAttributes {
    /// An exclusion query on the input data of the security rules, which could be logs, Agent events, or other types of data based on the security rule. Events matching this query are ignored by any detection rules referenced in the suppression rule.
    #[serde(rename = "data_exclusion_query")]
    pub data_exclusion_query: Option<String>,
    /// A description for the suppression rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the suppression rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// A Unix millisecond timestamp giving an expiration date for the suppression rule. After this date, it won't suppress signals anymore. If unset, the expiration date of the suppression rule is left untouched. If set to `null`, the expiration date is removed.
    #[serde(
        rename = "expiration_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub expiration_date: Option<Option<i64>>,
    /// The name of the suppression rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The rule query of the suppression rule, with the same syntax as the search bar for detection rules.
    #[serde(rename = "rule_query")]
    pub rule_query: Option<String>,
    /// The suppression query of the suppression rule. If a signal matches this query, it is suppressed and not triggered. Same syntax as the queries to search signals in the signal explorer.
    #[serde(rename = "suppression_query")]
    pub suppression_query: Option<String>,
    /// The current version of the suppression. This is optional, but it can help prevent concurrent modifications.
    #[serde(rename = "version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSuppressionUpdateAttributes {
    pub fn new() -> SecurityMonitoringSuppressionUpdateAttributes {
        SecurityMonitoringSuppressionUpdateAttributes {
            data_exclusion_query: None,
            description: None,
            enabled: None,
            expiration_date: None,
            name: None,
            rule_query: None,
            suppression_query: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_exclusion_query(mut self, value: String) -> Self {
        self.data_exclusion_query = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: Option<i64>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn rule_query(mut self, value: String) -> Self {
        self.rule_query = Some(value);
        self
    }

    pub fn suppression_query(mut self, value: String) -> Self {
        self.suppression_query = Some(value);
        self
    }

    pub fn version(mut self, value: i32) -> Self {
        self.version = Some(value);
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

impl Default for SecurityMonitoringSuppressionUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSuppressionUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSuppressionUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSuppressionUpdateAttributesVisitor {
            type Value = SecurityMonitoringSuppressionUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_exclusion_query: Option<String> = None;
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut expiration_date: Option<Option<i64>> = None;
                let mut name: Option<String> = None;
                let mut rule_query: Option<String> = None;
                let mut suppression_query: Option<String> = None;
                let mut version: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_exclusion_query" => {
                            if v.is_null() {
                                continue;
                            }
                            data_exclusion_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration_date" => {
                            expiration_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_query" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suppression_query" => {
                            if v.is_null() {
                                continue;
                            }
                            suppression_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringSuppressionUpdateAttributes {
                    data_exclusion_query,
                    description,
                    enabled,
                    expiration_date,
                    name,
                    rule_query,
                    suppression_query,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSuppressionUpdateAttributesVisitor)
    }
}
