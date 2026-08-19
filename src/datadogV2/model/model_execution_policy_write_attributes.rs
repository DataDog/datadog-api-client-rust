// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes used to create or update an execution policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionPolicyWriteAttributes {
    /// The set of actions this policy applies to.
    #[serde(rename = "action_pattern")]
    pub action_pattern: crate::datadogV2::model::ExecutionPolicyActionPattern,
    /// Whether the policy allows or denies matching actions.
    #[serde(rename = "effect")]
    pub effect: crate::datadogV2::model::ExecutionPolicyEffect,
    /// The name of the execution policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Restricts where the policy applies. At most one of `kubernetes`, `scripts`,
    /// or `remote_action_rshell` can be set. An empty object means the policy has
    /// no scope restriction.
    #[serde(rename = "scope")]
    pub scope: Option<crate::datadogV2::model::ExecutionPolicyScope>,
    /// The targets this policy applies to.
    #[serde(rename = "targets")]
    pub targets: Option<Vec<crate::datadogV2::model::ExecutionPolicyTarget>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExecutionPolicyWriteAttributes {
    pub fn new(
        action_pattern: crate::datadogV2::model::ExecutionPolicyActionPattern,
        effect: crate::datadogV2::model::ExecutionPolicyEffect,
        name: String,
    ) -> ExecutionPolicyWriteAttributes {
        ExecutionPolicyWriteAttributes {
            action_pattern,
            effect,
            name,
            scope: None,
            targets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn scope(mut self, value: crate::datadogV2::model::ExecutionPolicyScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<crate::datadogV2::model::ExecutionPolicyTarget>) -> Self {
        self.targets = Some(value);
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

impl<'de> Deserialize<'de> for ExecutionPolicyWriteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExecutionPolicyWriteAttributesVisitor;
        impl<'a> Visitor<'a> for ExecutionPolicyWriteAttributesVisitor {
            type Value = ExecutionPolicyWriteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_pattern: Option<
                    crate::datadogV2::model::ExecutionPolicyActionPattern,
                > = None;
                let mut effect: Option<crate::datadogV2::model::ExecutionPolicyEffect> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<crate::datadogV2::model::ExecutionPolicyScope> = None;
                let mut targets: Option<Vec<crate::datadogV2::model::ExecutionPolicyTarget>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_pattern" => {
                            action_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "effect" => {
                            effect = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _effect) = effect {
                                match _effect {
                                    crate::datadogV2::model::ExecutionPolicyEffect::UnparsedObject(_effect) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            if v.is_null() {
                                continue;
                            }
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action_pattern =
                    action_pattern.ok_or_else(|| M::Error::missing_field("action_pattern"))?;
                let effect = effect.ok_or_else(|| M::Error::missing_field("effect"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ExecutionPolicyWriteAttributes {
                    action_pattern,
                    effect,
                    name,
                    scope,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExecutionPolicyWriteAttributesVisitor)
    }
}
