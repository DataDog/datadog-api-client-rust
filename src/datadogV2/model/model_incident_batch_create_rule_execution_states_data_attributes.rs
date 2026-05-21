// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for batch creating rule execution states.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentBatchCreateRuleExecutionStatesDataAttributes {
    /// List of rules to create execution states for.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::datadogV2::model::IncidentRuleExecutionStateRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentBatchCreateRuleExecutionStatesDataAttributes {
    pub fn new(
        rules: Vec<crate::datadogV2::model::IncidentRuleExecutionStateRule>,
    ) -> IncidentBatchCreateRuleExecutionStatesDataAttributes {
        IncidentBatchCreateRuleExecutionStatesDataAttributes {
            rules,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentBatchCreateRuleExecutionStatesDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentBatchCreateRuleExecutionStatesDataAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentBatchCreateRuleExecutionStatesDataAttributesVisitor {
            type Value = IncidentBatchCreateRuleExecutionStatesDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rules: Option<
                    Vec<crate::datadogV2::model::IncidentRuleExecutionStateRule>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rules = rules.ok_or_else(|| M::Error::missing_field("rules"))?;

                let content = IncidentBatchCreateRuleExecutionStatesDataAttributes {
                    rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentBatchCreateRuleExecutionStatesDataAttributesVisitor)
    }
}
