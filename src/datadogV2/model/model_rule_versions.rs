// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule version with a list of updates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleVersions {
    /// A list of changes.
    #[serde(rename = "changes")]
    pub changes: Option<Vec<crate::datadogV2::model::RuleVersionUpdate>>,
    /// Create a new rule.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::SecurityMonitoringRuleResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleVersions {
    pub fn new() -> RuleVersions {
        RuleVersions {
            changes: None,
            rule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn changes(mut self, value: Vec<crate::datadogV2::model::RuleVersionUpdate>) -> Self {
        self.changes = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::SecurityMonitoringRuleResponse) -> Self {
        self.rule = Some(value);
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

impl Default for RuleVersions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RuleVersions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleVersionsVisitor;
        impl<'a> Visitor<'a> for RuleVersionsVisitor {
            type Value = RuleVersions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut changes: Option<Vec<crate::datadogV2::model::RuleVersionUpdate>> = None;
                let mut rule: Option<crate::datadogV2::model::SecurityMonitoringRuleResponse> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "changes" => {
                            if v.is_null() {
                                continue;
                            }
                            changes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule) = rule {
                                match _rule {
                                    crate::datadogV2::model::SecurityMonitoringRuleResponse::UnparsedObject(_rule) => {
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

                let content = RuleVersions {
                    changes,
                    rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleVersionsVisitor)
    }
}
