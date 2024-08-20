// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Create a new signal correlation rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalRuleCreatePayload {
    /// Cases for generating signals.
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
    /// Additional queries to filter matched events before they are processed. This field is deprecated for log detection, signal correlation, and workload security rules.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Whether the notifications include the triggering group-by values in their title.
    #[serde(rename = "hasExtendedTitle")]
    pub has_extended_title: Option<bool>,
    /// Whether the rule is enabled.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    /// Message for generated signals.
    #[serde(rename = "message")]
    pub message: String,
    /// The name of the rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Options on rules.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::SecurityMonitoringRuleOptions,
    /// Queries for selecting signals which are part of the rule.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::SecurityMonitoringSignalRuleQuery>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalRuleType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalRuleCreatePayload {
    pub fn new(
        cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
        is_enabled: bool,
        message: String,
        name: String,
        options: crate::datadogV2::model::SecurityMonitoringRuleOptions,
        queries: Vec<crate::datadogV2::model::SecurityMonitoringSignalRuleQuery>,
    ) -> SecurityMonitoringSignalRuleCreatePayload {
        SecurityMonitoringSignalRuleCreatePayload {
            cases,
            filters: None,
            has_extended_title: None,
            is_enabled,
            message,
            name,
            options,
            queries,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filters(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringFilter>,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn has_extended_title(mut self, value: bool) -> Self {
        self.has_extended_title = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalRuleType,
    ) -> Self {
        self.type_ = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringSignalRuleCreatePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalRuleCreatePayloadVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalRuleCreatePayloadVisitor {
            type Value = SecurityMonitoringSignalRuleCreatePayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
                > = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut has_extended_title: Option<bool> = None;
                let mut is_enabled: Option<bool> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::SecurityMonitoringRuleOptions> =
                    None;
                let mut queries: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringSignalRuleQuery>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringSignalRuleType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cases" => {
                            cases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasExtendedTitle" => {
                            if v.is_null() {
                                continue;
                            }
                            has_extended_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "isEnabled" => {
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringSignalRuleType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
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
                let cases = cases.ok_or_else(|| M::Error::missing_field("cases"))?;
                let is_enabled = is_enabled.ok_or_else(|| M::Error::missing_field("is_enabled"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;

                let content = SecurityMonitoringSignalRuleCreatePayload {
                    cases,
                    filters,
                    has_extended_title,
                    is_enabled,
                    message,
                    name,
                    options,
                    queries,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalRuleCreatePayloadVisitor)
    }
}
