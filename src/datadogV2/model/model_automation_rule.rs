// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of an automation pipeline rule scope.
/// A rule can act on specific issue types, security rule types, security rule IDs, rule severities, or a query.
/// The query can be used to filter resources on tags and attributes.
/// The issue type and rule types fields are required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutomationRule {
    /// The type of issues on which the rule applies
    #[serde(rename = "issue_type")]
    pub issue_type: crate::datadogV2::model::IssueType,
    /// The query is composed of one or several key:value pairs, which can be used to filter resources on tags and attributes.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Security rule ids
    #[serde(rename = "rule_ids")]
    pub rule_ids: Option<Vec<String>>,
    /// Security rule types
    #[serde(rename = "rule_types")]
    pub rule_types: Vec<crate::datadogV2::model::SecurityRuleTypesItems>,
    /// The security rules severities to consider
    #[serde(rename = "severities")]
    pub severities: Option<Vec<crate::datadogV2::model::SecurityRuleSeverity>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutomationRule {
    pub fn new(
        issue_type: crate::datadogV2::model::IssueType,
        rule_types: Vec<crate::datadogV2::model::SecurityRuleTypesItems>,
    ) -> AutomationRule {
        AutomationRule {
            issue_type,
            query: None,
            rule_ids: None,
            rule_types,
            severities: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn rule_ids(mut self, value: Vec<String>) -> Self {
        self.rule_ids = Some(value);
        self
    }

    pub fn severities(mut self, value: Vec<crate::datadogV2::model::SecurityRuleSeverity>) -> Self {
        self.severities = Some(value);
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

impl<'de> Deserialize<'de> for AutomationRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutomationRuleVisitor;
        impl<'a> Visitor<'a> for AutomationRuleVisitor {
            type Value = AutomationRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut issue_type: Option<crate::datadogV2::model::IssueType> = None;
                let mut query: Option<String> = None;
                let mut rule_ids: Option<Vec<String>> = None;
                let mut rule_types: Option<Vec<crate::datadogV2::model::SecurityRuleTypesItems>> =
                    None;
                let mut severities: Option<Vec<crate::datadogV2::model::SecurityRuleSeverity>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "issue_type" => {
                            issue_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _issue_type) = issue_type {
                                match _issue_type {
                                    crate::datadogV2::model::IssueType::UnparsedObject(
                                        _issue_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_types" => {
                            rule_types = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severities" => {
                            if v.is_null() {
                                continue;
                            }
                            severities = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let issue_type = issue_type.ok_or_else(|| M::Error::missing_field("issue_type"))?;
                let rule_types = rule_types.ok_or_else(|| M::Error::missing_field("rule_types"))?;

                let content = AutomationRule {
                    issue_type,
                    query,
                    rule_ids,
                    rule_types,
                    severities,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutomationRuleVisitor)
    }
}
