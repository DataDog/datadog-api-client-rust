// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An execution policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExecutionPolicyAttributes {
    /// The set of actions this policy applies to.
    #[serde(rename = "action_pattern")]
    pub action_pattern: crate::datadogV2::model::ExecutionPolicyActionPattern,
    /// The date and time the execution policy was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The ID of the user who created the execution policy.
    #[serde(rename = "created_by")]
    pub created_by: String,
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
    pub targets: Vec<crate::datadogV2::model::ExecutionPolicyTarget>,
    /// The date and time the execution policy was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The ID of the user who last updated the execution policy.
    #[serde(rename = "updated_by")]
    pub updated_by: String,
    /// The version of the execution policy. Incremented on every update.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExecutionPolicyAttributes {
    pub fn new(
        action_pattern: crate::datadogV2::model::ExecutionPolicyActionPattern,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        effect: crate::datadogV2::model::ExecutionPolicyEffect,
        name: String,
        targets: Vec<crate::datadogV2::model::ExecutionPolicyTarget>,
        updated_at: chrono::DateTime<chrono::Utc>,
        updated_by: String,
        version: i32,
    ) -> ExecutionPolicyAttributes {
        ExecutionPolicyAttributes {
            action_pattern,
            created_at,
            created_by,
            effect,
            name,
            scope: None,
            targets,
            updated_at,
            updated_by,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn scope(mut self, value: crate::datadogV2::model::ExecutionPolicyScope) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for ExecutionPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExecutionPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for ExecutionPolicyAttributesVisitor {
            type Value = ExecutionPolicyAttributes;

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
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut effect: Option<crate::datadogV2::model::ExecutionPolicyEffect> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<crate::datadogV2::model::ExecutionPolicyScope> = None;
                let mut targets: Option<Vec<crate::datadogV2::model::ExecutionPolicyTarget>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<String> = None;
                let mut version: Option<i32> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let effect = effect.ok_or_else(|| M::Error::missing_field("effect"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = ExecutionPolicyAttributes {
                    action_pattern,
                    created_at,
                    created_by,
                    effect,
                    name,
                    scope,
                    targets,
                    updated_at,
                    updated_by,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExecutionPolicyAttributesVisitor)
    }
}
