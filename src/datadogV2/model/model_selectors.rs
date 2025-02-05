// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Selectors are used to filter security issues for which notifications should be generated.
/// Users can specify rule severities, rule types, a query to filter security issues on tags and attributes, and the trigger source.
/// Only the trigger_source field is required.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Selectors {
    /// The query is composed of one or several key:value pairs, which can be used to filter security issues on tags and attributes.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Security rule types used as filters in security rules.
    #[serde(rename = "rule_types")]
    pub rule_types: Option<Vec<crate::datadogV2::model::RuleTypesItems>>,
    /// The security rules severities to consider.
    #[serde(rename = "severities")]
    pub severities: Option<Vec<crate::datadogV2::model::RuleSeverity>>,
    /// The type of security issues on which the rule applies. Notification rules based on security signals need to use the trigger source "security_signals",
    /// while notification rules based on security vulnerabilities need to use the trigger source "security_findings".
    #[serde(rename = "trigger_source")]
    pub trigger_source: crate::datadogV2::model::TriggerSource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Selectors {
    pub fn new(trigger_source: crate::datadogV2::model::TriggerSource) -> Selectors {
        Selectors {
            query: None,
            rule_types: None,
            severities: None,
            trigger_source,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn rule_types(mut self, value: Vec<crate::datadogV2::model::RuleTypesItems>) -> Self {
        self.rule_types = Some(value);
        self
    }

    pub fn severities(mut self, value: Vec<crate::datadogV2::model::RuleSeverity>) -> Self {
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

impl<'de> Deserialize<'de> for Selectors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SelectorsVisitor;
        impl<'a> Visitor<'a> for SelectorsVisitor {
            type Value = Selectors;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<String> = None;
                let mut rule_types: Option<Vec<crate::datadogV2::model::RuleTypesItems>> = None;
                let mut severities: Option<Vec<crate::datadogV2::model::RuleSeverity>> = None;
                let mut trigger_source: Option<crate::datadogV2::model::TriggerSource> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_types" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_types = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severities" => {
                            if v.is_null() {
                                continue;
                            }
                            severities = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trigger_source" => {
                            trigger_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _trigger_source) = trigger_source {
                                match _trigger_source {
                                    crate::datadogV2::model::TriggerSource::UnparsedObject(
                                        _trigger_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let trigger_source =
                    trigger_source.ok_or_else(|| M::Error::missing_field("trigger_source"))?;

                let content = Selectors {
                    query,
                    rule_types,
                    severities,
                    trigger_source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SelectorsVisitor)
    }
}
