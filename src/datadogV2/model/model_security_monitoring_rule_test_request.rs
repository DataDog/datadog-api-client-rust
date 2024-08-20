// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Test the rule queries of a rule (rule property is ignored when applied to an existing rule)
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleTestRequest {
    /// Test a rule.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::SecurityMonitoringRuleTestPayload>,
    /// Data payloads used to test rules query with the expected result.
    #[serde(rename = "ruleQueryPayloads")]
    pub rule_query_payloads:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringRuleQueryPayload>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleTestRequest {
    pub fn new() -> SecurityMonitoringRuleTestRequest {
        SecurityMonitoringRuleTestRequest {
            rule: None,
            rule_query_payloads: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn rule(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleTestPayload,
    ) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn rule_query_payloads(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringRuleQueryPayload>,
    ) -> Self {
        self.rule_query_payloads = Some(value);
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

impl Default for SecurityMonitoringRuleTestRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleTestRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleTestRequestVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleTestRequestVisitor {
            type Value = SecurityMonitoringRuleTestRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rule: Option<crate::datadogV2::model::SecurityMonitoringRuleTestPayload> =
                    None;
                let mut rule_query_payloads: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleQueryPayload>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule) = rule {
                                match _rule {
                                    crate::datadogV2::model::SecurityMonitoringRuleTestPayload::UnparsedObject(_rule) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "ruleQueryPayloads" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_query_payloads =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleTestRequest {
                    rule,
                    rule_query_payloads,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleTestRequestVisitor)
    }
}
