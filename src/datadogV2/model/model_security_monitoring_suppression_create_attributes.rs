// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the attributes of the suppression rule to be created.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSuppressionCreateAttributes {
    /// An exclusion query on the input data of the security rules, which could be logs, Agent events, or other types of data based on the security rule. Events matching this query are ignored by any detection rules referenced in the suppression rule.
    #[serde(rename = "data_exclusion_query")]
    pub data_exclusion_query: Option<String>,
    /// A description for the suppression rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the suppression rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// A Unix millisecond timestamp giving an expiration date for the suppression rule. After this date, it won't suppress signals anymore.
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<i64>,
    /// The name of the suppression rule.
    #[serde(rename = "name")]
    pub name: String,
    /// The rule query of the suppression rule, with the same syntax as the search bar for detection rules.
    #[serde(rename = "rule_query")]
    pub rule_query: String,
    /// The suppression query of the suppression rule. If a signal matches this query, it is suppressed and is not triggered. It uses the same syntax as the queries to search signals in the Signals Explorer.
    #[serde(rename = "suppression_query")]
    pub suppression_query: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSuppressionCreateAttributes {
    pub fn new(
        enabled: bool,
        name: String,
        rule_query: String,
    ) -> SecurityMonitoringSuppressionCreateAttributes {
        SecurityMonitoringSuppressionCreateAttributes {
            data_exclusion_query: None,
            description: None,
            enabled,
            expiration_date: None,
            name,
            rule_query,
            suppression_query: None,
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

    pub fn expiration_date(mut self, value: i64) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn suppression_query(mut self, value: String) -> Self {
        self.suppression_query = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSuppressionCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSuppressionCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSuppressionCreateAttributesVisitor {
            type Value = SecurityMonitoringSuppressionCreateAttributes;

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
                let mut expiration_date: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut rule_query: Option<String> = None;
                let mut suppression_query: Option<String> = None;
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
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration_date" => {
                            if v.is_null() {
                                continue;
                            }
                            expiration_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_query" => {
                            rule_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suppression_query" => {
                            if v.is_null() {
                                continue;
                            }
                            suppression_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rule_query = rule_query.ok_or_else(|| M::Error::missing_field("rule_query"))?;

                let content = SecurityMonitoringSuppressionCreateAttributes {
                    data_exclusion_query,
                    description,
                    enabled,
                    expiration_date,
                    name,
                    rule_query,
                    suppression_query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSuppressionCreateAttributesVisitor)
    }
}
