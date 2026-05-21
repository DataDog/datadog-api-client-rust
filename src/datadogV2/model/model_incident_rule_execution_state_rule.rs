// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A rule in a batch request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRuleExecutionStateRule {
    /// Timestamp of the last rule execution.
    #[serde(
        rename = "last_executed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_executed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The rule identifier.
    #[serde(rename = "rule_uuid")]
    pub rule_uuid: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRuleExecutionStateRule {
    pub fn new(rule_uuid: uuid::Uuid) -> IncidentRuleExecutionStateRule {
        IncidentRuleExecutionStateRule {
            last_executed_at: None,
            rule_uuid,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_executed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_executed_at = Some(value);
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

impl<'de> Deserialize<'de> for IncidentRuleExecutionStateRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRuleExecutionStateRuleVisitor;
        impl<'a> Visitor<'a> for IncidentRuleExecutionStateRuleVisitor {
            type Value = IncidentRuleExecutionStateRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_executed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut rule_uuid: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_executed_at" => {
                            last_executed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_uuid" => {
                            rule_uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rule_uuid = rule_uuid.ok_or_else(|| M::Error::missing_field("rule_uuid"))?;

                let content = IncidentRuleExecutionStateRule {
                    last_executed_at,
                    rule_uuid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRuleExecutionStateRuleVisitor)
    }
}
